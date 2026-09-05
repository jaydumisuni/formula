use crate::candidate_space::CandidateSpaceContext;
use formula_core::{
    artifacts::StructuralIdentity, digest::ArtifactDigest, self_expansion::GrammarGeneration,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GrammarBindingError {
    GenerationMismatch,
    GrammarDigestMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrammarBoundCandidateContext {
    context: CandidateSpaceContext,
    grammar_digest: ArtifactDigest,
}

impl GrammarBoundCandidateContext {
    pub fn context(&self) -> &CandidateSpaceContext {
        &self.context
    }

    pub fn grammar_digest(&self) -> ArtifactDigest {
        self.grammar_digest
    }
}

pub fn bind_candidate_context_to_grammar(
    context: CandidateSpaceContext,
    grammar: &GrammarGeneration,
) -> Result<GrammarBoundCandidateContext, GrammarBindingError> {
    if context.universe_generation() != grammar.universe_generation() {
        return Err(GrammarBindingError::GenerationMismatch);
    }

    let grammar_digest = grammar.structural_digest();
    Ok(GrammarBoundCandidateContext {
        context: context.with_grammar_or_routes(grammar_digest),
        grammar_digest,
    })
}

pub fn validate_candidate_context_grammar(
    context: &CandidateSpaceContext,
    grammar: &GrammarGeneration,
) -> Result<(), GrammarBindingError> {
    if context.universe_generation() != grammar.universe_generation() {
        return Err(GrammarBindingError::GenerationMismatch);
    }
    if context.grammar_or_routes_digest() != grammar.structural_digest() {
        return Err(GrammarBindingError::GrammarDigestMismatch);
    }
    Ok(())
}
