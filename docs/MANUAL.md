# Manual

## 1. Purpose

`os_foundry_suite` is a Rust 2024 suite crate that provides a single entry point for composing an operating system project using multiple specialized crates.

The suite focuses on:

- Strongly-typed configuration (`OsBlueprint`).
- Deterministic validation and planning (`OsBuilder` => `BuildPlan`).
- Feature-gated access to the underlying crates.

## 2. Mental model

The suite follows a simple pipeline:

1. You create an `OsBlueprint` describing intent.
2. You validate it (`OsBlueprint::validate`).
3. You feed it into `OsBuilder`.
4. The builder derives a `BuildPlan`.
5. Your project (or an external tool/CLI) executes the plan.

In addition, the suite provides:

- `Profiles`: predefined blueprint presets.
- `OsBlueprint::validate_report()`: a structured validation report.
- `validate_blueprint_integrations()`: suite + integration validation in one call.

This crate is intentionally conservative: it does not perform IO or execute builds by default.

## 3. Feature-gated dependencies

The suite can be used without enabling any underlying crate.

When you enable a feature flag (e.g. `kernel-foundry`), the corresponding crate is made available under `os_foundry_suite::crates::*`.

## 4. Suggested project layout

A recommended downstream workspace layout:

- `kernel/` (your kernel crate)
- `services/` (service crates)
- `tools/` (host tools / build runners)
- `images/` (build outputs)

`os_foundry_suite` is intended to be a dependency of the host-side orchestration crate and/or shared config crate.

## 5. Validation philosophy

Suite validation checks:

- Configuration sanity (e.g. names, required toggles).
- Target support invariants.

Lower-level invariants (memory safety, ABI layouts, kernel invariants) remain the responsibility of the specialized crates.

## 6. Blueprint sections

The suite blueprint is intentionally split into sections:

- Kernel intent (`KernelConfig`)
- Memory intent (`MemoryConfig`)
- ABI intent (`AbiPolicy`)
- Services intent (`ServiceSet`)
- Observability intent (`ObservabilityConfig`)
- Image packaging intent (`ImageConfig`)

These sections allow downstream projects to keep configuration cohesive while delegating actual implementations to your specialized crates.

## 7. Next steps

- See `ARCHITECTURE.md` for module design.
- See `BLUEPRINT_REFERENCE.md` for configuration details.
- See `TARGETS.md` for target guidance.
- See `EXECUTION.md` for execution planning.
- See `SERVICES.md` for service modeling.
- See `INTEGRATION.md` for cross-crate integration.
- See `END_TO_END_GUIDE.md` for a complete host-side flow.
- See `IMAGE_GUIDE.md` and `IMAGE_LAYOUT_REFERENCE.md` for image planning.
- See `SERVICE_GRAPH_REFERENCE.md` for the service graph model.
