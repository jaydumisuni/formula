use formula_core::{
    artifacts::StructuralIdentity,
    canonical::CanonicalValue,
    digest::ArtifactDigest,
    federation::{BridgeContract, CertifiedFederationFact},
    theory::{FactPolarity, SharedFact},
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn fact() -> SharedFact {
    SharedFact::new(
        d("world"),
        d("subject"),
        CanonicalValue::String("value".into()),
        FactPolarity::Exact,
        d("fact-evidence"),
    )
}

fn certified() -> CertifiedFederationFact {
    CertifiedFederationFact::new(
        fact(),
        d("package"),
        d("adapter"),
        d("translation"),
        d("checker-route"),
        d("semantic-input"),
        d("checked-evidence"),
    )
}

#[test]
fn certified_federation_fact_digest_binds_every_authority_input() {
    let baseline = certified().structural_digest();

    let variants = [
        CertifiedFederationFact::new(
            fact(),
            d("package-2"),
            d("adapter"),
            d("translation"),
            d("checker-route"),
            d("semantic-input"),
            d("checked-evidence"),
        ),
        CertifiedFederationFact::new(
            fact(),
            d("package"),
            d("adapter-2"),
            d("translation"),
            d("checker-route"),
            d("semantic-input"),
            d("checked-evidence"),
        ),
        CertifiedFederationFact::new(
            fact(),
            d("package"),
            d("adapter"),
            d("translation-2"),
            d("checker-route"),
            d("semantic-input"),
            d("checked-evidence"),
        ),
        CertifiedFederationFact::new(
            fact(),
            d("package"),
            d("adapter"),
            d("translation"),
            d("checker-route-2"),
            d("semantic-input"),
            d("checked-evidence"),
        ),
        CertifiedFederationFact::new(
            fact(),
            d("package"),
            d("adapter"),
            d("translation"),
            d("checker-route"),
            d("semantic-input-2"),
            d("checked-evidence"),
        ),
        CertifiedFederationFact::new(
            fact(),
            d("package"),
            d("adapter"),
            d("translation"),
            d("checker-route"),
            d("semantic-input"),
            d("checked-evidence-2"),
        ),
    ];

    for variant in variants {
        assert_ne!(baseline, variant.structural_digest());
    }
}

#[test]
fn certified_federation_fact_accessors_preserve_exact_provenance() {
    let certified = certified();
    assert_eq!(certified.package(), d("package"));
    assert_eq!(certified.adapter(), d("adapter"));
    assert_eq!(certified.translation(), d("translation"));
    assert_eq!(certified.checker_route(), d("checker-route"));
    assert_eq!(certified.semantic_input(), d("semantic-input"));
    assert_eq!(certified.evidence(), d("checked-evidence"));
    assert_eq!(certified.fact().polarity(), FactPolarity::Exact);
}

#[test]
fn bridge_identity_is_directional_and_binds_polarity() {
    let forward = BridgeContract::new(
        d("sat-package"),
        d("arithmetic-package"),
        d("sat-subject"),
        d("arithmetic-subject"),
        FactPolarity::Exact,
        FactPolarity::Exact,
        d("bridge-translation"),
        d("bridge-evidence"),
    );
    let reverse = BridgeContract::new(
        d("arithmetic-package"),
        d("sat-package"),
        d("arithmetic-subject"),
        d("sat-subject"),
        FactPolarity::Exact,
        FactPolarity::Exact,
        d("bridge-translation"),
        d("bridge-evidence"),
    );
    let weakened = BridgeContract::new(
        d("sat-package"),
        d("arithmetic-package"),
        d("sat-subject"),
        d("arithmetic-subject"),
        FactPolarity::Exact,
        FactPolarity::NecessaryCondition,
        d("bridge-translation"),
        d("bridge-evidence"),
    );

    assert_ne!(forward.structural_digest(), reverse.structural_digest());
    assert_ne!(forward.structural_digest(), weakened.structural_digest());
    assert_eq!(forward.source_package(), d("sat-package"));
    assert_eq!(forward.target_package(), d("arithmetic-package"));
    assert_eq!(forward.source_subject(), d("sat-subject"));
    assert_eq!(forward.target_subject(), d("arithmetic-subject"));
}
