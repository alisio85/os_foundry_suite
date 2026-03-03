# Service graph reference

This document describes the suite-level `ServiceGraph` model.

## 1. Why a graph?

A service set (like `ServiceSet`) captures high-level intent, but it does not explain:

- which service must start before another
- what the dependency ordering is
- whether dependencies are missing

`ServiceGraph` solves this by providing a deterministic dependency graph.

## 2. Validation invariants

`ServiceGraph::validate()` enforces:

- **No missing dependencies**
  - every dependency edge must reference an existing service ID

- **No cycles**
  - cycles are rejected because they make startup ordering ambiguous

## 3. Determinism

The graph uses `BTreeMap` and `BTreeSet` so iteration is stable.

This helps when:

- writing tests
- comparing plans across machines
- producing reproducible logs

## 4. Example

```rust
use os_foundry_suite::prelude::*;

let mut graph = ServiceGraph::new();

graph.insert(ServiceNode::new(ServiceId("base")));
graph.insert(ServiceNode::new(ServiceId("net")).depends_on(ServiceId("base")));

graph.validate().unwrap();
```
