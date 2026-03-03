use crate::builder::BuildPlan;
use crate::config::{ImageKind, OsBlueprint};
use crate::error::{Error, Result};
use crate::targets::{Arch, Target};

/// A deterministic, side-effect free description of how a build could be executed.
///
/// The suite intentionally separates:
///
/// - Planning: `OsBuilder` => `BuildPlan`.
/// - Execution intent: `ExecutionPlanner` => `ExecutionPlan`.
///
/// The execution plan is designed for host tooling and CI systems. It does not perform any
/// IO on its own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionPlan {
    /// Original blueprint.
    pub blueprint: OsBlueprint,

    /// Target derived from planning.
    pub target: Target,

    /// Suggested Rust toolchain/channel.
    pub toolchain: ToolchainSpec,

    /// Suggested build command.
    pub build: CommandSpec,

    /// Optional run command (e.g. QEMU).
    pub run: Option<CommandSpec>,

    /// Optional packaging command.
    pub package: Option<CommandSpec>,
}

/// A normalized command specification.
///
/// This is intentionally simple so it can be:
///
/// - printed to logs
/// - serialized by downstream tools
/// - executed by external runners
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSpec {
    /// Executable name.
    pub program: &'static str,

    /// Arguments.
    pub args: Vec<&'static str>,
}

impl CommandSpec {
    /// Creates a new command spec.
    #[must_use]
    pub fn new(program: &'static str) -> Self {
        Self {
            program,
            args: Vec::new(),
        }
    }

    /// Adds an argument.
    #[must_use]
    pub fn arg(mut self, arg: &'static str) -> Self {
        self.args.push(arg);
        self
    }
}

/// Toolchain selection intent.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ToolchainChannel {
    /// Stable channel.
    Stable,
    /// Nightly channel.
    Nightly,
}

/// Suggested toolchain spec for host tooling.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ToolchainSpec {
    /// Which channel to use.
    pub channel: ToolchainChannel,
}

impl ToolchainSpec {
    /// Stable toolchain.
    #[must_use]
    pub const fn stable() -> Self {
        Self {
            channel: ToolchainChannel::Stable,
        }
    }

    /// Nightly toolchain.
    #[must_use]
    pub const fn nightly() -> Self {
        Self {
            channel: ToolchainChannel::Nightly,
        }
    }
}

/// QEMU run configuration intent.
///
/// This config is intentionally minimal and portable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QemuRunSpec {
    /// QEMU binary to use.
    pub program: &'static str,

    /// Suggested arguments.
    pub args: Vec<&'static str>,
}

impl QemuRunSpec {
    /// Creates a conservative default spec for the given target.
    #[must_use]
    pub fn for_target(target: Target) -> Option<Self> {
        let (program, args): (&'static str, Vec<&'static str>) = match target.arch {
            Arch::X86_64 => ("qemu-system-x86_64", vec!["-serial", "stdio"]),
            Arch::Aarch64 => ("qemu-system-aarch64", vec!["-serial", "stdio"]),
        };

        Some(Self { program, args })
    }

    /// Converts the spec into a [`CommandSpec`].
    #[must_use]
    pub fn to_command(&self) -> CommandSpec {
        CommandSpec {
            program: self.program,
            args: self.args.clone(),
        }
    }
}

/// A pure planner that converts a [`BuildPlan`] into an [`ExecutionPlan`].
#[derive(Debug, Copy, Clone)]
pub struct ExecutionPlanner;

impl ExecutionPlanner {
    /// Builds an execution plan.
    ///
    /// This method is deterministic and side-effect free.
    pub fn plan(build_plan: BuildPlan) -> Result<ExecutionPlan> {
        build_plan.blueprint.validate()?;

        let toolchain = ToolchainSpec::stable();

        let build = CommandSpec::new("cargo").arg("build").arg("--release");

        let run = QemuRunSpec::for_target(build_plan.target).map(|q| q.to_command());

        let package = match build_plan.blueprint.image.kind {
            ImageKind::None => None,
            ImageKind::Raw => Some(CommandSpec::new("cargo").arg("run").arg("-p").arg("image")),
            ImageKind::UefiGpt => Some(
                CommandSpec::new("cargo")
                    .arg("run")
                    .arg("-p")
                    .arg("uefi-image"),
            ),
        };

        if !build_plan.target.bare_metal {
            return Err(Error::unsupported_target(
                "execution planner only supports bare-metal targets in the current suite version",
            ));
        }

        Ok(ExecutionPlan {
            blueprint: build_plan.blueprint,
            target: build_plan.target,
            toolchain,
            build,
            run,
            package,
        })
    }
}

/// Convenience helper to plan execution directly from a blueprint and target.
///
/// This is equivalent to:
///
/// - `OsBuilder::new(blueprint).target(target).validate_and_plan()?`
/// - `ExecutionPlanner::plan(build_plan)`
pub fn plan_execution(blueprint: OsBlueprint, target: Target) -> Result<ExecutionPlan> {
    let build_plan = crate::builder::OsBuilder::new(blueprint)
        .target(target)
        .validate_and_plan()?;

    ExecutionPlanner::plan(build_plan)
}
