# Blueprint reference

## `OsBlueprint`

Fields:

- `name: &'static str`
  - Human-readable OS name.
  - Must not be empty.

- `kernel: KernelConfig`
  - High-level kernel intent.

- `memory: MemoryConfig`
  - High-level memory model intent.

- `abi: AbiPolicy`
  - ABI strictness intent.

- `services: ServiceSet`
  - High-level service selection.

- `observability: ObservabilityConfig`
  - Observability intent.

- `image: ImageConfig`
  - Image packaging intent.

### Validation rules

- `name` must not be empty.
- `services.base` must be `true` (current suite invariant).

### Validation report

`OsBlueprint::validate_report()` returns a `ValidationReport` with:

- `errors`: hard failures.
- `warnings`: suspicious configurations.

`OsBlueprint::validate()` fails if any error exists in the report.

## `KernelConfig`

- `dev_mode: bool`
  - Indicates a development-oriented configuration.

## `MemoryConfig`

- `paging: bool`
  - Whether paging/virtual memory is expected.

- `uefi_memory_map: bool`
  - Whether UEFI memory map semantics are expected.

## `AbiPolicy`

- `strict: bool`
  - Whether ABI constraints should be treated as strict.

## `ObservabilityConfig`

- `enabled: bool`
  - Whether observability is enabled.

## `ImageConfig` / `ImageKind`

- `ImageKind::None`
  - No packaging.

- `ImageKind::Raw`
  - Raw artifact packaging.

- `ImageKind::UefiGpt`
  - UEFI + GPT packaging intent.

## `ServiceSet`

Fields:

- `base: bool`
  - Baseline service capability is included.

- `observability: bool`
  - Whether to include observability hooks.

Presets:

- `ServiceSet::minimal()`
- `ServiceSet::dev()`
