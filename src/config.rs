use crate::error::{Error, Result};

/// A suite-level kernel configuration.
///
/// This configuration describes *intent* about the kernel build and runtime posture.
/// The suite does not execute builds by default, but it uses this data to validate
/// and derive a consistent build plan.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct KernelConfig {
    /// Whether this configuration is intended for development iteration.
    pub dev_mode: bool,
}

impl KernelConfig {
    /// Minimal kernel configuration.
    #[must_use]
    pub const fn minimal() -> Self {
        Self { dev_mode: false }
    }

    /// Development-oriented kernel configuration.
    #[must_use]
    pub const fn dev() -> Self {
        Self { dev_mode: true }
    }
}

/// A suite-level memory configuration.
///
/// This describes high-level assumptions about the environment.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MemoryConfig {
    /// Whether paging/virtual memory is expected.
    pub paging: bool,

    /// Whether the environment is expected to provide a UEFI memory map.
    pub uefi_memory_map: bool,
}

impl MemoryConfig {
    /// Conservative bare-metal defaults.
    #[must_use]
    pub const fn default_bare_metal() -> Self {
        Self {
            paging: true,
            uefi_memory_map: false,
        }
    }

    /// Conservative UEFI defaults.
    #[must_use]
    pub const fn default_uefi() -> Self {
        Self {
            paging: true,
            uefi_memory_map: true,
        }
    }
}

/// ABI policy used by the suite.
///
/// When strict ABI is enabled, the suite encourages downstream projects to enforce
/// layout and calling convention contracts using `os_abi_sentinel`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AbiPolicy {
    /// Whether ABI constraints should be treated as strict and non-negotiable.
    pub strict: bool,
}

impl AbiPolicy {
    /// Strict ABI policy.
    #[must_use]
    pub const fn strict() -> Self {
        Self { strict: true }
    }

    /// Relaxed ABI policy.
    #[must_use]
    pub const fn relaxed() -> Self {
        Self { strict: false }
    }
}

/// Observability configuration.
///
/// This config is suite-level intent. The actual implementation is delegated to
/// `os_observatory` when the `observatory` feature is enabled.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ObservabilityConfig {
    /// Enable structured events/tracing.
    pub enabled: bool,
}

impl ObservabilityConfig {
    /// Minimal observability config.
    #[must_use]
    pub const fn minimal() -> Self {
        Self { enabled: false }
    }

    /// Development-oriented observability config.
    #[must_use]
    pub const fn dev() -> Self {
        Self { enabled: true }
    }
}

/// Supported image kinds.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ImageKind {
    /// No image packaging is requested.
    None,
    /// A raw disk/kernel image.
    Raw,
    /// A UEFI + GPT-based image layout.
    UefiGpt,
}

/// Suite-level image packaging configuration.
///
/// This describes how an OS artifact should be packaged for execution or testing.
/// The suite does not implement the full pipeline yet; it provides the intent and
/// validation.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ImageConfig {
    /// Selected image kind.
    pub kind: ImageKind,
}

impl ImageConfig {
    /// No image packaging.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            kind: ImageKind::None,
        }
    }

    /// Raw image packaging.
    #[must_use]
    pub const fn raw() -> Self {
        Self {
            kind: ImageKind::Raw,
        }
    }

    /// UEFI + GPT image packaging.
    #[must_use]
    pub const fn uefi_gpt() -> Self {
        Self {
            kind: ImageKind::UefiGpt,
        }
    }
}

/// A deterministic validation report.
///
/// The suite distinguishes between:
///
/// - Errors: violations that must be fixed.
/// - Warnings: suspicious configurations that may be valid but deserve attention.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    /// Errors found during validation.
    pub errors: Vec<&'static str>,

    /// Warnings found during validation.
    pub warnings: Vec<&'static str>,
}

impl ValidationReport {
    /// Creates an empty report.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Returns `true` if validation produced no errors.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
}

/// A suite-level service selection.
///
/// This type describes *intent* (which categories of services to include) without forcing a
/// specific service runtime implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceSet {
    /// Include a baseline service manager/registry.
    pub base: bool,

    /// Include observability hooks.
    pub observability: bool,
}

impl ServiceSet {
    /// A minimal service set.
    #[must_use]
    pub const fn minimal() -> Self {
        Self {
            base: true,
            observability: false,
        }
    }

    /// A developer-friendly set, enabling observability.
    #[must_use]
    pub const fn dev() -> Self {
        Self {
            base: true,
            observability: true,
        }
    }
}

/// A strongly-typed configuration describing an operating system build.
///
/// The blueprint is the suite-level unit of intent: it captures what you want to produce
/// (kernel composition, service selection, ABI policy, and image preferences).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OsBlueprint {
    /// Human-readable name for the OS.
    pub name: &'static str,

    /// Kernel intent.
    pub kernel: KernelConfig,

    /// Memory model intent.
    pub memory: MemoryConfig,

    /// ABI intent.
    pub abi: AbiPolicy,

    /// Selected services.
    pub services: ServiceSet,

    /// Observability intent.
    pub observability: ObservabilityConfig,

    /// Image packaging intent.
    pub image: ImageConfig,
}

impl OsBlueprint {
    /// Creates a minimal development blueprint.
    #[must_use]
    pub const fn minimal_dev() -> Self {
        Self {
            name: "unnamed-os",
            kernel: KernelConfig::dev(),
            memory: MemoryConfig::default_bare_metal(),
            abi: AbiPolicy::strict(),
            services: ServiceSet::dev(),
            observability: ObservabilityConfig::dev(),
            image: ImageConfig::raw(),
        }
    }

    /// Performs validation and returns a detailed report.
    ///
    /// This method never fails: it returns a report that callers can inspect.
    ///
    /// The suite keeps the rules intentionally deterministic so they can be used in CI.
    #[must_use]
    pub fn validate_report(&self) -> ValidationReport {
        let mut report = ValidationReport::empty();

        if self.name.is_empty() {
            report.errors.push("name must not be empty");
        }

        if !self.services.base {
            report
                .errors
                .push("services.base must be enabled in the current suite version");
        }

        if self.image.kind == ImageKind::UefiGpt && !self.memory.uefi_memory_map {
            report.warnings.push(
                "image.kind is UefiGpt but memory.uefi_memory_map is false (did you mean MemoryConfig::default_uefi()?)",
            );
        }

        if self.observability.enabled && !self.services.observability {
            report.warnings.push(
                "observability.enabled is true but services.observability is false; consider enabling the service capability",
            );
        }

        report
    }

    /// Validates the blueprint.
    ///
    /// Validation is deterministic and only checks suite-level invariants.
    pub fn validate(&self) -> Result<()> {
        let report = self.validate_report();
        if let Some(message) = report.errors.first().copied() {
            return Err(Error::invalid_blueprint(message));
        }
        Ok(())
    }
}
