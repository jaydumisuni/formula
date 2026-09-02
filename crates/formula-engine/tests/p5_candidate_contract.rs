use formula_core::digest::ArtifactDigest;
use formula_engine::candidate_space::{
    CandidatePolarity, CandidateSpaceContext, CompletenessClass, FrozenCandidate,
    FrozenCandidateSpace, SearchAuthority,
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn context() -> CandidateSpaceContext {
    CandidateSpaceContext::new(d(1), d(2), d(3), d(4), d(5), d(6))
}

#[test]
fn context_identity_binds_only_local_semantic_inputs() {
    let a = context();
    let b = context();
    assert_eq!(a.digest(), b.digest());

    assert_ne!(a.digest(), a.clone().with_world(d(9)).digest());
    assert_ne!(a.digest(), a.clone().with_generation(d(9)).digest());
    assert_ne!(a.digest(), a.clone().with_query(d(9)).digest());
    assert_ne!(a.digest(), a.clone().with_grammar_or_routes(d(9)).digest());
    assert_ne!(a.digest(), a.with_policy(d(9)).digest());
}

#[test]
fn frozen_space_identity_is_deterministic_and_binds_semantics() {
    let a = FrozenCandidateSpace::new(
        context(),
        "affine-polynomial-v1",
        CandidatePolarity::Exact,
        CompletenessClass::CompleteWithinBound,
        d(10),
    );
    let b = FrozenCandidateSpace::new(
        context(),
        "affine-polynomial-v1",
        CandidatePolarity::Exact,
        CompletenessClass::CompleteWithinBound,
        d(10),
    );
    assert_eq!(a.digest(), b.digest());
    assert_ne!(
        a.digest(),
        FrozenCandidateSpace::new(
            context(),
            "affine-polynomial-v1",
            CandidatePolarity::HeuristicProposal,
            CompletenessClass::Incomplete,
            d(10),
        )
        .digest()
    );
}

#[test]
fn extracted_candidates_are_candidate_only() {
    let space = FrozenCandidateSpace::new(
        context(),
        "observational-expr-v1",
        CandidatePolarity::Exact,
        CompletenessClass::CompleteWithinBound,
        d(11),
    );
    let candidate = FrozenCandidate::new(space.digest(), d(12), 7);
    assert_eq!(candidate.authority(), SearchAuthority::CandidateOnly);
    assert_eq!(candidate.space_digest(), space.digest());
}
