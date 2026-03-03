# Execution

This document describes the suite-level execution planning layer.

## 1. Purpose

`os_foundry_suite` intentionally separates:

- **Build planning** (`OsBuilder` -> `BuildPlan`)
- **Execution planning** (`ExecutionPlanner` -> `ExecutionPlan`)

The execution layer is **side-effect free**. It does not run commands. Instead it produces
portable `CommandSpec` values that external tooling can execute.

## 2. Key types

- `ExecutionPlan`
  - `toolchain: ToolchainSpec`
  - `build: CommandSpec`
  - `run: Option<CommandSpec>`
  - `package: Option<CommandSpec>`

- `CommandSpec`
  - `program: &'static str`
  - `args: Vec<&'static str>`

## 3. QEMU intent

`QemuRunSpec` provides a minimal target-derived run configuration.

Downstream tooling can extend it (e.g. disk images, firmware, networking).

## 4. Reproducibility

The suite recommends:

- keeping plans deterministic
- keeping arguments stable across CI/local runs
- embedding build identifiers into artifacts (via underlying crates or downstream tooling)
