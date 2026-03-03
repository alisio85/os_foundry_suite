//! Workspace and artifact modeling.
//!
//! This module provides deterministic, side-effect free structures that describe:
//!
//! - where source code typically lives
//! - where build artifacts are expected to be placed
//! - how tools can refer to those artifacts
//!
//! The suite intentionally does not perform IO here. Downstream tooling can take these
//! structures and implement filesystem operations.

/// A deterministic workspace layout.
///
/// This describes a conventional directory structure that downstream projects can adopt.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorkspaceLayout {
    /// Path to the kernel crate directory.
    pub kernel_dir: String,

    /// Path to the services directory.
    pub services_dir: String,

    /// Path to host tools directory.
    pub tools_dir: String,

    /// Path to build output directory.
    pub out_dir: String,
}

impl WorkspaceLayout {
    /// Returns a conventional workspace layout.
    #[must_use]
    pub fn conventional() -> Self {
        Self {
            kernel_dir: "kernel".to_string(),
            services_dir: "services".to_string(),
            tools_dir: "tools".to_string(),
            out_dir: "out".to_string(),
        }
    }
}

/// Deterministic build metadata.
///
/// This structure is intentionally conservative. It avoids time-based fields by default
/// to preserve reproducibility.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuildMetadata {
    /// A stable build identifier.
    ///
    /// Downstream tooling may set this to a git hash, semver tag, or CI build number.
    pub build_id: String,

    /// Optional VCS revision string (e.g. git commit).
    pub vcs_revision: Option<String>,
}

impl BuildMetadata {
    /// Creates minimal build metadata with a human-supplied build id.
    #[must_use]
    pub fn new(build_id: impl Into<String>) -> Self {
        Self {
            build_id: build_id.into(),
            vcs_revision: None,
        }
    }
}

/// A catalog of build artifacts.
///
/// Artifact paths are modeled as strings because:
///
/// - the suite aims to avoid filesystem dependencies
/// - downstream tooling may run on different hosts
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArtifactCatalog {
    /// Build metadata.
    pub meta: BuildMetadata,

    /// Conventional workspace layout.
    pub layout: WorkspaceLayout,

    /// Optional path to the kernel binary.
    pub kernel_binary: Option<String>,

    /// Optional path to the produced image artifact.
    pub image_artifact: Option<String>,

    /// Optional path to a serial log or boot transcript.
    pub boot_log: Option<String>,
}

impl ArtifactCatalog {
    /// Creates an empty catalog.
    #[must_use]
    pub fn new(meta: BuildMetadata, layout: WorkspaceLayout) -> Self {
        Self {
            meta,
            layout,
            kernel_binary: None,
            image_artifact: None,
            boot_log: None,
        }
    }
}
