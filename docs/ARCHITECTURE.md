# Architecture

## 1. Design intent

`os_foundry_suite` is a facade and orchestration crate.

- It re-exports your existing crates behind feature flags.
- It provides a stable suite-level API for downstream projects.

## 2. Modules

- `config`:
  - Defines `OsBlueprint` and supporting configuration types.
  - Provides deterministic, side-effect free validation.

- `builder`:
  - Defines `OsBuilder` and `BuildPlan`.
  - Converts intent (blueprint + target) into a derived plan.

- `execution`:
  - Defines `ExecutionPlanner` and `ExecutionPlan`.
  - Converts a `BuildPlan` into deterministic command specifications.

- `services`:
  - Defines `ServiceGraph` and related types.
  - Provides deterministic service dependency validation.

- `targets`:
  - Defines `Arch` and `Target`.

- `error`:
  - Defines suite-level `Error` and `Result`.

- `prelude`:
  - Convenience imports.

## 3. Stability policy

- Suite-level types should evolve carefully with backwards compatibility.
- Underlying crates remain the authoritative implementation of their domain.

## 4. Integration boundary

The suite-level crate should be the *place* where cross-crate conventions live:

- consistent naming
- consistent feature flags
- stable configuration model
- stable observability conventions (event categories, milestone IDs)

Actual implementations remain in the underlying crates.
