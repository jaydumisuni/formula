use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    self_expansion::{
        ActivationMode, ClassifiedPromotionCandidate, EvidenceFreshness, GrammarGeneration,
        PromotionClass, PromotionClassRegistryV1, SemanticChange, SemanticChangeClass,
        SupersessionKind,
    },
};

fn digest(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

#[test]
fn every_frozen_d5_promotion_class_has_one_deterministic_policy() {
    let policies = PromotionClassRegistryV1::policies();
    assert_eq!(policies.len(), PromotionClass::ALL.len());
    for class in PromotionClass::ALL {
        assert_eq!(policies.iter().filter(|policy| policy.class() == class).count(), 1);
    }
    assert_eq!(
        PromotionClassRegistryV1::digest(),
        PromotionClassRegistryV1::digest()
    );
}

#[test]
fn classified_promotion_identity_is_order_independent_and_effect_sensitive() {
    let left = ClassifiedPromotionCandidate::new(
        digest("base"),
        PromotionClass::StructureWitness,
        ActivationMode::DefaultAutomatic,
        SemanticChangeClass::ConservativeExtension,
        vec![digest("cap-b"), digest("cap-a"), digest("cap-a")],
        vec![],
        vec![digest("scope-b"), digest("scope-a")],
    );
    let right = ClassifiedPromotionCandidate::new(
        digest("base"),
        PromotionClass::StructureWitness,
        ActivationMode::DefaultAutomatic,
        SemanticChangeClass::ConservativeExtension,
        vec![digest("cap-a"), digest("cap-b")],
        vec![],
        vec![digest("scope-a"), digest("scope-b")],
    );
    let different = ClassifiedPromotionCandidate::new(
        digest("base"),
        PromotionClass::SemanticPrimitive,
        ActivationMode::DefaultAutomatic,
        SemanticChangeClass::ConservativeExtension,
        vec![digest("cap-a"), digest("cap-b")],
        vec![],
        vec![digest("scope-a"), digest("scope-b")],
    );

    assert_eq!(left.structural_digest(), right.structural_digest());
    assert_ne!(left.structural_digest(), different.structural_digest());
}

#[test]
fn grammar_generation_identity_is_generation_and_activation_bound() {
    let lambda_g = GrammarGeneration::new(
        digest("u-g"),
        None,
        vec![digest("ctor")],
        vec![],
        vec![digest("shadow")],
        vec![digest("route")],
        vec![digest("theory-rule")],
    );
    let lambda_g1 = GrammarGeneration::new(
        digest("u-g1"),
        Some(lambda_g.structural_digest()),
        vec![digest("ctor")],
        vec![digest("meta")],
        vec![],
        vec![digest("route")],
        vec![digest("theory-rule")],
    );

    assert_ne!(lambda_g.structural_digest(), lambda_g1.structural_digest());
    assert_eq!(lambda_g.shadow_metaprimitives(), &[digest("shadow")]);
    assert_eq!(lambda_g1.activated_metaprimitives(), &[digest("meta")]);
}

#[test]
fn semantic_change_identity_tracks_class_and_dependency_cone() {
    let first = SemanticChange::new(
        digest("old"),
        digest("new"),
        SemanticChangeClass::DefinitionalEquivalent,
        vec![digest("dep-b"), digest("dep-a")],
        vec![digest("cone-b"), digest("cone-a")],
        vec![digest("evidence")],
    );
    let second = SemanticChange::new(
        digest("old"),
        digest("new"),
        SemanticChangeClass::DefinitionalEquivalent,
        vec![digest("dep-a"), digest("dep-b")],
        vec![digest("cone-a"), digest("cone-b")],
        vec![digest("evidence")],
    );

    assert_eq!(first.structural_digest(), second.structural_digest());
    assert_eq!(EvidenceFreshness::Transportable.as_str(), "TRANSPORTABLE");
    assert_eq!(SupersessionKind::ReplacedRealizationBy.as_str(), "REPLACED_REALIZATION_BY");
}
