use os_foundry_suite::prelude::*;

#[test]
fn execution_plan_is_deterministic_for_dev_profile() {
    let bp = Profiles::dev_qemu();
    let plan = plan_execution(bp, Target::x86_64_bare_metal()).unwrap();

    assert_eq!(plan.build.program, "cargo");
    assert!(plan.run.is_some());
}

#[test]
fn service_graph_from_service_set_validates() {
    let services = ServiceSet::dev();
    let graph = ServiceGraph::from_service_set(&services);
    graph.validate().unwrap();
}

#[test]
fn service_graph_detects_missing_dependency() {
    let mut graph = ServiceGraph::new();
    graph.insert(ServiceNode::new(ServiceId("a")).depends_on(ServiceId("missing")));

    let err = graph.validate().unwrap_err();
    assert_eq!(
        err.to_string(),
        "invalid blueprint: service graph has a dependency referencing a missing service"
    );
}

#[test]
fn service_graph_detects_cycle() {
    let mut graph = ServiceGraph::new();
    graph.insert(ServiceNode::new(ServiceId("a")).depends_on(ServiceId("b")));
    graph.insert(ServiceNode::new(ServiceId("b")).depends_on(ServiceId("a")));

    let err = graph.validate().unwrap_err();
    assert_eq!(
        err.to_string(),
        "invalid blueprint: service graph contains a dependency cycle"
    );
}
