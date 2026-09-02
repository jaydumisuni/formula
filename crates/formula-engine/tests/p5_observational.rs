use formula_core::digest::ArtifactDigest;
use formula_engine::{
    candidate_space::CandidateSpaceContext,
    observational::{BoolExpr, ByteExpr, ObservationalExprSpace, U8BoolGrammar},
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn context() -> CandidateSpaceContext {
    CandidateSpaceContext::new(d(1), d(2), d(3), d(4), d(5), d(6))
}

#[test]
fn typed_grammar_uses_exact_u8_wrapping_semantics() {
    let expr = ByteExpr::sub_wrap(ByteExpr::x(), ByteExpr::one());
    assert_eq!(expr.eval(0), 255);

    let bool_expr = BoolExpr::eq_zero(ByteExpr::bit_and(ByteExpr::x(), expr));
    assert!(bool_expr.eval(0));
    assert!(bool_expr.eval(1));
    assert!(!bool_expr.eval(3));
}

#[test]
fn ast_identity_and_cost_are_deterministic() {
    let a = BoolExpr::and(
        BoolExpr::neq_zero(ByteExpr::x()),
        BoolExpr::eq_zero(ByteExpr::bit_and(
            ByteExpr::x(),
            ByteExpr::sub_wrap(ByteExpr::x(), ByteExpr::one()),
        )),
    );
    let b = a.clone();
    assert_eq!(a.digest(), b.digest());
    assert_eq!(a.cost(), b.cost());
}

#[test]
fn observational_space_rebuild_is_equivalent_to_fresh_space() {
    let grammar = U8BoolGrammar::minimal();
    let mut incremental = ObservationalExprSpace::new(context(), grammar.clone(), 7);
    incremental.restrict_exact_sample(1, true);
    incremental.restrict_exact_sample(3, false);

    let mut fresh = ObservationalExprSpace::new(context(), grammar, 7);
    fresh.restrict_exact_sample(3, false);
    fresh.restrict_exact_sample(1, true);

    assert_eq!(incremental.freeze().digest(), fresh.freeze().digest());
    assert_eq!(incremental.extract_min_cost().map(|c| c.digest()), fresh.extract_min_cost().map(|c| c.digest()));
}

#[test]
fn behavior_bucketing_keeps_one_lowest_cost_representative() {
    let mut space = ObservationalExprSpace::new(context(), U8BoolGrammar::minimal(), 5);
    space.restrict_exact_sample(1, true);
    let reps = space.behavior_representatives();
    assert!(!reps.is_empty());
    for pair in reps.windows(2) {
        assert_ne!(pair[0].behavior(), pair[1].behavior());
    }
}
