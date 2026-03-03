use crate::config::OsBlueprint;
use crate::error::{Error, Result};
use crate::targets::Target;

/// A resolved plan produced by [`OsBuilder`].
///
/// A build plan is the suite's "intermediate representation": it records derived decisions
/// (like the effective target) after validating the blueprint and builder inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildPlan {
    /// The blueprint that produced this plan.
    pub blueprint: OsBlueprint,

    /// The resolved target.
    pub target: Target,
}

/// High-level orchestrator for converting an [`OsBlueprint`] into a [`BuildPlan`].
#[derive(Debug, Clone)]
pub struct OsBuilder {
    blueprint: OsBlueprint,
    target: Option<Target>,
}

impl OsBuilder {
    /// Creates a new builder.
    #[must_use]
    pub const fn new(blueprint: OsBlueprint) -> Self {
        Self {
            blueprint,
            target: None,
        }
    }

    /// Sets the target.
    #[must_use]
    pub const fn target(mut self, target: Target) -> Self {
        self.target = Some(target);
        self
    }

    /// Produces a [`BuildPlan`] without performing any IO.
    ///
    /// This method is intentionally side-effect free so it can be used in unit tests and
    /// as a validation step in CI.
    pub fn build_plan(self) -> BuildPlan {
        let target = self.target.unwrap_or_else(Target::x86_64_bare_metal);
        BuildPlan {
            blueprint: self.blueprint,
            target,
        }
    }

    /// Validates the configuration and returns a build plan.
    pub fn validate_and_plan(self) -> Result<BuildPlan> {
        self.blueprint.validate()?;

        let plan = self.build_plan();

        if !plan.target.bare_metal {
            return Err(Error::unsupported_target(
                "non-bare-metal environments are not supported in the current suite version",
            ));
        }

        Ok(plan)
    }
}
