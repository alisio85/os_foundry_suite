# End-to-end guide

This guide shows a complete *host-side* workflow using `os_foundry_suite`.

## 1. Goal

- Start from a suite profile.
- Validate the blueprint.
- Validate cross-module integrations.
- Derive a build plan.
- Derive an execution plan (command specs).
- Derive an image plan.
- Derive a service graph.

All steps are deterministic and side-effect free.

## 2. Example

```rust
use os_foundry_suite::prelude::*;

fn main() -> Result<()> {
    // 1) Start from a preset.
    let mut blueprint = Profiles::uefi_dev();
    blueprint.name = "my-os";

    // 2) Blueprint validation report.
    let report = blueprint.validate_report();
    if !report.is_ok() {
        return Err(Error::invalid_blueprint(
            report.errors.first().copied().unwrap_or("unknown error"),
        ));
    }

    // 3) Integration validation (ABI + observability conventions).
    let report = validate_blueprint_integrations(&blueprint);
    if !report.is_ok() {
        return Err(Error::invalid_blueprint(
            report.errors.first().copied().unwrap_or("unknown error"),
        ));
    }

    // 4) Build planning.
    let build_plan = OsBuilder::new(blueprint.clone())
        .target(Target::x86_64_uefi())
        .validate_and_plan()?;

    // 5) Execution planning.
    let exec_plan = ExecutionPlanner::plan(build_plan.clone())?;

    // 6) Image planning.
    let image_plan = plan_image(&blueprint, build_plan.target)?;

    // 7) Service graph.
    let service_graph = ServiceGraph::from_service_set(&blueprint.services);
    service_graph.validate()?;

    // This crate does not execute commands; downstream tooling can.
    let _ = (exec_plan, image_plan, service_graph);

    Ok(())
}
```

## 3. Next steps

- Use the `ExecutionPlan` to implement a host-side runner.
- Map `ServiceGraph` into your service runtime of choice.
- Convert `ImagePlan` into packaging actions using dedicated tooling.
