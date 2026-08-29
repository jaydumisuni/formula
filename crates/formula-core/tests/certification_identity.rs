use formula_core::{
    artifacts::{
        AuthorityContract, EvidenceEnvelope, Judgement, Observer, RealizationMetadata,
        StructuralIdentity,
    },
    canonical::CanonicalValue,
    certification::{
        CertificateEnvelope, FrozenCandidate, PromotionManifest, RealizationCheckManifest,
    },
    digest::ArtifactDigest,
};

fn d(label: &[u8]) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label)
}

fn frozen(semantic: Vec<ArtifactDigest>, dependencies: Vec<ArtifactDigest>) -> FrozenCandidate {
    FrozenCandidate::new(
        "semantic-primitive".into(),
        semantic,
        d(b"world"),
        d(b"generation"),
        dependencies,
        vec![d(b"judgement")],
        d(b"authority-contract"),
        d(b"observer"),
    )
}

fn envelope(
    candidate: ArtifactDigest,
    generation: ArtifactDigest,
    world: ArtifactDigest,
    checker: ArtifactDigest,
    body: ArtifactDigest,
) -> CertificateEnvelope {
    CertificateEnvelope::new(
        candidate,
        d(b"target"),
        generation,
        world,
        CanonicalValue::String("universal".into()),
        "PROVED".into(),
        "EXHAUSTIVE".into(),
        "u8-exhaustive".into(),
        "1".into(),
        body,
        d(b"producer"),
        checker,
        d(b"checker-trust-root"),
        vec![d(b"dependency-a"), d(b"dependency-b")],
        d(b"authority-contract"),
        d(b"observer"),
        CanonicalValue::String("replay-v1".into()),
    )
}

#[test]
fn frozen_candidate_digest_changes_when_candidate_content_changes() {
    let original = frozen(vec![d(b"semantic-a")], vec![d(b"dependency")]);
    let changed = frozen(vec![d(b"semantic-b")], vec![d(b"dependency")]);
    assert_ne!(original.structural_digest(), changed.structural_digest());
}

#[test]
fn frozen_candidate_normalizes_set_like_inputs() {
    let a = frozen(
        vec![d(b"semantic-b"), d(b"semantic-a"), d(b"semantic-a")],
        vec![d(b"dependency-b"), d(b"dependency-a"), d(b"dependency-a")],
    );
    let b = frozen(
        vec![d(b"semantic-a"), d(b"semantic-b")],
        vec![d(b"dependency-a"), d(b"dependency-b")],
    );
    assert_eq!(a.structural_digest(), b.structural_digest());
}

#[test]
fn certificate_envelope_binds_generation_world_checker_and_body() {
    let candidate = frozen(vec![d(b"semantic")], vec![]).structural_digest();
    let base = envelope(
        candidate,
        d(b"generation-a"),
        d(b"world-a"),
        d(b"checker-a"),
        d(b"body-a"),
    );

    for changed in [
        envelope(
            candidate,
            d(b"generation-b"),
            d(b"world-a"),
            d(b"checker-a"),
            d(b"body-a"),
        ),
        envelope(
            candidate,
            d(b"generation-a"),
            d(b"world-b"),
            d(b"checker-a"),
            d(b"body-a"),
        ),
        envelope(
            candidate,
            d(b"generation-a"),
            d(b"world-a"),
            d(b"checker-b"),
            d(b"body-a"),
        ),
        envelope(
            candidate,
            d(b"generation-a"),
            d(b"world-a"),
            d(b"checker-a"),
            d(b"body-b"),
        ),
    ] {
        assert_ne!(base.structural_digest(), changed.structural_digest());
    }
}

#[test]
fn p1_accessors_are_read_only_and_do_not_change_structural_identity() {
    let judgement = Judgement::new(
        d(b"world"),
        CanonicalValue::String("claim".into()),
        vec![d(b"reference")],
    );
    let evidence = EvidenceEnvelope::new(
        judgement.structural_digest(),
        d(b"world"),
        CanonicalValue::String("global".into()),
        "fixture".into(),
        d(b"body"),
        d(b"producer"),
        d(b"checker"),
        d(b"trust-root"),
        "PROVED".into(),
        vec![d(b"reference")],
        CanonicalValue::String("replay-v1".into()),
    );
    let authority = AuthorityContract::new(
        "deterministic-proof".into(),
        vec!["fixture".into()],
        "exact".into(),
    );
    let observer = Observer::new("full-value".into(), CanonicalValue::String("exact".into()));
    let realization = RealizationMetadata::new(
        d(b"semantic-target"),
        "rust-cpu".into(),
        d(b"source"),
        d(b"binary"),
        CanonicalValue::String("u8->bool".into()),
        d(b"validation"),
    );

    let before = [
        judgement.structural_digest(),
        evidence.structural_digest(),
        authority.structural_digest(),
        observer.structural_digest(),
        realization.structural_digest(),
    ];

    assert_eq!(judgement.world(), d(b"world"));
    assert_eq!(evidence.target_judgement(), judgement.structural_digest());
    assert_eq!(evidence.evidence_family(), "fixture");
    assert_eq!(authority.requested_authority_class(), "deterministic-proof");
    assert_eq!(authority.exactness_requirement(), "exact");
    assert_eq!(observer.observer_family(), "full-value");
    assert_eq!(realization.semantic_target(), d(b"semantic-target"));

    let after = [
        judgement.structural_digest(),
        evidence.structural_digest(),
        authority.structural_digest(),
        observer.structural_digest(),
        realization.structural_digest(),
    ];
    assert_eq!(before, after);
}

#[test]
fn promotion_and_realization_manifests_are_structurally_addressable() {
    let candidate = frozen(vec![d(b"semantic")], vec![]).structural_digest();
    let promotion = PromotionManifest::new(
        d(b"parent-generation"),
        candidate,
        vec![d(b"evidence")],
        vec![d(b"semantic")],
        vec![d(b"authority-binding")],
    );
    let realization = RealizationCheckManifest::new(
        d(b"semantic"),
        d(b"realization-metadata"),
        d(b"generation"),
        d(b"world"),
        d(b"authority-contract"),
        d(b"observer"),
        d(b"binary"),
    );

    assert_ne!(
        promotion.structural_digest(),
        realization.structural_digest()
    );
}
