use crate::artifacts::{ArtifactCatalog, BuildMetadata, WorkspaceLayout};
use crate::execution::ExecutionPlan;
use crate::image::ImagePlan;

/// Artifact pipeline planning.
///
/// This module provides deterministic helpers that derive conventional artifact paths from
/// a combination of:
///
/// - workspace layout
/// - build metadata
/// - execution plan
/// - image plan
///
/// The suite does not perform any IO; it only generates stable path strings.
///
/// Derives an [`ArtifactCatalog`] using conventional path rules.
///
/// The goal of this function is to provide a stable baseline that downstream tooling can
/// adopt for:
///
/// - CI artifact uploads
/// - log collection
/// - reproducible output locations
#[must_use]
pub fn derive_artifacts(
    meta: BuildMetadata,
    layout: WorkspaceLayout,
    exec: &ExecutionPlan,
    image: Option<&ImagePlan>,
) -> ArtifactCatalog {
    let mut catalog = ArtifactCatalog::new(meta, layout);

    // Kernel binary location is a convention. Downstream tooling can override it.
    //
    // We intentionally include target information to avoid collisions.
    let arch = match exec.target.arch {
        crate::targets::Arch::X86_64 => "x86_64",
        crate::targets::Arch::Aarch64 => "aarch64",
    };

    catalog.kernel_binary = Some(format!("{}/kernel-{}.elf", catalog.layout.out_dir, arch));

    if let Some(image) = image {
        let ext = match image.kind {
            crate::config::ImageKind::None => "none",
            crate::config::ImageKind::Raw => "img",
            crate::config::ImageKind::UefiGpt => "img",
        };

        catalog.image_artifact = Some(format!(
            "{}/os-image-{}.{}",
            catalog.layout.out_dir, arch, ext
        ));
    }

    catalog.boot_log = Some(format!("{}/boot-{}.log", catalog.layout.out_dir, arch));

    catalog
}
