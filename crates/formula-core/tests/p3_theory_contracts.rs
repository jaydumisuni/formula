use formula_core::{
    artifacts::StructuralIdentity,
    canonical::CanonicalValue,
    digest::ArtifactDigest,
    theory::{CapabilityContract, ClosureContext, FactPolarity, SharedFact, TheoryPackageManifest},
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

#[test]
fn package_manifest_identity_is_set_normalized() {
    let a = TheoryPackageManifest::new(
        "pkg.integer.v1".into(),
        d("foundation"),
        vec![d("export-b"), d("export-a"), d("export-a")],
        vec![
            CapabilityContract::new(d("cap-b"), vec![d("goal-b")]),
            CapabilityContract::new(d("cap-a"), vec![d("goal-a")]),
        ],
        vec![d("dependency-b"), d("dependency-a"), d("dependency-a")],
        vec!["rewrite:add".into(), "symbol:+".into(), "symbol:+".into()],
    );
    let b = TheoryPackageManifest::new(
        "pkg.integer.v1".into(),
        d("foundation"),
        vec![d("export-a"), d("export-b")],
        vec![
            CapabilityContract::new(d("cap-a"), vec![d("goal-a")]),
            CapabilityContract::new(d("cap-b"), vec![d("goal-b")]),
        ],
        vec![d("dependency-a"), d("dependency-b")],
        vec!["symbol:+".into(), "rewrite:add".into()],
    );

    assert_eq!(a.structural_digest(), b.structural_digest());
}

#[test]
fn closure_context_identity_is_generation_and_world_scoped() {
    let base = ClosureContext::new(
        d("generation-1"),
        d("world-1"),
        vec![d("package-a")],
        d("rules"),
        d("authority-policy"),
    );
    let another_world = ClosureContext::new(
        d("generation-1"),
        d("world-2"),
        vec![d("package-a")],
        d("rules"),
        d("authority-policy"),
    );
    let another_generation = ClosureContext::new(
        d("generation-2"),
        d("world-1"),
        vec![d("package-a")],
        d("rules"),
        d("authority-policy"),
    );

    assert_ne!(base.structural_digest(), another_world.structural_digest());
    assert_ne!(
        base.structural_digest(),
        another_generation.structural_digest()
    );
}

#[test]
fn shared_fact_polarity_is_part_of_structural_identity() {
    let exact = SharedFact::new(
        d("world"),
        d("subject"),
        CanonicalValue::String("x in [2,3]".into()),
        FactPolarity::Exact,
        d("evidence"),
    );
    let over = SharedFact::new(
        d("world"),
        d("subject"),
        CanonicalValue::String("x in [2,3]".into()),
        FactPolarity::OverApproximation,
        d("evidence"),
    );

    assert_ne!(exact.structural_digest(), over.structural_digest());
}
