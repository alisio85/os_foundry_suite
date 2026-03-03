use os_foundry_suite::prelude::*;

#[test]
fn conventional_layout_is_stable() {
    let layout = WorkspaceLayout::conventional();
    assert_eq!(layout.kernel_dir, "kernel");
    assert_eq!(layout.out_dir, "out");
}

#[test]
fn artifact_catalog_can_be_constructed() {
    let meta = BuildMetadata::new("build-001");
    let layout = WorkspaceLayout::conventional();

    let catalog = ArtifactCatalog::new(meta, layout);
    assert!(catalog.kernel_binary.is_none());
}

#[cfg(feature = "serde")]
#[test]
fn artifact_catalog_roundtrips_with_serde_json() {
    let meta = BuildMetadata::new("build-xyz");
    let layout = WorkspaceLayout::conventional();

    let mut catalog = ArtifactCatalog::new(meta, layout);
    catalog.kernel_binary = Some("out/kernel.elf".to_string());

    let json = serde_json::to_string(&catalog).unwrap();
    let back: ArtifactCatalog = serde_json::from_str(&json).unwrap();

    assert_eq!(back.kernel_binary.as_deref(), Some("out/kernel.elf"));
}
