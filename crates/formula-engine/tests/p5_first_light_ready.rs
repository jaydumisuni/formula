use formula_core::digest::ArtifactDigest;
use formula_engine::{
    affine_polynomial::{AffinePolynomialSpace, Rational128},
    candidate_space::CandidateSpaceContext,
    observational::{BoolExpr, ByteExpr, ObservationalExprSpace, U8BoolGrammar},
    query::RequestedResultClass,
    route_space::{ReductionRouteSpace, RouteCandidate},
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn context() -> CandidateSpaceContext {
    CandidateSpaceContext::new(d(1), d(2), d(3), d(4), d(5), d(6))
}

#[test]
fn public_polynomial_fixture_refines_the_whole_affine_family() {
    let mut space = AffinePolynomialSpace::new(context(), 2);
    let before = space.affine_dimension().unwrap();
    space.add_exact_sample(0, 4).unwrap();
    let after_one = space.affine_dimension().unwrap();
    space.add_exact_sample(1, 7).unwrap();
    let after_two = space.affine_dimension().unwrap();
    space.add_exact_sample(2, 10).unwrap();
    let after_three = space.affine_dimension().unwrap();

    assert_eq!((before, after_one, after_two, after_three), (3, 2, 1, 0));
    assert_eq!(
        space.extract_min_degree_unique().unwrap().coefficients(),
        &[Rational128::integer(4), Rational128::integer(3)]
    );
}

#[test]
fn public_route_fixture_selects_exact_lower_cost_route_without_name_dispatch() {
    let expensive = RouteCandidate::new(
        d(10),
        "public-source",
        "target-a",
        vec![RequestedResultClass::Witness],
        vec![d(20)],
        true,
        9,
    );
    let cheap = RouteCandidate::new(
        d(11),
        "public-source",
        "target-b",
        vec![RequestedResultClass::Witness],
        vec![d(20)],
        true,
        2,
    );
    let mut space = ReductionRouteSpace::new(context(), vec![expensive, cheap]);
    space.restrict_result_class(RequestedResultClass::Witness);
    space.restrict_capabilities(&[d(20)]);

    assert_eq!(space.extract_min_cost().unwrap().route_digest(), d(11));
}

#[test]
fn public_zero_related_near_miss_is_eliminated_by_counterexample() {
    let near_miss = BoolExpr::eq_zero(ByteExpr::bit_and(
        ByteExpr::x(),
        ByteExpr::sub_wrap(ByteExpr::x(), ByteExpr::one()),
    ));
    assert!(near_miss.eval(1));
    assert!(!near_miss.eval(3));

    let mut space = ObservationalExprSpace::new(context(), U8BoolGrammar::minimal(), 7);
    space.restrict_exact_sample(1, true);
    let before = space.freeze().digest();
    space.refine_counterexample(3, true);
    let after = space.freeze().digest();

    assert_ne!(before, after);
    assert!(
        space
            .behavior_representatives()
            .iter()
            .all(|representative| representative.expression().eval(3))
    );
}
