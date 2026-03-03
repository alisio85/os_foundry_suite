# Image pipeline

This document explains the suite-level image packaging intent.

## 1. Intent vs implementation

`os_foundry_suite` models image packaging as intent via `ImageConfig` and `ImageKind`.
It then derives a deterministic `ImagePlan` via `plan_image(&OsBlueprint, Target)`.

- The suite validates that the intent is internally consistent.
- Actual packaging is delegated to downstream tooling and/or the specialized crates.

For the image layout model, see `IMAGE_LAYOUT_REFERENCE.md`.

## 2. Image kinds

- `ImageKind::None`
  - No packaging requested.

- `ImageKind::Raw`
  - A raw image artifact is desired.

- `ImageKind::UefiGpt`
  - UEFI + GPT packaging is desired.
  - Typically requires UEFI memory map assumptions (see `MemoryConfig`).

## 3. Recommendations

- Prefer deterministic build identifiers embedded into images.
- Ensure serial output is enabled for early boot debugging.
