#![doc = include_str!("../docs/CRATE_OVERVIEW.md")]

//!
//! # os_foundry_suite
//!
//! This crate is a *suite* facade that unifies multiple OS-building crates into a single,
//! coherent, well-documented entry point.
//!
//! It is designed to:
//!
//! - Provide a stable, user-friendly *orchestration* API (blueprints, builders, targets).
//! - Re-export the underlying crates behind feature flags.
//! - Offer conventions and integration helpers so downstream projects can compose kernel,
//!   services, ABI contracts, state machines, observability, and images with less friction.
//!
//! ## Feature flags
//!
//! The underlying crates are optional dependencies. Enable only what you need:
//!
//! - `kernel-foundry` => `os_kernel_foundry`
//! - `dev-toolkit` => `os_dev_toolkit`
//! - `metal-primitives` => `os_metal_primitives`
//! - `service-fabric` => `os_service_fabric`
//! - `linker-sculptor` => `os_linker_sculptor`
//! - `slab-vault` => `os_slab_vault`
//! - `abi-sentinel` => `os_abi_sentinel`
//! - `state-maestro` => `os_state_maestro`
//! - `observatory` => `os_observatory`
//! - `image-lens` => `os_image_lens`
//!
//! Additional features:
//!
//! - `serde` enables serialization support for select suite-level types.
//!
//! ## Quick start
//!
//! ```
//! use os_foundry_suite::prelude::*;
//!
//! let blueprint = OsBlueprint::minimal_dev();
//! blueprint.validate().unwrap();
//!
//! let plan = OsBuilder::new(blueprint)
//!     .target(Target::x86_64_bare_metal())
//!     .validate_and_plan()
//!     .unwrap();
//!
//! assert_eq!(plan.target.arch, Arch::X86_64);
//! ```
//!
//! ## Scope
//!
//! This crate intentionally focuses on *composition and orchestration*. It does not attempt
//! to hide all complexity involved in OS development; instead it provides a consistent
//! structure and strongly-typed configuration to reduce accidental complexity.

pub mod artifacts;
pub mod builder;
pub mod config;
pub mod error;
pub mod execution;
pub mod export;
pub mod image;
pub mod integration;
pub mod pipeline;
pub mod prelude;
pub mod profiles;
pub mod report;
pub mod services;
pub mod targets;

pub use artifacts::{ArtifactCatalog, BuildMetadata, WorkspaceLayout};
pub use builder::{BuildPlan, OsBuilder};
pub use config::{
    AbiPolicy, ImageConfig, ImageKind, KernelConfig, MemoryConfig, ObservabilityConfig,
    OsBlueprint, ServiceSet, ValidationReport,
};
pub use error::{Error, Result};
pub use execution::{
    CommandSpec, ExecutionPlan, ExecutionPlanner, QemuRunSpec, ToolchainChannel, ToolchainSpec,
    plan_execution,
};
pub use export::{
    CommandExport, ExecutionPlanExport, ImagePlanExport, ServiceGraphExport, SuiteReportExport,
};
pub use image::{
    BootArtifact, ImageLayoutIntent, ImagePlan, PartitionScheme, PartitionSpec, plan_image,
};
pub use integration::validate_blueprint_integrations;
pub use pipeline::derive_artifacts;
pub use profiles::Profiles;
pub use report::SuiteReport;
pub use services::{ServiceGraph, ServiceId, ServiceNode};
pub use targets::{Arch, Target};

/// Re-exports of the underlying crates, gated behind feature flags.
///
/// The naming is intentionally short and stable, so downstream code can write:
/// `os_foundry_suite::crates::kernel::...`.
pub mod crates {
    /// Re-export of `os_kernel_foundry`.
    #[cfg(feature = "kernel-foundry")]
    pub use os_kernel_foundry as kernel;

    /// Re-export of `os_dev_toolkit`.
    #[cfg(feature = "dev-toolkit")]
    pub use os_dev_toolkit as dev;

    /// Re-export of `os_metal_primitives`.
    #[cfg(feature = "metal-primitives")]
    pub use os_metal_primitives as metal;

    /// Re-export of `os_service_fabric`.
    #[cfg(feature = "service-fabric")]
    pub use os_service_fabric as services;

    /// Re-export of `os_linker_sculptor`.
    #[cfg(feature = "linker-sculptor")]
    pub use os_linker_sculptor as linker;

    /// Re-export of `os_slab_vault`.
    #[cfg(feature = "slab-vault")]
    pub use os_slab_vault as slab;

    /// Re-export of `os_abi_sentinel`.
    #[cfg(feature = "abi-sentinel")]
    pub use os_abi_sentinel as abi;

    /// Re-export of `os_state_maestro`.
    #[cfg(feature = "state-maestro")]
    pub use os_state_maestro as state;

    /// Re-export of `os_observatory`.
    #[cfg(feature = "observatory")]
    pub use os_observatory as observatory;

    /// Re-export of `os_image_lens`.
    #[cfg(feature = "image-lens")]
    pub use os_image_lens as image;
}
