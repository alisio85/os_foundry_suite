# Targets

`Target` is a suite-level description of the intended architecture and environment.

## Supported targets

- `Target::x86_64_bare_metal()`
- `Target::x86_64_uefi()`
- `Target::aarch64_bare_metal()`

## Notes

- The suite currently enforces `bare_metal = true` when validating a plan.
- Firmware interfaces and boot flows should be implemented by downstream projects using the underlying crates.
