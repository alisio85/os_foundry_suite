use crate::config::ServiceSet;
use crate::error::{Error, Result};
use std::collections::{BTreeMap, BTreeSet};

/// A stable identifier for a service.
///
/// The suite uses string IDs because they are:
///
/// - easy to log
/// - easy to compare across systems
/// - stable for configuration
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ServiceId(pub &'static str);

/// A single service node in the service graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceNode {
    /// Service identifier.
    pub id: ServiceId,

    /// Service dependencies.
    pub depends_on: BTreeSet<ServiceId>,
}

impl ServiceNode {
    /// Creates a new node.
    #[must_use]
    pub fn new(id: ServiceId) -> Self {
        Self {
            id,
            depends_on: BTreeSet::new(),
        }
    }

    /// Adds a dependency.
    #[must_use]
    pub fn depends_on(mut self, dep: ServiceId) -> Self {
        self.depends_on.insert(dep);
        self
    }
}

/// A deterministic service dependency graph.
///
/// The graph is keyed by `ServiceId` and supports deterministic iteration.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ServiceGraph {
    nodes: BTreeMap<ServiceId, ServiceNode>,
}

impl ServiceGraph {
    /// Creates an empty service graph.
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
        }
    }

    /// Inserts a node. Replaces any existing node with the same ID.
    pub fn insert(&mut self, node: ServiceNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Returns an iterator over nodes.
    pub fn nodes(&self) -> impl Iterator<Item = &ServiceNode> {
        self.nodes.values()
    }

    /// Validates the graph for basic invariants.
    ///
    /// Current invariants:
    ///
    /// - All dependencies must reference existing nodes.
    /// - The graph must be acyclic.
    pub fn validate(&self) -> Result<()> {
        for node in self.nodes.values() {
            for dep in &node.depends_on {
                if !self.nodes.contains_key(dep) {
                    return Err(Error::invalid_blueprint(
                        "service graph has a dependency referencing a missing service",
                    ));
                }
            }
        }

        self.validate_acyclic()?;
        Ok(())
    }

    fn validate_acyclic(&self) -> Result<()> {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        enum Mark {
            Temporary,
            Permanent,
        }

        let mut marks: BTreeMap<&ServiceId, Mark> = BTreeMap::new();

        fn visit<'a>(
            graph: &'a ServiceGraph,
            id: &'a ServiceId,
            marks: &mut BTreeMap<&'a ServiceId, Mark>,
        ) -> Result<()> {
            match marks.get(id).copied() {
                Some(Mark::Permanent) => return Ok(()),
                Some(Mark::Temporary) => {
                    return Err(Error::invalid_blueprint(
                        "service graph contains a dependency cycle",
                    ));
                }
                None => {}
            }

            marks.insert(id, Mark::Temporary);

            let node = graph
                .nodes
                .get(id)
                .ok_or_else(|| Error::invalid_blueprint("service graph references unknown node"))?;

            for dep in &node.depends_on {
                visit(graph, dep, marks)?;
            }

            marks.insert(id, Mark::Permanent);
            Ok(())
        }

        for id in self.nodes.keys() {
            visit(self, id, &mut marks)?;
        }

        Ok(())
    }

    /// Creates a default graph based on a suite-level [`ServiceSet`].
    ///
    /// This is intentionally minimal: it provides a stable baseline and room to expand.
    #[must_use]
    pub fn from_service_set(services: &ServiceSet) -> Self {
        let mut graph = Self::new();

        if services.base {
            graph.insert(ServiceNode::new(ServiceId("base")));
        }

        if services.observability {
            graph
                .insert(ServiceNode::new(ServiceId("observability")).depends_on(ServiceId("base")));
        }

        graph
    }

    /// If the `service-fabric` feature is enabled, this function can be used by downstream
    /// tooling as a stable integration point.
    #[cfg(feature = "service-fabric")]
    pub fn service_fabric_available() -> bool {
        true
    }
}
