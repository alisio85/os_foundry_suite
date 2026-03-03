use os_foundry_suite::SuiteReportExport;
use os_foundry_suite::prelude::*;

#[test]
fn derive_artifacts_is_deterministic() {
    let bp = Profiles::dev_qemu();
    let build_plan = OsBuilder::new(bp.clone())
        .target(Target::x86_64_bare_metal())
        .validate_and_plan()
        .unwrap();

    let exec = ExecutionPlanner::plan(build_plan).unwrap();
    let img = plan_image(&bp, exec.target).unwrap();

    let meta = BuildMetadata::new("build-1");
    let layout = WorkspaceLayout::conventional();

    let cat = derive_artifacts(meta, layout, &exec, img.as_ref());
    assert!(cat.kernel_binary.as_ref().unwrap().contains("x86_64"));
    assert!(cat.boot_log.as_ref().unwrap().contains("x86_64"));
}

#[test]
fn suite_report_can_be_built() {
    let bp = Profiles::dev_qemu();

    let validation = os_foundry_suite::validate_blueprint_integrations(&bp);
    let build_plan = OsBuilder::new(bp.clone())
        .target(Target::x86_64_bare_metal())
        .validate_and_plan()
        .unwrap();

    let exec = ExecutionPlanner::plan(build_plan.clone()).unwrap();
    let image_plan = plan_image(&bp, build_plan.target).unwrap();

    let services = ServiceGraph::from_service_set(&bp.services);
    services.validate().unwrap();

    let artifacts = derive_artifacts(
        BuildMetadata::new("build-1"),
        WorkspaceLayout::conventional(),
        &exec,
        image_plan.as_ref(),
    );

    let report = SuiteReport {
        validation,
        build_plan,
        execution_plan: exec,
        image_plan,
        services,
        artifacts,
    };

    assert!(report.validation.is_ok());
}

#[cfg(feature = "serde")]
#[test]
fn suite_report_serializes() {
    let bp = Profiles::dev_qemu();
    let validation = os_foundry_suite::validate_blueprint_integrations(&bp);
    let build_plan = OsBuilder::new(bp.clone())
        .target(Target::x86_64_bare_metal())
        .validate_and_plan()
        .unwrap();

    let exec = ExecutionPlanner::plan(build_plan.clone()).unwrap();
    let image_plan = plan_image(&bp, build_plan.target).unwrap();

    let services = ServiceGraph::from_service_set(&bp.services);
    let artifacts = derive_artifacts(
        BuildMetadata::new("build-1"),
        WorkspaceLayout::conventional(),
        &exec,
        image_plan.as_ref(),
    );

    let report = SuiteReport {
        validation,
        build_plan,
        execution_plan: exec,
        image_plan,
        services,
        artifacts,
    };

    let export = SuiteReportExport::from(&report);
    let json = serde_json::to_string(&export).unwrap();
    assert!(json.contains("\"valid\""));
    assert!(json.contains("\"execution\""));
    assert!(json.contains("\"services\""));
}
