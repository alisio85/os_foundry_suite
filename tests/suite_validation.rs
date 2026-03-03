use os_foundry_suite::prelude::*;
use os_foundry_suite::validate_blueprint_integrations;

#[test]
fn minimal_profile_is_valid() {
    let bp = Profiles::minimal();
    let report = bp.validate_report();
    assert!(report.is_ok(), "expected no errors, got: {:?}", report);
}

#[test]
fn dev_qemu_profile_has_observability_intent() {
    let bp = Profiles::dev_qemu();
    assert!(bp.observability.enabled);
}

#[test]
fn uefi_profile_emits_warning_if_memory_is_not_uefi() {
    let mut bp = Profiles::uefi_dev();
    bp.memory = MemoryConfig::default_bare_metal();

    let report = bp.validate_report();
    assert!(report.is_ok());
    assert!(
        report.warnings.iter().any(|w| w.contains("UefiGpt")),
        "expected UEFI/GPT warning, got: {:?}",
        report
    );
}

#[test]
fn validate_and_plan_defaults_target_and_is_bare_metal() {
    let bp = OsBlueprint::minimal_dev();
    let plan = OsBuilder::new(bp).validate_and_plan().unwrap();
    assert!(plan.target.bare_metal);
}

#[test]
fn integration_validation_is_deterministic_and_ok_for_profiles() {
    let bp = Profiles::dev_qemu();
    let report = validate_blueprint_integrations(&bp);
    assert!(report.is_ok(), "expected no errors, got: {:?}", report);
}

#[test]
fn integration_validation_adds_abi_warning_when_strict() {
    let bp = Profiles::minimal();
    let report = validate_blueprint_integrations(&bp);
    assert!(report.is_ok());
    assert!(
        report
            .warnings
            .iter()
            .any(|w| w.contains("ABI policy is strict")),
        "expected ABI warning, got: {:?}",
        report
    );
}
