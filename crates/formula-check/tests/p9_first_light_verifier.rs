use formula_check::first_light::{
    FirstLightReplayEvidence, FirstLightVerificationError, checker_identity_v1,
    verifier_identity_v1, verify_first_light_manifest_v1,
};
use formula_core::{
    digest::ArtifactDigest,
    first_light::{
        FirstLightNativeEvidence, FirstLightProofManifest, FirstLightReuseEvidence,
        FirstLightTargetEvidence, NegativeControlEvidence, NegativeControlId,
        NegativeControlManifest,
    },
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn controls() -> NegativeControlManifest {
    NegativeControlManifest::complete(
        NegativeControlId::ALL
            .iter()
            .copied()
            .map(|id| NegativeControlEvidence::new(id, d(id.as_str())))
            .collect(),
    )
    .unwrap()
}

fn target(name: &str) -> FirstLightTargetEvidence {
    FirstLightTargetEvidence::new(
        d(&format!("{name}-query")),
        d(&format!("{name}-campaign")),
        d(&format!("{name}-candidate")),
        d(&format!("{name}-certification")),
        vec![d(&format!("{name}-aux"))],
    )
}

fn native() -> FirstLightNativeEvidence {
    FirstLightNativeEvidence::new(
        d("native-source"),
        d("native-toolchain"),
        d("native-binary"),
        d("native-realization"),
    )
}

fn reuse() -> FirstLightReuseEvidence {
    FirstLightReuseEvidence::new(
        d("reuse-query"),
        d("reuse-campaign"),
        d("reuse-capability"),
        d("reuse-plan"),
        d("reuse-result"),
        d("reuse-metrics"),
        d("native-realization"),
    )
}

fn manifest() -> FirstLightProofManifest {
    FirstLightProofManifest::new(
        "canonical-source".into(),
        d("u0"),
        d("u1"),
        d("world"),
        d("packages"),
        target("fl-a"),
        target("fl-b"),
        target("fl-c"),
        d("promotion"),
        d("closure-before"),
        d("closure-after"),
        d("closure-delta"),
        native(),
        reuse(),
        controls(),
        verifier_identity_v1(),
        checker_identity_v1(),
    )
}

fn replay() -> FirstLightReplayEvidence {
    FirstLightReplayEvidence {
        source_commit: "canonical-source".into(),
        u0_digest: d("u0"),
        u1_digest: d("u1"),
        u1_parent: d("u0"),
        world: d("world"),
        activated_package_set: d("packages"),
        fl_a: target("fl-a"),
        fl_b: target("fl-b"),
        fl_c: target("fl-c"),
        promotion_digest: d("promotion"),
        closure_before: d("closure-before"),
        closure_after: d("closure-after"),
        closure_delta: d("closure-delta"),
        native: native(),
        reuse: reuse(),
        reuse_candidate_spaces: 0,
        reuse_discovery_work_cells: 0,
        reuse_result_exact: true,
        negative_controls: controls(),
        verifier_identity: verifier_identity_v1(),
        checker_identity: checker_identity_v1(),
    }
}

#[test]
fn exact_replay_produces_only_the_frozen_ordered_pass_markers() {
    let verification = verify_first_light_manifest_v1(&manifest(), &replay()).unwrap();
    assert_eq!(
        verification.markers(),
        &[
            "PASS D1_AUTHORITY_SEPARATION",
            "PASS D2_IDENTITY_GENERATION_REPLAY",
            "PASS D2_CERTIFICATE_ROUTING",
            "PASS D2_SEARCH_STATE_SEPARATION",
            "PASS D3_BLIND_SEMANTIC_ELABORATION",
            "PASS D3_REPRESENTATION_REDUCTION",
            "PASS D3_SYMBOLIC_CANDIDATE_SPACE",
            "PASS D3_FALSE_NEARMISS_REJECTION",
            "PASS D4_NATIVE_REALIZATION_EQUIVALENCE",
            "PASS D4_CPU_LOCAL_OFFLINE",
            "PASS D5_ATOMIC_PROMOTION",
            "PASS D5_CAPABILITY_CLOSURE_EXPANDED",
            "PASS D5_SECOND_QUERY_REUSE",
            "PASS NEGATIVE_CONTROLS",
            "PASS FIRST_LIGHT_COMPLETE",
        ]
    );
    assert_eq!(verification.markers().len(), 15);
    assert_eq!(verification.markers().last(), Some(&"PASS FIRST_LIGHT_COMPLETE"));
}

#[test]
fn verifier_fails_closed_on_parent_rediscovery_or_inexact_reuse() {
    let mut wrong_parent = replay();
    wrong_parent.u1_parent = d("not-u0");
    assert_eq!(
        verify_first_light_manifest_v1(&manifest(), &wrong_parent).unwrap_err(),
        FirstLightVerificationError::UniverseParentMismatch
    );

    let mut rediscovered = replay();
    rediscovered.reuse_candidate_spaces = 1;
    assert_eq!(
        verify_first_light_manifest_v1(&manifest(), &rediscovered).unwrap_err(),
        FirstLightVerificationError::RediscoveryDetected
    );

    let mut inexact = replay();
    inexact.reuse_result_exact = false;
    assert_eq!(
        verify_first_light_manifest_v1(&manifest(), &inexact).unwrap_err(),
        FirstLightVerificationError::ReuseResultNotExact
    );
}

#[test]
fn verifier_fails_closed_on_any_manifest_or_control_identity_mismatch() {
    let mut changed = replay();
    changed.closure_after = d("changed-closure-after");
    assert_eq!(
        verify_first_light_manifest_v1(&manifest(), &changed).unwrap_err(),
        FirstLightVerificationError::ManifestEvidenceMismatch
    );

    let mut changed_control = replay();
    changed_control.negative_controls = NegativeControlManifest::complete(
        NegativeControlId::ALL
            .iter()
            .copied()
            .map(|id| {
                let evidence = if id == NegativeControlId::ForgedEvidence {
                    d("different-control-evidence")
                } else {
                    d(id.as_str())
                };
                NegativeControlEvidence::new(id, evidence)
            })
            .collect(),
    )
    .unwrap();
    assert_eq!(
        verify_first_light_manifest_v1(&manifest(), &changed_control).unwrap_err(),
        FirstLightVerificationError::NegativeControlsMismatch
    );
}
