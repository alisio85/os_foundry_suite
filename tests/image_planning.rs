use os_foundry_suite::prelude::*;

#[test]
fn plan_image_none_returns_none() {
    let mut bp = Profiles::minimal();
    bp.image = ImageConfig::none();

    let plan = plan_image(&bp, Target::x86_64_bare_metal()).unwrap();
    assert!(plan.is_none());
}

#[test]
fn plan_image_raw_returns_plan() {
    let mut bp = Profiles::minimal();
    bp.image = ImageConfig::raw();

    let plan = plan_image(&bp, Target::x86_64_bare_metal())
        .unwrap()
        .unwrap();
    assert_eq!(plan.kind, ImageKind::Raw);
    assert_eq!(plan.layout.scheme, PartitionScheme::None);
}

#[test]
fn plan_image_uefi_requires_target_uefi() {
    let bp = Profiles::uefi_dev();

    let err = plan_image(&bp, Target::x86_64_bare_metal()).unwrap_err();
    assert_eq!(
        err.to_string(),
        "invalid blueprint: image.kind is UefiGpt but target.uefi is false"
    );
}

#[test]
fn plan_image_uefi_requires_uefi_memory_map() {
    let mut bp = Profiles::uefi_dev();
    bp.memory = MemoryConfig::default_bare_metal();

    let err = plan_image(&bp, Target::x86_64_uefi()).unwrap_err();
    assert_eq!(
        err.to_string(),
        "invalid blueprint: image.kind is UefiGpt but memory.uefi_memory_map is false"
    );
}

#[test]
fn plan_image_uefi_happy_path() {
    let bp = Profiles::uefi_dev();

    let plan = plan_image(&bp, Target::x86_64_uefi()).unwrap().unwrap();
    assert_eq!(plan.kind, ImageKind::UefiGpt);
    assert_eq!(plan.layout.scheme, PartitionScheme::Gpt);
    assert_eq!(plan.layout.boot, BootArtifact::UefiApp);
}
