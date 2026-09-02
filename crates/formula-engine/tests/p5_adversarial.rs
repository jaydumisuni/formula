use formula_core::digest::ArtifactDigest;
use formula_engine::{
    affine_polynomial::AffinePolynomialSpace,
    candidate_space::{CandidateSpaceContext, SearchAuthority},
    discovery::{run_bounded_cegis, CandidateValidation, CegisOutcome, DiscoveryOracle},
    observational::{FrozenExprCandidate, ObservationalExprSpace, U8BoolGrammar},
    query::RequestedResultClass,
    route_space::{ReductionRouteSpace, RouteCandidate, ScopedRouteFailure},
    search_policy::HeuristicRanking,
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn context() -> CandidateSpaceContext {
    CandidateSpaceContext::new(d(1), d(2), d(3), d(4), d(5), d(6))
}

#[test]
fn contradictory_exact_samples_empty_affine_space_instead_of_refuting_the_goal() {
    let mut space = AffinePolynomialSpace::new(context(), 1);
    space.add_exact_sample(0, 1).unwrap();
    space.add_exact_sample(0, 2).unwrap();
    assert!(space.empty().unwrap());
}

#[test]
fn heuristic_priority_cannot_delete_or_authorize_exact_route_candidates() {
    let a = RouteCandidate::new(
        d(10),
        "source",
        "a",
        vec![RequestedResultClass::Witness],
        vec![],
        true,
        1,
    );
    let b = RouteCandidate::new(
        d(11),
        "source",
        "b",
        vec![RequestedResultClass::Witness],
        vec![],
        true,
        2,
    );
    let space = ReductionRouteSpace::new(context(), vec![a, b]);
    let ranking = HeuristicRanking::new(vec![(d(10), 0), (d(11), 100)]);

    assert_eq!(ranking.authority(), SearchAuthority::CandidateOnly);
    assert!(space.contains(d(10)));
    assert!(space.contains(d(11)));
}

#[test]
fn scoped_failure_cannot_prune_an_unrelated_route() {
    let a = RouteCandidate::new(
        d(10),
        "source",
        "a",
        vec![RequestedResultClass::Decision],
        vec![],
        true,
        1,
    );
    let b = RouteCandidate::new(
        d(11),
        "source",
        "b",
        vec![RequestedResultClass::Decision],
        vec![],
        true,
        2,
    );
    let mut space = ReductionRouteSpace::new(context(), vec![a, b]);
    space.subtract_scoped_failure(&ScopedRouteFailure::new(d(30), vec![d(10)]));

    assert!(!space.contains(d(10)));
    assert!(space.contains(d(11)));
}

#[test]
fn bounded_search_exhaustion_has_no_refutation_variant() {
    struct CounterexampleForever;
    impl DiscoveryOracle for CounterexampleForever {
        fn output_for_sample(&mut self, input: u8) -> bool {
            input != 0
        }

        fn validate_frozen_candidate(
            &mut self,
            _candidate: &FrozenExprCandidate,
        ) -> CandidateValidation {
            CandidateValidation::Counterexample {
                input: 0,
                expected: false,
            }
        }
    }

    let mut space = ObservationalExprSpace::new(context(), U8BoolGrammar::minimal(), 5);
    let outcome = run_bounded_cegis(&mut space, &mut CounterexampleForever, &[1], 1);
    assert!(matches!(outcome, CegisOutcome::ResourceBoundedUnknown(_)));
}
