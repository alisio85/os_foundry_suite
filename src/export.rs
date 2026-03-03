use crate::execution::{CommandSpec, ExecutionPlan};
use crate::image::ImagePlan;
use crate::report::SuiteReport;
use crate::services::ServiceGraph;

/// Export helpers.
///
/// This module provides owned-string export structures intended for serialization.
///
/// The suite keeps its core types minimal and stable; these export types are designed to be:
///
/// - straightforward to serialize
/// - insensitive to lifetime choices
/// - stable for CI tooling
///
/// An owned export representation of [`CommandSpec`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommandExport {
    /// Executable name.
    pub program: String,

    /// Arguments.
    pub args: Vec<String>,
}

impl From<&CommandSpec> for CommandExport {
    fn from(value: &CommandSpec) -> Self {
        Self {
            program: value.program.to_string(),
            args: value.args.iter().map(|a| (*a).to_string()).collect(),
        }
    }
}

/// An owned export representation of [`ExecutionPlan`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExecutionPlanExport {
    /// Target architecture as string.
    pub arch: String,

    /// Whether the target is bare metal.
    pub bare_metal: bool,

    /// Whether the target is UEFI.
    pub uefi: bool,

    /// Build command.
    pub build: CommandExport,

    /// Optional run command.
    pub run: Option<CommandExport>,

    /// Optional packaging command.
    pub package: Option<CommandExport>,
}

impl From<&ExecutionPlan> for ExecutionPlanExport {
    fn from(value: &ExecutionPlan) -> Self {
        let arch = match value.target.arch {
            crate::targets::Arch::X86_64 => "x86_64",
            crate::targets::Arch::Aarch64 => "aarch64",
        };

        Self {
            arch: arch.to_string(),
            bare_metal: value.target.bare_metal,
            uefi: value.target.uefi,
            build: CommandExport::from(&value.build),
            run: value.run.as_ref().map(CommandExport::from),
            package: value.package.as_ref().map(CommandExport::from),
        }
    }
}

/// A minimal export representation for image planning.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImagePlanExport {
    /// Image kind.
    pub kind: String,

    /// Partition scheme.
    pub scheme: String,

    /// Boot artifact.
    pub boot: String,
}

impl From<&ImagePlan> for ImagePlanExport {
    fn from(value: &ImagePlan) -> Self {
        let kind = match value.kind {
            crate::config::ImageKind::None => "none",
            crate::config::ImageKind::Raw => "raw",
            crate::config::ImageKind::UefiGpt => "uefi-gpt",
        };

        let scheme = match value.layout.scheme {
            crate::image::PartitionScheme::None => "none",
            crate::image::PartitionScheme::Gpt => "gpt",
        };

        let boot = match value.layout.boot {
            crate::image::BootArtifact::Kernel => "kernel",
            crate::image::BootArtifact::UefiApp => "uefi-app",
        };

        Self {
            kind: kind.to_string(),
            scheme: scheme.to_string(),
            boot: boot.to_string(),
        }
    }
}

/// A minimal export representation for service graphs.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ServiceGraphExport {
    /// Node IDs.
    pub nodes: Vec<String>,
}

impl From<&ServiceGraph> for ServiceGraphExport {
    fn from(value: &ServiceGraph) -> Self {
        Self {
            nodes: value.nodes().map(|n| n.id.0.to_string()).collect(),
        }
    }
}

/// A serde-friendly export representation of the suite report.
///
/// This type intentionally contains only export-friendly sub-structures.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SuiteReportExport {
    /// Whether validation had errors.
    pub valid: bool,

    /// Validation errors.
    pub errors: Vec<String>,

    /// Validation warnings.
    pub warnings: Vec<String>,

    /// Execution plan export.
    pub execution: ExecutionPlanExport,

    /// Optional image export.
    pub image: Option<ImagePlanExport>,

    /// Services export.
    pub services: ServiceGraphExport,

    /// Artifact paths.
    pub kernel_binary: Option<String>,

    /// Image artifact path.
    pub image_artifact: Option<String>,

    /// Boot log path.
    pub boot_log: Option<String>,
}

impl From<&SuiteReport> for SuiteReportExport {
    fn from(value: &SuiteReport) -> Self {
        Self {
            valid: value.validation.is_ok(),
            errors: value
                .validation
                .errors
                .iter()
                .map(|s| (*s).to_string())
                .collect(),
            warnings: value
                .validation
                .warnings
                .iter()
                .map(|s| (*s).to_string())
                .collect(),
            execution: ExecutionPlanExport::from(&value.execution_plan),
            image: value.image_plan.as_ref().map(ImagePlanExport::from),
            services: ServiceGraphExport::from(&value.services),
            kernel_binary: value.artifacts.kernel_binary.clone(),
            image_artifact: value.artifacts.image_artifact.clone(),
            boot_log: value.artifacts.boot_log.clone(),
        }
    }
}
