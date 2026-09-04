use formula_core::{
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    promotion::{PromotionRecord, PromotionState},
    theory::ClosureContext,
};
use formula_packages::{
    activation::validate_activation,
    closure::{
        CapabilityClosureDelta, ClosureError, derive_capabilities_with_semantic_activations,
    },
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn contexts(
    primitive: ArtifactDigest,
    evidence: ArtifactDigest,
) -> (
    UniverseGeneration,
    UniverseGeneration,
    ClosureContext,
    ClosureContext,
) {
    let u0 = UniverseGeneration::new(0, None, vec![], vec![]);
    let u1 = UniverseGeneration::new(1, Some(u0.digest()), vec![primitive], vec![evidence]);
    let world = d("p9-closure-world");
    let rules = d("p9-closure-rules");
    let policy = d("p9-closure-policy");
    let c0 = ClosureContext::new(u0.digest(), world, vec![], rules, policy);
    let c1 = ClosureContext::new(u1.digest(), world, vec![], rules, policy);
    (u0, u1, c0, c1)
}

#[test]
fn activated_u1_semantic_primitive_expands_capability_closure() {
    let primitive = d("p9-reusable-primitive");
    let evidence = d("p9-reusable-evidence");
    let (u0, u1, c0, c1) = contexts(primitive, evidence);
    let packages0 = validate_activation(&u0, &[], &[], &[]).unwrap();
    let packages1 = validate_activation(&u1, &[], &[], &[]).unwrap();
    let activation = PromotionRecord::new(
        d("p9-promotion-candidate"),
        PromotionState::Activated,
        u1.digest(),
        d("p9-promotion-policy"),
        vec![evidence],
        vec![primitive],
    );

    let before =
        derive_capabilities_with_semantic_activations(&c0, &packages0, &[], &[], &u0, &[]).unwrap();
    let after = derive_capabilities_with_semantic_activations(
        &c1,
        &packages1,
        &[],
        &[],
        &u1,
        &[activation],
    )
    .unwrap();

    assert!(!before.contains(primitive));
    assert!(after.contains(primitive));
    let delta = CapabilityClosureDelta::between(&before, &after);
    assert_eq!(delta.added().collect::<Vec<_>>(), vec![primitive]);
    assert!(delta.removed().next().is_none());
}

#[test]
fn admitted_but_not_activated_primitive_does_not_become_reusable() {
    let primitive = d("p9-admitted-only-primitive");
    let evidence = d("p9-admitted-only-evidence");
    let (_, u1, _, c1) = contexts(primitive, evidence);
    let packages1 = validate_activation(&u1, &[], &[], &[]).unwrap();
    let admitted = PromotionRecord::new(
        d("p9-admitted-only-candidate"),
        PromotionState::Admitted,
        u1.digest(),
        d("p9-promotion-policy"),
        vec![evidence],
        vec![primitive],
    );

    let error =
        derive_capabilities_with_semantic_activations(&c1, &packages1, &[], &[], &u1, &[admitted])
            .unwrap_err();
    assert_eq!(error, ClosureError::SemanticActivationStateMismatch);
}

#[test]
fn wrong_generation_unadmitted_primitive_or_unbound_evidence_fails_closed() {
    let primitive = d("p9-invalid-primitive");
    let evidence = d("p9-invalid-evidence");
    let (u0, u1, _, c1) = contexts(primitive, evidence);
    let packages1 = validate_activation(&u1, &[], &[], &[]).unwrap();

    let wrong_generation = PromotionRecord::new(
        d("p9-wrong-generation-candidate"),
        PromotionState::Activated,
        u0.digest(),
        d("p9-promotion-policy"),
        vec![],
        vec![],
    );
    assert_eq!(
        derive_capabilities_with_semantic_activations(
            &c1,
            &packages1,
            &[],
            &[],
            &u1,
            &[wrong_generation],
        )
        .unwrap_err(),
        ClosureError::SemanticActivationGenerationMismatch
    );

    let not_admitted = d("p9-not-admitted-capability");
    let unadmitted = PromotionRecord::new(
        d("p9-unadmitted-candidate"),
        PromotionState::Activated,
        u1.digest(),
        d("p9-promotion-policy"),
        vec![evidence],
        vec![not_admitted],
    );
    assert_eq!(
        derive_capabilities_with_semantic_activations(
            &c1,
            &packages1,
            &[],
            &[],
            &u1,
            &[unadmitted],
        )
        .unwrap_err(),
        ClosureError::SemanticActivationPrimitiveNotAdmitted
    );

    let unbound_evidence = d("p9-unbound-closure-evidence");
    let unbound = PromotionRecord::new(
        d("p9-unbound-candidate"),
        PromotionState::Activated,
        u1.digest(),
        d("p9-promotion-policy"),
        vec![unbound_evidence],
        vec![primitive],
    );
    assert_eq!(
        derive_capabilities_with_semantic_activations(&c1, &packages1, &[], &[], &u1, &[unbound],)
            .unwrap_err(),
        ClosureError::SemanticActivationEvidenceNotAuthorityBound
    );
}
