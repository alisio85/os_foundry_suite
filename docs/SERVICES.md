# Services

This document describes the suite-level services model.

## 1. Purpose

OS projects typically contain multiple services (or subsystems) that must be started,
stopped, supervised, and observed.

`os_foundry_suite` models this as a deterministic dependency graph (`ServiceGraph`) so that:

- the dependency structure is explicit
- validation can detect missing dependencies
- validation can detect dependency cycles

## 2. Key types

- `ServiceId`
  - a stable string identifier

- `ServiceNode`
  - `id: ServiceId`
  - `depends_on: BTreeSet<ServiceId>`

- `ServiceGraph`
  - deterministic storage with `BTreeMap`
  - `validate()` checks invariants

## 3. Mapping from `ServiceSet`

`ServiceGraph::from_service_set(&ServiceSet)` builds a minimal graph.

Downstream projects can:

- add nodes
- add edges
- validate

## 4. Integration with `os_service_fabric`

When the `service-fabric` feature is enabled, the suite can expose stable integration
points for downstream tooling.
