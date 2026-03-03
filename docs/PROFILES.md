# Profiles

`Profiles` provides suite-maintained presets that create an `OsBlueprint` with strong defaults.

## Why profiles exist

OS projects typically start by combining multiple decisions:

- target environment
- service selection
- observability posture
- ABI strictness
- image packaging intent

Profiles exist to reduce the amount of boilerplate required to reach a consistent starting point.

## Available profiles

- `Profiles::minimal()`
  - A minimal, conservative blueprint.

- `Profiles::dev_qemu()`
  - Development-oriented intent suitable for QEMU-based iteration.

- `Profiles::uefi_dev()`
  - Development intent that selects UEFI + GPT image packaging and UEFI memory assumptions.

## Customization

Profiles are plain Rust functions returning a value.

The intended usage is:

- call a profile
- override specific fields
- validate

Example:

```rust
use os_foundry_suite::prelude::*;

let mut bp = Profiles::dev_qemu();
bp.name = "my-os";

let report = bp.validate_report();
assert!(report.is_ok());
```
