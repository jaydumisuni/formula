use formula_check::self_expansion_verifier::{
    P10_CANONICAL_MARKERS, SelfExpansionReplayClaims, SelfExpansionReplayEvidence,
    SelfExpansionVerificationFailure, verify_self_expansion_manifest,
};
use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    self_expansion_proof::{
        SelfExpansionNegativeControl, SelfExpansionNegativeControlEvidence,
        SelfExpansionNegativeControlManifest, SelfExpansionProofManifest,
    },
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn controls() -> SelfExpansionNegativeControlManifest {
    SelfExpansionNegativeControlManifest::new(
        SelfExpansionNegativeControl::ALL
            .into_iter()
            .map(|control| {
                SelfExpansionNegativeControlEvidence::new(
                    control,
                    d(&format!("p10:nc:{}", control.as_str())),
                )
            })
            .collect(),
    )
    .unwrap()
}

fn manifest() -> SelfExpansionProofManifest {
    SelfExpansionProofManifest::new(
        "p10-source-sha".into(),
        d("p9:frozen"),
        d("p10:u:g"),
        d("p10:u:g1"),
        d("p10:world"),
        d("p10:registry"),
        d("p10:rational-package"),
        d("p10:rational-package"),
        d("p10:closure:before"),
        d("p10:closure:after"),
        d("p10:closure:delta"),
        d("cap:rational:field"),
        d("p10:structure-witness"),
        d("p10:base-promotion"),
        d("p10:expansion-authorization"),
        d("p10:lambda:g"),
        d("p10:lambda:g1"),
        d("p10:nogood-proof"),
        d("p10:route-proof"),
        d("p10:shadow-metaprimitive"),
        d("p10:semantic-change"),
        d("p10:proof-evolution"),
        d("p10:realization-upgrade"),
        d("p10:realization-rollback"),
        controls(),
        d("p10:checker"),
        d("p10:verifier"),
    )
}

fn replay(manifest: &SelfExpansionProofManifest) -> SelfExpansionReplayEvidence {
    SelfExpansionReplayEvidence::new(
        manifest.structural_digest(),
        manifest.source_generation(),
        manifest.expanded_generation(),
        manifest.source_generation(),
        manifest.registry_digest(),
        manifest.rational_package_before(),
        manifest.rational_package_after(),
        manifest.closure_before(),
        manifest.closure_after(),
        manifest.closure_delta(),
        manifest.unlocked_capability(),
        manifest.structure_witness(),
        manifest.base_promotion(),
        manifest.expansion_authorization(),
        manifest.lambda_before(),
        manifest.lambda_after(),
        manifest.nogood_proof(),
        manifest.route_proof(),
        manifest.shadow_metaprimitive(),
        manifest.semantic_change(),
        manifest.proof_evolution(),
        manifest.realization_upgrade(),
        manifest.realization_rollback(),
        manifest.negative_controls().clone(),
        manifest.checker_identity(),
        manifest.verifier_identity(),
        SelfExpansionReplayClaims::all_proved(),
    )
}

#[test]
fn complete_manifest_emits_exact_p10_marker_order() {
    let manifest = manifest();
    let evidence = replay(&manifest);
    let result = verify_self_expansion_manifest(&manifest, &evidence).unwrap();
    assert_eq!(result.markers(), &P10_CANONICAL_MARKERS);
}

#[test]
fn missing_negative_control_or_changed_package_digest_rejects_complete_claim() {
    let manifest = manifest();
    let mut incomplete = SelfExpansionNegativeControl::ALL
        .into_iter()
        .take(11)
        .map(|control| {
            SelfExpansionNegativeControlEvidence::new(
                control,
                d(&format!("p10:nc:{}", control.as_str())),
            )
        })
        .collect::<Vec<_>>();
    assert!(SelfExpansionNegativeControlManifest::new(incomplete.clone()).is_err());
    incomplete.push(SelfExpansionNegativeControlEvidence::new(
        SelfExpansionNegativeControl::WrongBasePromotion,
        d("p10:duplicate"),
    ));
    assert!(SelfExpansionNegativeControlManifest::new(incomplete).is_err());

    let mut evidence = replay(&manifest);
    evidence.set_rational_package_after_for_test(d("p10:mutated-package"));
    assert_eq!(
        verify_self_expansion_manifest(&manifest, &evidence),
        Err(SelfExpansionVerificationFailure::ReplayBindingMismatch)
    );
}

#[test]
fn verifier_rejects_unproved_hardening_claim_before_markers() {
    let manifest = manifest();
    let mut evidence = replay(&manifest);
    evidence.set_claims_for_test(SelfExpansionReplayClaims::none_proved());
    assert_eq!(
        verify_self_expansion_manifest(&manifest, &evidence),
        Err(SelfExpansionVerificationFailure::HardeningClaimNotProved)
    );
}
