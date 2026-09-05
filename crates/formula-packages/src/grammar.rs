use crate::expansion::{ExpansionError, PromotedRouteActivation, active_routes, validate_record};
use formula_core::{
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    self_expansion::{ActivationMode, ExpansionActivationRecord, GrammarGeneration, PromotionClass},
};

pub fn derive_grammar_generation(
    generation: &UniverseGeneration,
    parent_grammar: Option<ArtifactDigest>,
    activations: &[ExpansionActivationRecord],
    routes: &[PromotedRouteActivation],
    theory_rules: &[ArtifactDigest],
) -> Result<GrammarGeneration, ExpansionError> {
    let mut activated_constructors = Vec::new();
    let mut activated_metaprimitives = Vec::new();
    let mut shadow_metaprimitives = Vec::new();

    for record in activations {
        validate_record(generation, record)?;
        if matches!(
            record.mode(),
            ActivationMode::Superseded | ActivationMode::Quarantined
        ) {
            continue;
        }

        match (record.promotion_class(), record.mode()) {
            (PromotionClass::MetaprimitiveSearchMethod, ActivationMode::ShadowOnly) => {
                shadow_metaprimitives.push(record.subject());
            }
            (
                PromotionClass::MetaprimitiveSearchMethod,
                ActivationMode::BoundedAutomatic | ActivationMode::DefaultAutomatic,
            ) => {
                activated_metaprimitives.push(record.subject());
            }
            (
                PromotionClass::Representation
                | PromotionClass::DecompositionSufficientSummary
                | PromotionClass::SemanticPrimitive
                | PromotionClass::Capability
                | PromotionClass::PackageTheoryExtension,
                ActivationMode::BoundedAutomatic | ActivationMode::DefaultAutomatic,
            ) => {
                activated_constructors.push(record.subject());
            }
            _ => {}
        }
    }

    for route in routes {
        if route.generation() != generation.digest() {
            return Err(ExpansionError::GenerationMismatch);
        }
    }

    Ok(GrammarGeneration::new(
        generation.digest(),
        parent_grammar,
        activated_constructors,
        activated_metaprimitives,
        shadow_metaprimitives,
        active_routes(routes),
        theory_rules.to_vec(),
    ))
}
