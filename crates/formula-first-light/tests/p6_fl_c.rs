use formula_core::digest::ArtifactDigest;
use formula_engine::{
    candidate_space::CandidateSpaceContext,
    observational::{ObservationalExprSpace, U8BoolGrammar},
};
use formula_first_light::fl_c::{
    fl_c_grammar_digest, fl_c_oracle, fl_c_target_digest, fl_c_zero_near_miss,
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn context() -> CandidateSpaceContext {
    CandidateSpaceContext::new(d(1), d(2), d(3), d(4), d(5), d(6))
}

fn expected(input: u8) -> bool {
    matches!(input, 1 | 2 | 4 | 8 | 16 | 32 | 64 | 128)
}

#[test]
fn mandatory_zero_near_miss_is_visible_then_rejected_by_sealed_oracle() {
    let near_miss = fl_c_zero_near_miss();
    assert!(near_miss.eval(0));
    assert!(near_miss.eval(1));
    assert!(!near_miss.eval(3));

    let mut space = ObservationalExprSpace::new(context(), U8BoolGrammar::minimal(), 6);
    for input in 1_u8..=u8::MAX {
        space.restrict_exact_sample(input, expected(input));
    }
    let candidate = space.extract_min_cost().expect("near-miss candidate");
    assert_eq!(candidate.expression(), &near_miss);
    assert_eq!(
        fl_c_oracle().first_counterexample(&candidate),
        Some((0, false))
    );
}

#[test]
fn exhaustive_frozen_candidate_has_no_hidden_counterexample() {
    let mut space = ObservationalExprSpace::new(context(), U8BoolGrammar::minimal(), 9);
    for input in u8::MIN..=u8::MAX {
        space.restrict_exact_sample(input, expected(input));
    }
    let candidate = space.extract_min_cost().expect("exact candidate");
    assert_eq!(fl_c_oracle().first_counterexample(&candidate), None);
}

#[test]
fn target_and_grammar_bindings_are_stable_and_distinct() {
    assert_eq!(fl_c_grammar_digest(), U8BoolGrammar::minimal().digest());
    assert_eq!(fl_c_target_digest(), fl_c_target_digest());
    assert_ne!(fl_c_target_digest(), fl_c_grammar_digest());
}
