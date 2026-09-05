use formula_core::{
    artifacts::StructuralIdentity, digest::ArtifactDigest, self_expansion::GrammarGeneration,
};
use formula_engine::{
    candidate_space::CandidateSpaceContext,
    self_expansion::{
        GrammarBindingError, bind_candidate_context_to_grammar, validate_candidate_context_grammar,
    },
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn base_context(generation: ArtifactDigest) -> CandidateSpaceContext {
    CandidateSpaceContext::new(
        generation,
        d("p10:world"),
        d("p10:query"),
        d("p10:obligation"),
        d("p10:legacy-grammar"),
        d("p10:policy"),
    )
}

#[test]
fn candidate_space_context_binds_exact_lambda_digest() {
    let generation = d("p10:u:g");
    let lambda_g = GrammarGeneration::new(
        generation,
        None,
        vec![d("p10:constructor:a")],
        vec![],
        vec![],
        vec![d("p10:route:a")],
        vec![d("p10:theory:a")],
    );

    let bound = bind_candidate_context_to_grammar(base_context(generation), &lambda_g).unwrap();
    assert_eq!(
        bound.context().grammar_or_routes_digest(),
        lambda_g.structural_digest()
    );
    assert_eq!(bound.grammar_digest(), lambda_g.structural_digest());
}

#[test]
fn candidate_built_under_lambda_g_cannot_be_silently_reused_under_lambda_g1() {
    let generation = d("p10:u:g");
    let lambda_g = GrammarGeneration::new(
        generation,
        None,
        vec![d("p10:constructor:a")],
        vec![],
        vec![],
        vec![d("p10:route:a")],
        vec![d("p10:theory:a")],
    );
    let lambda_g1 = GrammarGeneration::new(
        generation,
        Some(lambda_g.structural_digest()),
        vec![d("p10:constructor:a"), d("p10:constructor:b")],
        vec![],
        vec![],
        vec![d("p10:route:a")],
        vec![d("p10:theory:a")],
    );

    let old = bind_candidate_context_to_grammar(base_context(generation), &lambda_g).unwrap();
    assert_eq!(
        validate_candidate_context_grammar(old.context(), &lambda_g),
        Ok(())
    );
    assert_eq!(
        validate_candidate_context_grammar(old.context(), &lambda_g1),
        Err(GrammarBindingError::GrammarDigestMismatch)
    );

    let rebuilt = bind_candidate_context_to_grammar(base_context(generation), &lambda_g1).unwrap();
    assert_ne!(old.context().digest(), rebuilt.context().digest());
}

#[test]
fn grammar_binding_rejects_generation_mismatch() {
    let context_generation = d("p10:u:g");
    let lambda =
        GrammarGeneration::new(d("p10:u:g1"), None, vec![], vec![], vec![], vec![], vec![]);

    assert_eq!(
        bind_candidate_context_to_grammar(base_context(context_generation), &lambda),
        Err(GrammarBindingError::GenerationMismatch)
    );
}
