use crate::config::{ImageConfig, ImageKind, MemoryConfig, OsBlueprint};
use crate::error::{Error, Result};
use crate::targets::Target;

/// A high-level boot artifact selection.
///
/// This enum is a suite-level intent signal describing what the boot chain should load.
/// The suite itself does not implement loading.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BootArtifact {
    /// The kernel binary is loaded directly.
    Kernel,

    /// A UEFI application (EFI stub or bootloader) is loaded.
    UefiApp,
}

/// Partition scheme selection.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PartitionScheme {
    /// No partitioning (single raw artifact).
    None,

    /// GPT partition scheme.
    Gpt,
}

/// A suite-level partition descriptor.
///
/// This is intentionally minimal and stable.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PartitionSpec {
    /// A stable label for the partition.
    pub label: &'static str,

    /// Suggested size in MiB.
    pub size_mib: u32,
}

/// Image layout intent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageLayoutIntent {
    /// Partition scheme.
    pub scheme: PartitionScheme,

    /// Partitions.
    pub partitions: Vec<PartitionSpec>,

    /// Selected boot artifact intent.
    pub boot: BootArtifact,
}

impl ImageLayoutIntent {
    /// No layout.
    #[must_use]
    pub fn none() -> Self {
        Self {
            scheme: PartitionScheme::None,
            partitions: Vec::new(),
            boot: BootArtifact::Kernel,
        }
    }

    /// A conservative UEFI + GPT layout intent.
    #[must_use]
    pub fn uefi_gpt_default() -> Self {
        Self {
            scheme: PartitionScheme::Gpt,
            partitions: vec![
                PartitionSpec {
                    label: "ESP",
                    size_mib: 128,
                },
                PartitionSpec {
                    label: "OS",
                    size_mib: 512,
                },
            ],
            boot: BootArtifact::UefiApp,
        }
    }
}

/// A deterministic plan describing how an image should be packaged.
///
/// The plan is derived from blueprint intent and target selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImagePlan {
    /// Kind of image.
    pub kind: ImageKind,

    /// Layout intent.
    pub layout: ImageLayoutIntent,
}

/// Builds an [`ImagePlan`] from suite-level configuration.
///
/// This function is deterministic and side-effect free.
pub fn plan_image(blueprint: &OsBlueprint, target: Target) -> Result<Option<ImagePlan>> {
    validate_image_intent(&blueprint.image, blueprint.memory, target)?;

    match blueprint.image.kind {
        ImageKind::None => Ok(None),
        ImageKind::Raw => Ok(Some(ImagePlan {
            kind: ImageKind::Raw,
            layout: ImageLayoutIntent::none(),
        })),
        ImageKind::UefiGpt => Ok(Some(ImagePlan {
            kind: ImageKind::UefiGpt,
            layout: ImageLayoutIntent::uefi_gpt_default(),
        })),
    }
}

fn validate_image_intent(image: &ImageConfig, memory: MemoryConfig, target: Target) -> Result<()> {
    if image.kind == ImageKind::UefiGpt {
        if !target.uefi {
            return Err(Error::invalid_blueprint(
                "image.kind is UefiGpt but target.uefi is false",
            ));
        }

        if !memory.uefi_memory_map {
            return Err(Error::invalid_blueprint(
                "image.kind is UefiGpt but memory.uefi_memory_map is false",
            ));
        }
    }

    Ok(())
}
