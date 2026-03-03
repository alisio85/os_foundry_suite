# FAQ

## Is this crate `no_std`?

The suite crate is currently designed for host-side orchestration and uses `std`.

## Does this crate build my OS automatically?

Not by default.

The suite focuses on:

- configuration
- validation
- planning

Actual compilation and image generation are intended to be implemented by downstream tools.

## How do I access the underlying crates?

Enable the corresponding feature flag and import it from `os_foundry_suite::crates::*`.

Example:

```rust
// Cargo.toml: os_foundry_suite = { version = "...", features = ["observatory"] }

use os_foundry_suite::crates::observatory;
```
