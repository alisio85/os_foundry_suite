# Integration

`os_foundry_suite` is a suite crate, so a major part of its value is providing stable integration points.

## Principles

- The suite should remain usable even if no underlying crate features are enabled.
- Integration helpers are deterministic and side-effect free.
- Feature-gated integrations expose deeper helpers when the corresponding crate is enabled.

## Suite-level integration validation

`validate_blueprint_integrations(&OsBlueprint)` performs:

- base blueprint validation (`OsBlueprint::validate_report()`)
- ABI policy checks (`integration::abi`)
- observability checks (`integration::observability`)

The output is always a `ValidationReport`.

## Underlying crate integration

When features are enabled, the suite exposes those crates under `os_foundry_suite::crates::*` and may add conventions.

Examples:

- `observatory` feature exposes `integration::observability::conventions::*`.
- `abi-sentinel` feature exposes `integration::abi::sentinel::*`.
