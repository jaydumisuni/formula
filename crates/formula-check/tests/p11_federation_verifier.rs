use formula_check::federation_verifier::{
    FederationReplayClaims, FederationReplayEvidence, FederationVerificationFailure,
    P11_CANONICAL_MARKERS, verify_federation_breadth_manifest,
};
use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    federation_proof::{
        FederationBreadthProofManifest, FederationNegativeControl,
        FederationNegativeControlEvidence, FederationNegativeControlManifest, FederationRouteKind,
        FederationRouteProof,
    },
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn route(kind: FederationRouteKind, prefix: &str) -> FederationRouteProof {
    FederationRouteProof::new(
        kind,
        d(&format!("{prefix}:package")),
        d(&format!("{prefix}:adapter")),
        d(&format!("{prefix}:input")),
        d(&format!("{prefix}:evidence")),
        d(&format!("{prefix}:fact")),
    )
}

fn controls() -> FederationNegativeControlManifest {
    FederationNegativeControlManifest::new(
        FederationNegativeControl::ALL
            .into_iter()
            .map(|control| {
                FederationNegativeControlEvidence::new(
                    control,
                    d(&format!("negative:{}", control.as_str())),
                )
            })
            .collect(),
    )
    .expect("complete controls")
}

fn proof() -> (FederationBreadthProofManifest, FederationReplayEvidence) {
    let sat = route(FederationRouteKind::SatLrat, "sat");
    let arithmetic = route(FederationRouteKind::ExactArithmetic, "arith");
    let negative_controls = controls();
    let checker = d("checker");
    let verifier = d("verifier");
    let manifest = FederationBreadthProofManifest::new(
        "source-sha".into(),
        d("p10"),
        d("world"),
        sat.clone(),
        arithmetic.clone(),
        d("bridge"),
        d("composition"),
        d("bridged-fact"),
        d("final-target"),
        negative_controls.clone(),
        checker,
        verifier,
    );
    let evidence = FederationReplayEvidence::new(
        manifest.structural_digest(),
        d("p10"),
        d("world"),
        sat,
        arithmetic.clone(),
        d("bridge"),
        d("composition"),
        d("bridged-fact"),
        d("final-target"),
        arithmetic.certified_fact(),
        d("sat:package"),
        d("arith:package"),
        negative_controls,
        checker,
        verifier,
        FederationReplayClaims::all_proved(),
    );
    (manifest, evidence)
}

#[test]
fn independent_replay_emits_exact_p11_marker_contract() {
    let (manifest, evidence) = proof();
    let result = verify_federation_breadth_manifest(&manifest, &evidence).expect("valid replay");
    assert_eq!(result.markers(), &P11_CANONICAL_MARKERS);
    assert_eq!(
        P11_CANONICAL_MARKERS,
        [
            "PASS P11_SAT_LRAT_CHECKED",
            "PASS P11_EXACT_ARITHMETIC_CHECKED",
            "PASS P11_FEDERATION_PROVENANCE_BOUND",
            "PASS P11_SHARED_FACT_POLARITY_PRESERVED",
            "PASS P11_BRIDGE_CONTRACT_ENFORCED",
            "PASS P11_HETEROGENEOUS_COOPERATION",
            "PASS P11_PRODUCER_IDENTITY_UNTRUSTED",
            "PASS P11_NEGATIVE_CONTROLS",
            "PASS FEDERATION_BREADTH_PROVED",
        ]
    );
}

#[test]
fn mutated_replay_binding_is_rejected() {
    let (manifest, mut evidence) = proof();
    evidence.set_arithmetic_evidence_for_test(d("mutated-arithmetic-evidence"));
    assert_eq!(
        verify_federation_breadth_manifest(&manifest, &evidence),
        Err(FederationVerificationFailure::ReplayBindingMismatch)
    );
}

#[test]
fn cooperation_requires_heterogeneous_packages_and_complete_claims() {
    let (manifest, mut evidence) = proof();
    evidence.set_bridge_target_package_for_test(d("sat:package"));
    assert_eq!(
        verify_federation_breadth_manifest(&manifest, &evidence),
        Err(FederationVerificationFailure::HeterogeneousCooperationNotProved)
    );

    let (manifest, mut evidence) = proof();
    evidence.set_claims_for_test(FederationReplayClaims::none_proved());
    assert_eq!(
        verify_federation_breadth_manifest(&manifest, &evidence),
        Err(FederationVerificationFailure::FederationClaimNotProved)
    );
}
