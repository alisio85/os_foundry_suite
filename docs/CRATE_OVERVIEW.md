# os_foundry_suite

`os_foundry_suite` is a Rust 2024 *suite crate* that unifies multiple OS-building crates into a single, cohesive entry point.

## What this crate provides

- A stable, well-documented facade (`prelude`, `OsBlueprint`, `OsBuilder`, `Target`).
- Suite presets: `Profiles`.
- Integration validation: `validate_blueprint_integrations()`.
- Optional, feature-gated re-exports of the underlying crates.
- Suite-level conventions: naming, configuration validation, and integration points.

## Underlying crates

The following crates can be enabled via feature flags and accessed through `os_foundry_suite::crates::*`.

- `os_kernel_foundry` (`kernel-foundry`)
- `os_dev_toolkit` (`dev-toolkit`)
- `os_metal_primitives` (`metal-primitives`)
- `os_service_fabric` (`service-fabric`)
- `os_linker_sculptor` (`linker-sculptor`)
- `os_slab_vault` (`slab-vault`)
- `os_abi_sentinel` (`abi-sentinel`)
- `os_state_maestro` (`state-maestro`)
- `os_observatory` (`observatory`)
- `os_image_lens` (`image-lens`)

## Attribution

This project was produced by an artificial intelligence system (AI) based on an original idea by **alisio85**.
