use crate::config::{
    AbiPolicy, ImageConfig, KernelConfig, MemoryConfig, ObservabilityConfig, OsBlueprint,
    ServiceSet,
};

/// Suite-provided presets for quickly starting new OS projects.
///
/// Profiles are intentionally conservative and *host-runnable*: they only construct
/// configuration values (they do not perform IO).
///
/// The goal is to offer strong defaults that are easy to override.
pub struct Profiles;

impl Profiles {
    /// A minimal blueprint intended for small experiments.
    #[must_use]
    pub const fn minimal() -> OsBlueprint {
        OsBlueprint {
            name: "unnamed-os",
            kernel: KernelConfig::minimal(),
            memory: MemoryConfig::default_bare_metal(),
            abi: AbiPolicy::strict(),
            services: ServiceSet::minimal(),
            observability: ObservabilityConfig::minimal(),
            image: ImageConfig::none(),
        }
    }

    /// A development blueprint intended for QEMU-based iteration.
    #[must_use]
    pub const fn dev_qemu() -> OsBlueprint {
        OsBlueprint {
            name: "unnamed-os",
            kernel: KernelConfig::dev(),
            memory: MemoryConfig::default_bare_metal(),
            abi: AbiPolicy::strict(),
            services: ServiceSet::dev(),
            observability: ObservabilityConfig::dev(),
            image: ImageConfig::raw(),
        }
    }

    /// A development blueprint targeting UEFI workflows.
    #[must_use]
    pub const fn uefi_dev() -> OsBlueprint {
        OsBlueprint {
            name: "unnamed-os",
            kernel: KernelConfig::dev(),
            memory: MemoryConfig::default_uefi(),
            abi: AbiPolicy::strict(),
            services: ServiceSet::dev(),
            observability: ObservabilityConfig::dev(),
            image: ImageConfig::uefi_gpt(),
        }
    }
}
