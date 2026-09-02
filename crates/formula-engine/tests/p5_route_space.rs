use formula_core::digest::ArtifactDigest;
use formula_engine::{
    candidate_space::CandidateSpaceContext,
    query::RequestedResultClass,
    route_space::{ReductionRouteSpace, RouteCandidate, ScopedRouteFailure},
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn context() -> CandidateSpaceContext {
    CandidateSpaceContext::new(d(1), d(2), d(3), d(4), d(5), d(6))
}

fn route(id: u8, target: &str, classes: Vec<RequestedResultClass>, caps: Vec<ArtifactDigest>, cost: u64) -> RouteCandidate {
    RouteCandidate::new(d(id), "boolean-xor", target, classes, caps, true, cost)
}

#[test]
fn inadmissible_cheaper_route_cannot_win() {
    let direct = route(10, "boolean-direct", vec![RequestedResultClass::Decision], vec![], 1);
    let gf2 = route(11, "gf2-affine", vec![RequestedResultClass::Decision, RequestedResultClass::Witness], vec![d(20)], 3);
    let mut space = ReductionRouteSpace::new(context(), vec![direct, gf2]);
    space.restrict_result_class(RequestedResultClass::Witness);
    space.restrict_capabilities(&[d(20)]);
    assert_eq!(space.extract_min_cost().unwrap().route_digest(), d(11));
}

#[test]
fn failure_pruning_is_scoped() {
    let a = route(10, "a", vec![RequestedResultClass::Witness], vec![], 1);
    let b = route(11, "b", vec![RequestedResultClass::Witness], vec![], 2);
    let mut space = ReductionRouteSpace::new(context(), vec![a, b]);
    space.subtract_scoped_failure(&ScopedRouteFailure::new(d(30), vec![d(10)]));
    assert!(!space.contains(d(10)));
    assert!(space.contains(d(11)));
}

#[test]
fn route_order_is_non_semantic() {
    let a = route(10, "a", vec![RequestedResultClass::Witness], vec![], 2);
    let b = route(11, "b", vec![RequestedResultClass::Witness], vec![], 1);
    let left = ReductionRouteSpace::new(context(), vec![a.clone(), b.clone()]);
    let right = ReductionRouteSpace::new(context(), vec![b, a]);
    assert_eq!(left.freeze().digest(), right.freeze().digest());
    assert_eq!(left.partition_by_target(), right.partition_by_target());
}
