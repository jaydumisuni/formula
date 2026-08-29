use formula_core::{
    artifacts::{
        AuthorityContract, Entity, EvidenceEnvelope, Judgement, Observer, RealizationMetadata,
        Relation, StructuralIdentity, World,
    },
    canonical::CanonicalValue,
    digest::ArtifactDigest,
};
use std::collections::BTreeMap;

fn d(label: &[u8]) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label)
}

#[test]
fn entity_structural_identity_is_stable_and_reference_sensitive() {
    let structure = CanonicalValue::Object(BTreeMap::from([
        (
            "kind".into(),
            CanonicalValue::String("integer-literal".into()),
        ),
        ("value".into(), CanonicalValue::Integer(17.into())),
    ]));
    let a = Entity::new(d(b"foundation"), structure.clone(), vec![d(b"parent")]);
    let b = Entity::new(d(b"foundation"), structure, vec![d(b"parent")]);
    assert_eq!(a.structural_digest(), b.structural_digest());

    let changed = Entity::new(
        d(b"foundation"),
        CanonicalValue::Integer(17.into()),
        vec![d(b"different-parent")],
    );
    assert_ne!(a.structural_digest(), changed.structural_digest());
}

#[test]
fn semantic_equivalence_does_not_alias_structural_digest() {
    let x_plus_x = Entity::new(
        d(b"foundation"),
        CanonicalValue::String("x+x".into()),
        vec![],
    );
    let two_x = Entity::new(
        d(b"foundation"),
        CanonicalValue::String("2*x".into()),
        vec![],
    );
    assert_ne!(x_plus_x.structural_digest(), two_x.structural_digest());

    let equivalence = Judgement::new(
        d(b"world"),
        CanonicalValue::Object(BTreeMap::from([
            (
                "kind".into(),
                CanonicalValue::String("Equivalent".into()),
            ),
            (
                "left".into(),
                CanonicalValue::Digest(x_plus_x.structural_digest()),
            ),
            (
                "right".into(),
                CanonicalValue::Digest(two_x.structural_digest()),
            ),
        ])),
        vec![x_plus_x.structural_digest(), two_x.structural_digest()],
    );
    assert_ne!(equivalence.structural_digest(), x_plus_x.structural_digest());
    assert_ne!(equivalence.structural_digest(), two_x.structural_digest());
}

#[test]
fn machine_local_metadata_cannot_enter_entity_identity() {
    let entity = Entity::new(
        d(b"foundation"),
        CanonicalValue::Integer(17.into()),
        vec![],
    );
    let bytes = entity.canonical_value().to_canonical_bytes();
    for forbidden in [
        "/tmp/machine-A/17",
        "2026-08-29T04:00:00Z",
        "pid=4242",
        "nonce=machine-random",
    ] {
        assert!(
            !bytes.windows(forbidden.len()).any(|w| w == forbidden.as_bytes()),
            "non-semantic metadata leaked into structural bytes: {forbidden}"
        );
    }
}

#[test]
fn all_p1_schema_families_are_structurally_addressable() {
    let entity = Entity::new(d(b"foundation"), CanonicalValue::Integer(1.into()), vec![]);
    let relation = Relation::new(
        d(b"foundation"),
        2,
        CanonicalValue::String("R(x,y)".into()),
        vec![entity.structural_digest()],
    );
    let world = World::new(
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        d(b"foundation"),
    );
    let judgement = Judgement::new(
        world.structural_digest(),
        CanonicalValue::String("R(a,b)".into()),
        vec![relation.structural_digest()],
    );
    let evidence = EvidenceEnvelope::new(
        judgement.structural_digest(),
        world.structural_digest(),
        CanonicalValue::String("global".into()),
        "fixture".into(),
        d(b"certificate-body"),
        d(b"producer"),
        d(b"checker"),
        d(b"trust-root"),
        "PROVED".into(),
        vec![relation.structural_digest()],
        CanonicalValue::String("replay-v1".into()),
    );
    let realization = RealizationMetadata::new(
        relation.structural_digest(),
        "rust-cpu".into(),
        d(b"source"),
        d(b"binary"),
        CanonicalValue::String("integer->integer".into()),
        evidence.structural_digest(),
    );
    let authority = AuthorityContract::new(
        "deterministic-proof".into(),
        vec!["fixture".into()],
        "exact".into(),
    );
    let observer = Observer::new(
        "witness".into(),
        CanonicalValue::String("full-value".into()),
    );

    let digests = [
        entity.structural_digest(),
        relation.structural_digest(),
        world.structural_digest(),
        judgement.structural_digest(),
        evidence.structural_digest(),
        realization.structural_digest(),
        authority.structural_digest(),
        observer.structural_digest(),
    ];
    assert_eq!(digests.len(), 8);
    assert!(digests.windows(2).all(|pair| pair[0] != pair[1]));
}
