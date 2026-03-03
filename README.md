# os_foundry_suite

A Rust 2024 *suite crate* that unifies multiple OS-building crates into a single, cohesive entry point.

## What this is

`os_foundry_suite` is a facade + orchestration crate. It provides:

- A stable public API: `OsBlueprint`, `OsBuilder`, `Target`, and `prelude`.
- Suite presets: `Profiles`.
- A deterministic validation report: `OsBlueprint::validate_report()`.
- Integration validation: `validate_blueprint_integrations()`.
- Execution planning: `ExecutionPlanner` / `ExecutionPlan`.
- Services modeling: `ServiceGraph`.
- Feature-gated re-exports of the underlying crates under `os_foundry_suite::crates::*`.
- Suite-level conventions: typed configuration, deterministic validation, and integration points.

## Included crates (feature-gated)

Enable only what you need:

- `kernel-foundry` => `os_kernel_foundry`
- `dev-toolkit` => `os_dev_toolkit`
- `metal-primitives` => `os_metal_primitives`
- `service-fabric` => `os_service_fabric`
- `linker-sculptor` => `os_linker_sculptor`
- `slab-vault` => `os_slab_vault`
- `abi-sentinel` => `os_abi_sentinel`
- `state-maestro` => `os_state_maestro`
- `observatory` => `os_observatory`
- `image-lens` => `os_image_lens`

## Quick start

```rust
use os_foundry_suite::prelude::*;

let blueprint = OsBlueprint::minimal_dev();
blueprint.validate()?;

let plan = OsBuilder::new(blueprint)
    .target(Target::x86_64_bare_metal())
    .validate_and_plan()?;

assert_eq!(plan.target.arch, Arch::X86_64);
# Ok::<(), os_foundry_suite::Error>(())
```

## Documentation

- Index: `docs/INDEX.md`
- Manual: `docs/MANUAL.md`
- Architecture: `docs/ARCHITECTURE.md`
- Blueprint reference: `docs/BLUEPRINT_REFERENCE.md`
- Targets: `docs/TARGETS.md`
- Observability guide: `docs/OBSERVABILITY_GUIDE.md`
- Execution: `docs/EXECUTION.md`
- Services: `docs/SERVICES.md`
- Integration: `docs/INTEGRATION.md`
- Image layout reference: `docs/IMAGE_LAYOUT_REFERENCE.md`
- End-to-end guide: `docs/END_TO_END_GUIDE.md`
- Image guide: `docs/IMAGE_GUIDE.md`
- Service graph reference: `docs/SERVICE_GRAPH_REFERENCE.md`
- Artifacts and workspace layout: `docs/ARTIFACTS.md`
- Pipeline: `docs/PIPELINE.md`
- Reporting: `docs/REPORTING.md`

## Attribution

This project was produced by an artificial intelligence system (AI) based on an original idea by **alisio85**.

## License

MIT (see `LICENSE`).
