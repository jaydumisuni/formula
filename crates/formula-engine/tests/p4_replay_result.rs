use formula_core::digest::ArtifactDigest;
use formula_engine::{
    obligation::TerminalState, query::ResourceContract, replay::ReplayManifest,
    result_bundle::ResultBundle,
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn replay() -> ReplayManifest {
    ReplayManifest::new(
        d(1),
        d(2),
        d(3),
        d(4),
        d(5),
        d(6),
        "compiler-v1",
        "scheduler-v1",
        ResourceContract::new(100, 1024, 50),
        d(7),
        d(8),
    )
}

#[test]
fn identical_replay_inputs_have_identical_identity() {
    let left = replay();
    let right = replay();
    assert_eq!(left.canonical_bytes(), right.canonical_bytes());
    assert_eq!(left.digest(), right.digest());
}

#[test]
fn every_replay_semantic_and_policy_input_is_identity_binding() {
    let base = replay().digest();
    let variants = [
        replay().with_universe_generation(d(20)).digest(),
        replay().with_world(d(21)).digest(),
        replay().with_query_digest(d(22)).digest(),
        replay().with_activated_package_set(d(23)).digest(),
        replay().with_relevant_region_digest(d(24)).digest(),
        replay().with_theory_profile_digest(d(25)).digest(),
        replay()
            .with_compiler_policy_version("compiler-v2")
            .digest(),
        replay()
            .with_scheduler_policy_version("scheduler-v2")
            .digest(),
        replay()
            .with_resource_contract(ResourceContract::new(200, 1024, 50))
            .digest(),
        replay().with_random_key(d(26)).digest(),
        replay().with_campaign_digest(d(27)).digest(),
    ];

    for variant in variants {
        assert_ne!(base, variant);
    }
}

#[test]
fn result_bundle_is_structural_and_deterministic() {
    let left = ResultBundle::new(
        d(1),
        d(2),
        TerminalState::Satisfied,
        vec![d(3)],
        vec![d(4), d(5)],
        vec![d(6)],
        vec![d(7)],
        vec![d(8)],
        vec![d(9)],
        vec![d(10)],
    );
    let right = ResultBundle::new(
        d(1),
        d(2),
        TerminalState::Satisfied,
        vec![d(3), d(3)],
        vec![d(5), d(4)],
        vec![d(6)],
        vec![d(7)],
        vec![d(8)],
        vec![d(9)],
        vec![d(10)],
    );

    assert_eq!(left.digest(), right.digest());
    assert_eq!(left.evidence_refs(), &[d(4), d(5)]);
    assert_eq!(left.terminal_state(), TerminalState::Satisfied);
}
