use crate::artifacts::ArtifactCatalog;
use crate::builder::BuildPlan;
use crate::config::ValidationReport;
use crate::execution::ExecutionPlan;
use crate::image::ImagePlan;
use crate::services::ServiceGraph;

/// A high-level suite report.
///
/// This structure is designed for host tooling and CI.
///
/// It bundles the main suite-level artifacts into a single value:
///
/// - validation report
/// - build plan
/// - execution plan
/// - optional image plan
/// - service graph
/// - artifact catalog
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuiteReport {
    /// Validation report.
    pub validation: ValidationReport,

    /// Build plan.
    pub build_plan: BuildPlan,

    /// Execution plan.
    pub execution_plan: ExecutionPlan,

    /// Image plan, if any.
    pub image_plan: Option<ImagePlan>,

    /// Service dependency graph.
    pub services: ServiceGraph,

    /// Artifact catalog.
    pub artifacts: ArtifactCatalog,
}
