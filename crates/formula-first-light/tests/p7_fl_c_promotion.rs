use formula_check::promotion::{PromotionDecision, authorize_promotion_v1};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::{FrozenCandidate as CertifiedFrozenCandidate, PromotionManifest},
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    promotion::{PromotionCandidate, PromotionRecord, PromotionState},
};
use formula_engine::{
    candidate_space::CandidateSpaceContext,
    observational::{ObservationalExprSpace, U8BoolGrammar},
};
use formula_first_light::fl_c::{fl_c_grammar_digest, fl_c_oracle, fl_c_target_digest};
use formula_store::authority_store::AuthorityStore;
use tempfile::tempdir;

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

#[test]
fn fl_c_semantic_primitive_is_frozen_certified_and_promoted_to_u1() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![], vec![]);
    let u0_digest = store.initialize_genesis(&u0).unwrap();
    let u0_bytes = u0.canonical_bytes();
    let world = d("p7-fl-c-world");

    let context = CandidateSpaceContext::new(
        u0_digest,
        world,
        d("p7-fl-c-query"),
        d("p7-fl-c-obligation"),
        fl_c_grammar_digest(),
        d("p7-fl-c-search-policy"),
    );
    let mut space = ObservationalExprSpace::new(context, U8BoolGrammar::minimal(), 9);

    let final_candidate = (0..=u8::MAX)
        .find_map(|_| {
            let candidate = space.extract_min_cost().expect("bounded FL-C candidate");
            match fl_c_oracle().first_counterexample(&candidate) {
                Some((input, expected)) => {
                    space.refine_counterexample(input, expected);
                    None
                }
                None => Some(candidate),
            }
        })
        .expect("bounded FL-C discovery converges within U8 counterexample bound");

    assert_eq!(fl_c_oracle().first_counterexample(&final_candidate), None);

    let primitive = final_candidate.expression().digest();
    let evidence = d("p7-fl-c-exhaustive-equivalence-evidence");
    let frozen = CertifiedFrozenCandidate::new(
        "first-light-fl-c-semantic-primitive".into(),
        vec![primitive],
        world,
        u0_digest,
        vec![],
        vec![fl_c_target_digest()],
        d("p7-fl-c-authority-contract"),
        d("p7-fl-c-observer"),
    );
    let manifest = PromotionManifest::new(
        u0_digest,
        frozen.structural_digest(),
        vec![evidence],
        vec![primitive],
        vec![evidence],
    );
    let promotion = PromotionCandidate::new(
        frozen.structural_digest(),
        manifest.structural_digest(),
        u0_digest,
        u0_digest,
        vec![],
        vec![],
    );

    let decision = authorize_promotion_v1(
        &manifest,
        &frozen,
        &promotion,
        &[evidence],
        &u0,
        &[],
    )
    .unwrap();
    let PromotionDecision::Authorized(authorization) = decision else {
        panic!("valid FL-C semantic primitive was quarantined")
    };

    let certified = PromotionRecord::new(
        promotion.structural_digest(),
        PromotionState::Certified,
        u0_digest,
        authorization.policy_digest(),
        vec![evidence],
        vec![primitive],
    );
    let outcome = store.promote(&authorization).unwrap();
    let u1 = store.replay_generation(outcome.new_generation()).unwrap();
    let activated = PromotionRecord::new(
        promotion.structural_digest(),
        PromotionState::Activated,
        outcome.new_generation(),
        authorization.policy_digest(),
        vec![evidence],
        vec![primitive],
    );

    assert_eq!(outcome.parent_generation(), u0_digest);
    assert!(u1.admitted().contains(&primitive));
    assert!(u1.authority_bindings().contains(&evidence));
    assert_ne!(certified.structural_digest(), outcome.admitted_record().structural_digest());
    assert_ne!(outcome.admitted_record().structural_digest(), activated.structural_digest());
    assert_ne!(certified.structural_digest(), activated.structural_digest());

    let replayed_u0 = store.replay_generation(u0_digest).unwrap();
    assert_eq!(replayed_u0.digest(), u0_digest);
    assert_eq!(replayed_u0.canonical_bytes(), u0_bytes);
    assert!(!replayed_u0.admitted().contains(&primitive));
}
