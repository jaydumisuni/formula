use formula_core::digest::ArtifactDigest;
use formula_engine::{
    affine_polynomial::AffinePolynomialSpace,
    candidate_space::CandidateSpaceContext,
    observational::{ObservationalExprSpace, U8BoolGrammar},
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn context() -> CandidateSpaceContext {
    CandidateSpaceContext::new(d(1), d(2), d(3), d(4), d(5), d(6))
}

#[test]
fn identical_local_inputs_recompile_to_identical_space_and_candidate_identity() {
    let mut left = AffinePolynomialSpace::new(context(), 2);
    let mut right = AffinePolynomialSpace::new(context(), 2);
    for (x, y) in [(0, 3), (1, 5), (2, 7)] {
        left.add_exact_sample(x, y).unwrap();
        right.add_exact_sample(x, y).unwrap();
    }

    assert_eq!(left.freeze().digest(), right.freeze().digest());
    assert_eq!(
        left.extract_min_degree_unique().unwrap().digest(),
        right.extract_min_degree_unique().unwrap().digest()
    );
}

#[test]
fn unrelated_campaign_state_cannot_perturb_local_candidate_space_identity() {
    let unrelated_campaign_before = d(40);
    let unrelated_campaign_after = d(41);
    assert_ne!(unrelated_campaign_before, unrelated_campaign_after);

    let mut before = ObservationalExprSpace::new(context(), U8BoolGrammar::minimal(), 5);
    let mut after = ObservationalExprSpace::new(context(), U8BoolGrammar::minimal(), 5);
    before.restrict_exact_sample(1, true);
    after.restrict_exact_sample(1, true);

    assert_eq!(before.freeze().digest(), after.freeze().digest());
    assert_eq!(
        before.extract_min_cost().map(|candidate| candidate.digest()),
        after.extract_min_cost().map(|candidate| candidate.digest())
    );
}

#[test]
fn local_semantic_context_changes_perturb_identity() {
    let base = context();
    let variants = [
        base.clone().with_generation(d(11)),
        base.clone().with_world(d(12)),
        base.clone().with_query(d(13)),
        base.clone().with_grammar_or_routes(d(14)),
        base.clone().with_policy(d(15)),
    ];

    let base_digest = ObservationalExprSpace::new(base, U8BoolGrammar::minimal(), 5)
        .freeze()
        .digest();
    for variant in variants {
        assert_ne!(
            base_digest,
            ObservationalExprSpace::new(variant, U8BoolGrammar::minimal(), 5)
                .freeze()
                .digest()
        );
    }
}
