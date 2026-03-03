# Security model

This document describes the security assumptions and trust boundaries of `os_foundry_suite`.

## 1. Scope

This crate is an orchestration + facade layer. It does not directly execute privileged code or perform device I/O by default.

## 2. Trust boundaries

- **Host tooling**: configuration construction, validation, plan derivation.
- **Kernel/runtime**: implemented by downstream crates and binaries.

The suite should be safe to depend on from host tools.

## 3. Unsafe code policy

- The suite itself aims to keep `unsafe` usage to a minimum.
- When `unsafe` is needed, it must be documented with explicit invariants.

## 4. Supply chain

- Dependencies are pinned using semver versions in `Cargo.toml`.
- CI enforces `clippy -D warnings` and `rustdoc -D warnings`.

## 5. ABI and compatibility

If strict ABI is enabled (see `AbiPolicy`), downstream projects should enforce ABI layout contracts.
