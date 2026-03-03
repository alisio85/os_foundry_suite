# Image guide

This guide explains how to use suite-level image planning.

## 1. Intent

The suite models image packaging intent via:

- `OsBlueprint.image: ImageConfig`
- `ImageConfig.kind: ImageKind`

From this intent, you can derive a deterministic `ImagePlan`:

- `plan_image(&OsBlueprint, Target)`

## 2. Typical flows

### Raw image intent

- select `ImageConfig::raw()`
- use a bare-metal target

### UEFI + GPT intent

- select `ImageConfig::uefi_gpt()`
- use a UEFI target (e.g. `Target::x86_64_uefi()`)
- ensure `MemoryConfig::default_uefi()` is used

## 3. Validation

If you select `ImageKind::UefiGpt`, the suite enforces:

- `target.uefi == true`
- `memory.uefi_memory_map == true`

This ensures suite-level intent is coherent before downstream tooling starts packaging.
