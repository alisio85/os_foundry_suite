# Artifacts and workspace layout

This document describes the suite-level modeling of workspaces and build artifacts.

## 1. Purpose

OS projects typically involve multiple directories and multiple artifacts (kernel binaries, disk images, logs).

`os_foundry_suite` provides:

- `WorkspaceLayout`: a conventional directory model
- `BuildMetadata`: reproducible build identifiers
- `ArtifactCatalog`: a deterministic container of artifact locations

All of these are *side-effect free*. They do not touch the filesystem.

## 2. WorkspaceLayout

`WorkspaceLayout` describes conventional directories:

- `kernel_dir`
- `services_dir`
- `tools_dir`
- `out_dir`

Use `WorkspaceLayout::conventional()` to get default values.

## 3. BuildMetadata

`BuildMetadata` is intentionally conservative:

- `build_id` is required
- `vcs_revision` is optional

The suite avoids time-based fields by default to preserve reproducibility.

## 4. ArtifactCatalog

`ArtifactCatalog` stores optional artifact paths, such as:

- `kernel_binary`
- `image_artifact`
- `boot_log`

Downstream tooling can populate these fields and then serialize them for CI debugging.

## 5. Serialization

When the `serde` feature is enabled, these types derive `Serialize` and `Deserialize`.
