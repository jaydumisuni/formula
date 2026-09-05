use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    self_expansion::{RealizationUpgrade, SemanticChangeClass},
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

#[test]
fn realization_upgrade_identity_is_selection_only_and_variant_sensitive() {
    let generation = d("p10:u1");
    let semantic = d("p10:semantic");
    let r1 = d("p10:r1");
    let r2 = d("p10:r2");
    let upgrade = RealizationUpgrade::new(
        semantic,
        generation,
        r1,
        r2,
        SemanticChangeClass::RealizationOnly,
        vec![d("p10:validation")],
        d("p10:selection-policy"),
    );
    let reverse = RealizationUpgrade::new(
        semantic,
        generation,
        r2,
        r1,
        SemanticChangeClass::RealizationOnly,
        vec![d("p10:validation")],
        d("p10:selection-policy"),
    );

    assert_eq!(upgrade.semantic_artifact(), semantic);
    assert_eq!(upgrade.universe_generation(), generation);
    assert_eq!(upgrade.semantic_change_class(), SemanticChangeClass::RealizationOnly);
    assert_ne!(upgrade.structural_digest(), reverse.structural_digest());
}
