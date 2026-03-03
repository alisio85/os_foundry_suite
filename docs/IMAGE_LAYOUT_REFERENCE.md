# Image layout reference

This document specifies the suite-level image layout model.

## 1. Purpose

The suite models image packaging as *intent* and derives a deterministic `ImagePlan`.

- The plan is **side-effect free**.
- It is designed to be executed by downstream host tooling.

## 2. Key types

- `ImagePlan`
  - `kind: ImageKind`
  - `layout: ImageLayoutIntent`

- `ImageLayoutIntent`
  - `scheme: PartitionScheme`
  - `partitions: Vec<PartitionSpec>`
  - `boot: BootArtifact`

## 3. Partition scheme

- `PartitionScheme::None`
  - single artifact layout (typical for `ImageKind::Raw`)

- `PartitionScheme::Gpt`
  - GPT-based layout intent (typical for `ImageKind::UefiGpt`)

## 4. Boot artifact

- `BootArtifact::Kernel`
  - the boot chain loads a kernel binary directly

- `BootArtifact::UefiApp`
  - the boot chain loads a UEFI application (EFI stub or bootloader)

## 5. Validation rules

The suite enforces the following invariants:

- If `image.kind == ImageKind::UefiGpt`:
  - `target.uefi` must be `true`
  - `memory.uefi_memory_map` must be `true`

These rules are enforced by `plan_image(&OsBlueprint, Target)`.

## 6. Extending the model

Downstream tooling may extend this model by:

- adding additional partition metadata
- mapping `PartitionSpec` into real GPT/ESP creation steps
- emitting reproducible build identifiers
