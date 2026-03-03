use crate::config::{AbiPolicy, ObservabilityConfig, OsBlueprint, ValidationReport};

/// Integration helpers that coordinate suite-level intent with underlying crates.
///
/// This module is intentionally designed around *optional* integrations:
///
/// - If a given underlying crate feature is disabled, the corresponding integration API is
///   either not available (feature-gated) or becomes a no-op at the suite level.
///
/// The suite's job here is to define *conventions* and provide stable glue points.
/// Suite-level ABI integration helpers.
pub mod abi {
    use super::*;

    /// Validates that the blueprint ABI policy is consistent with the suite's requirements.
    ///
    /// This function is pure and deterministic.
    #[must_use]
    pub fn validate_policy(policy: AbiPolicy) -> ValidationReport {
        let mut report = ValidationReport::empty();

        if policy.strict {
            // Suite-level note: strict ABI is encouraged, but not all projects will be
            // enforcing it at the same time.
            report.warnings.push(
                "ABI policy is strict; ensure you enforce layout contracts in your downstream crates",
            );
        }

        report
    }

    /// If the `abi-sentinel` feature is enabled, this module can expose higher-level helpers
    /// that delegate to `os_abi_sentinel`.
    #[cfg(feature = "abi-sentinel")]
    pub mod sentinel {
        /// Returns `true` if ABI sentinel integration is available.
        ///
        /// This is primarily useful for runtime/feature discovery in host tooling.
        #[must_use]
        pub const fn available() -> bool {
            true
        }
    }
}

/// Suite-level observability integration helpers.
pub mod observability {
    use super::*;

    /// Produces a deterministic validation report for the observability configuration.
    #[must_use]
    pub fn validate_config(cfg: ObservabilityConfig) -> ValidationReport {
        let mut report = ValidationReport::empty();

        if cfg.enabled {
            report.warnings.push(
                "observability is enabled; define stable boot milestones to make tests deterministic",
            );
        }

        report
    }

    /// If the `observatory` feature is enabled, provide a small, stable place for conventions.
    #[cfg(feature = "observatory")]
    pub mod conventions {
        /// A recommended event category for boot milestones.
        pub const BOOT_CATEGORY: &str = "boot";

        /// A recommended event category for service lifecycle.
        pub const SERVICE_CATEGORY: &str = "service";
    }
}

/// Validates the blueprint using suite-level and integration-level checks.
///
/// This function is deterministic and side-effect free.
#[must_use]
pub fn validate_blueprint_integrations(blueprint: &OsBlueprint) -> ValidationReport {
    let mut report = blueprint.validate_report();

    let abi = abi::validate_policy(blueprint.abi);
    report.errors.extend(abi.errors);
    report.warnings.extend(abi.warnings);

    let obs = observability::validate_config(blueprint.observability);
    report.errors.extend(obs.errors);
    report.warnings.extend(obs.warnings);

    report
}
