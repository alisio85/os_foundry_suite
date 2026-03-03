# Pipeline

This document describes the suite-level pipeline helpers.

## 1. Purpose

The suite provides deterministic helpers to derive conventional artifact paths.

This is useful for:

- CI artifact upload conventions
- reproducible output locations
- structured reporting

## 2. `derive_artifacts`

`derive_artifacts(meta, layout, exec, image)` produces an `ArtifactCatalog`.

Conventions:

- kernel binary path includes the target architecture
- image artifact path is derived from `ImagePlan.kind`
- boot log path includes the target architecture

Downstream tooling can override any of these paths.
