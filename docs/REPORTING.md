# Reporting

This document describes suite-level reporting.

## 1. Purpose

Host tooling and CI systems often need a single structured value to export:

- validation results
- build/execution/image plans
- service graph
- artifact catalog

The suite provides `SuiteReport` for this purpose.

## 2. `SuiteReport`

`SuiteReport` bundles:

- `validation: ValidationReport`
- `build_plan: BuildPlan`
- `execution_plan: ExecutionPlan`
- `image_plan: Option<ImagePlan>`
- `services: ServiceGraph`
- `artifacts: ArtifactCatalog`

`SuiteReport` is an in-memory, native structure.

For serialization and stable interchange formats, use `SuiteReportExport`.

When the `serde` feature is enabled, `SuiteReportExport` can be serialized and deserialized.

## 3. Export types

The suite also provides export helper types (owned strings) designed for stable JSON/TOML:

- `ExecutionPlanExport`
- `ImagePlanExport`
- `ServiceGraphExport`
- `CommandExport`
