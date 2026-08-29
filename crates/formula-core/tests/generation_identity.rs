use formula_core::{digest::ArtifactDigest, generation::UniverseGeneration};

fn d(value: &[u8]) -> ArtifactDigest {
    ArtifactDigest::of_bytes(value)
}

#[test]
fn generation_identity_is_order_independent_for_set_like_members() {
    let a = UniverseGeneration::new(
        1,
        Some(d(b"u0")),
        vec![d(b"b"), d(b"a"), d(b"a")],
        vec![d(b"e2"), d(b"e1"), d(b"e1")],
    );
    let b = UniverseGeneration::new(
        1,
        Some(d(b"u0")),
        vec![d(b"a"), d(b"b")],
        vec![d(b"e1"), d(b"e2")],
    );
    assert_eq!(a.canonical_bytes(), b.canonical_bytes());
    assert_eq!(a.digest(), b.digest());
}

#[test]
fn parent_or_authority_delta_changes_generation_root() {
    let base = UniverseGeneration::new(
        1,
        Some(d(b"u0")),
        vec![d(b"a")],
        vec![d(b"e1")],
    );
    let changed_parent = UniverseGeneration::new(
        1,
        Some(d(b"other")),
        vec![d(b"a")],
        vec![d(b"e1")],
    );
    let changed_authority = UniverseGeneration::new(
        1,
        Some(d(b"u0")),
        vec![d(b"a")],
        vec![d(b"e2")],
    );
    assert_ne!(base.digest(), changed_parent.digest());
    assert_ne!(base.digest(), changed_authority.digest());
}

#[test]
fn generation_identity_has_no_machine_metadata_surface() {
    let generation = UniverseGeneration::new(0, None, vec![d(b"a")], vec![]);
    let bytes = generation.canonical_bytes();
    for forbidden in ["/tmp/host-A", "2026-08-29T05:00:00Z", "pid=99"] {
        assert!(!bytes.windows(forbidden.len()).any(|w| w == forbidden.as_bytes()));
    }
}
