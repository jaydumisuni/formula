use formula_core::{
    artifacts::StructuralIdentity,
    canonical::CanonicalValue,
    digest::ArtifactDigest,
    federation::BridgeContract,
    theory::{
        CompositionClaim, CompositionClass, FactPolarity, FederationAdapterManifest, SharedFact,
    },
};
use formula_packages::{
    cooperation::{CooperationError, apply_bridge, certify_federation_fact},
    federation::{FederationError, FederationMode, FederationRequest},
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn adapter(package: ArtifactDigest) -> FederationAdapterManifest {
    FederationAdapterManifest::new(
        "sat-lrat-v1".into(),
        package,
        vec![d("semantic-input")],
        vec![d("source-subject")],
        vec![d("translation")],
        vec![d("checker-route")],
        vec![],
        vec!["EXACT_RESULT".into()],
        true,
    )
}

fn request(checker: ArtifactDigest, translation: ArtifactDigest) -> FederationRequest {
    FederationRequest::new(
        "EXACT_RESULT".into(),
        true,
        Some(checker),
        Some(translation),
        false,
    )
}

fn fact(polarity: FactPolarity, evidence: ArtifactDigest) -> SharedFact {
    SharedFact::new(
        d("world"),
        d("source-subject"),
        CanonicalValue::String("branch-a".into()),
        polarity,
        evidence,
    )
}

fn certified_exact() -> formula_core::federation::CertifiedFederationFact {
    let package = d("sat-package");
    let adapter = adapter(package);
    let evidence = d("checked-evidence");
    certify_federation_fact(
        &adapter,
        FederationMode::CheckedResult,
        &request(d("checker-route"), d("translation")),
        adapter.structural_digest(),
        d("semantic-input"),
        evidence,
        fact(FactPolarity::Exact, evidence),
    )
    .expect("checked fact must certify")
}

#[test]
fn checked_adapter_admits_provenance_bound_fact() {
    let certified = certified_exact();
    assert_eq!(certified.package(), d("sat-package"));
    assert_eq!(certified.semantic_input(), d("semantic-input"));
    assert_eq!(certified.checker_route(), d("checker-route"));
    assert_eq!(certified.translation(), d("translation"));
    assert_eq!(certified.evidence(), d("checked-evidence"));
}

#[test]
fn candidate_only_cannot_be_promoted_by_matching_producer_identity() {
    let package = d("trusted-looking-producer-package");
    let adapter = adapter(package);
    let evidence = d("checked-evidence");
    assert_eq!(
        certify_federation_fact(
            &adapter,
            FederationMode::CandidateOnly,
            &request(d("checker-route"), d("translation")),
            adapter.structural_digest(),
            d("semantic-input"),
            evidence,
            fact(FactPolarity::Exact, evidence),
        ),
        Err(CooperationError::CandidateOnlyCannotCertify)
    );
}

#[test]
fn wrong_checker_route_and_translation_fail_closed() {
    let package = d("sat-package");
    let adapter = adapter(package);
    let evidence = d("checked-evidence");

    assert_eq!(
        certify_federation_fact(
            &adapter,
            FederationMode::CheckedResult,
            &request(d("wrong-checker"), d("translation")),
            adapter.structural_digest(),
            d("semantic-input"),
            evidence,
            fact(FactPolarity::Exact, evidence),
        ),
        Err(CooperationError::Adapter(
            FederationError::CheckerRouteMismatch
        ))
    );

    assert_eq!(
        certify_federation_fact(
            &adapter,
            FederationMode::CheckedResult,
            &request(d("checker-route"), d("wrong-translation")),
            adapter.structural_digest(),
            d("semantic-input"),
            evidence,
            fact(FactPolarity::Exact, evidence),
        ),
        Err(CooperationError::Adapter(
            FederationError::TranslationMismatch
        ))
    );
}

#[test]
fn adapter_digest_semantic_input_output_and_evidence_are_exact_bindings() {
    let package = d("sat-package");
    let adapter = adapter(package);
    let evidence = d("checked-evidence");
    let request = request(d("checker-route"), d("translation"));

    assert_eq!(
        certify_federation_fact(
            &adapter,
            FederationMode::CheckedResult,
            &request,
            d("wrong-adapter-digest"),
            d("semantic-input"),
            evidence,
            fact(FactPolarity::Exact, evidence),
        ),
        Err(CooperationError::AdapterDigestMismatch)
    );

    assert_eq!(
        certify_federation_fact(
            &adapter,
            FederationMode::CheckedResult,
            &request,
            adapter.structural_digest(),
            d("stale-semantic-input"),
            evidence,
            fact(FactPolarity::Exact, evidence),
        ),
        Err(CooperationError::SemanticInputMismatch)
    );

    let wrong_subject = SharedFact::new(
        d("world"),
        d("wrong-output-subject"),
        CanonicalValue::String("branch-a".into()),
        FactPolarity::Exact,
        evidence,
    );
    assert_eq!(
        certify_federation_fact(
            &adapter,
            FederationMode::CheckedResult,
            &request,
            adapter.structural_digest(),
            d("semantic-input"),
            evidence,
            wrong_subject,
        ),
        Err(CooperationError::SemanticOutputMismatch)
    );

    assert_eq!(
        certify_federation_fact(
            &adapter,
            FederationMode::CheckedResult,
            &request,
            adapter.structural_digest(),
            d("semantic-input"),
            evidence,
            fact(FactPolarity::Exact, d("other-evidence")),
        ),
        Err(CooperationError::EvidenceMismatch)
    );
}

#[test]
fn directional_certified_bridge_translates_exact_fact() {
    let source = certified_exact();
    let bridge = BridgeContract::new(
        d("sat-package"),
        d("arithmetic-package"),
        d("source-subject"),
        d("target-subject"),
        FactPolarity::Exact,
        FactPolarity::Exact,
        d("bridge-translation"),
        d("bridge-evidence"),
    );
    let composition = CompositionClaim::new(
        d("sat-package"),
        d("arithmetic-package"),
        CompositionClass::CertifiedCombination,
        d("composition-evidence"),
    );

    let bridged = apply_bridge(&source, Some(&bridge), &composition).expect("safe bridge");
    assert_eq!(bridged.world(), d("world"));
    assert_eq!(bridged.subject(), d("target-subject"));
    assert_eq!(bridged.polarity(), FactPolarity::Exact);
    assert_eq!(bridged.payload(), source.fact().payload());
    assert_ne!(bridged.evidence(), source.fact().evidence());
}

#[test]
fn bridge_is_mandatory_and_directional() {
    let source = certified_exact();
    let composition = CompositionClaim::new(
        d("sat-package"),
        d("arithmetic-package"),
        CompositionClass::SoundCooperation,
        d("composition-evidence"),
    );

    assert_eq!(
        apply_bridge(&source, None, &composition),
        Err(CooperationError::MissingBridgeContract)
    );

    let reverse = BridgeContract::new(
        d("arithmetic-package"),
        d("sat-package"),
        d("target-subject"),
        d("source-subject"),
        FactPolarity::Exact,
        FactPolarity::Exact,
        d("bridge-translation"),
        d("bridge-evidence"),
    );
    assert_eq!(
        apply_bridge(&source, Some(&reverse), &composition),
        Err(CooperationError::BridgeSourcePackageMismatch)
    );
}

#[test]
fn bridge_cannot_strengthen_fact_polarity() {
    let package = d("sat-package");
    let adapter = adapter(package);
    let evidence = d("checked-evidence");
    let source = certify_federation_fact(
        &adapter,
        FederationMode::CheckedResult,
        &request(d("checker-route"), d("translation")),
        adapter.structural_digest(),
        d("semantic-input"),
        evidence,
        fact(FactPolarity::OverApproximation, evidence),
    )
    .expect("checked over-approximation remains certified as over-approximation");
    let bridge = BridgeContract::new(
        d("sat-package"),
        d("arithmetic-package"),
        d("source-subject"),
        d("target-subject"),
        FactPolarity::OverApproximation,
        FactPolarity::Exact,
        d("bridge-translation"),
        d("bridge-evidence"),
    );
    let composition = CompositionClaim::new(
        d("sat-package"),
        d("arithmetic-package"),
        CompositionClass::CertifiedCombination,
        d("composition-evidence"),
    );

    assert_eq!(
        apply_bridge(&source, Some(&bridge), &composition),
        Err(CooperationError::PolarityUpgrade)
    );
}

#[test]
fn heuristic_or_unsupported_composition_cannot_authorize_bridge() {
    let source = certified_exact();
    let bridge = BridgeContract::new(
        d("sat-package"),
        d("arithmetic-package"),
        d("source-subject"),
        d("target-subject"),
        FactPolarity::Exact,
        FactPolarity::Exact,
        d("bridge-translation"),
        d("bridge-evidence"),
    );

    for class in [
        CompositionClass::HeuristicOnly,
        CompositionClass::Unsupported,
        CompositionClass::Quarantined,
    ] {
        let composition = CompositionClaim::new(
            d("sat-package"),
            d("arithmetic-package"),
            class,
            d("composition-evidence"),
        );
        assert_eq!(
            apply_bridge(&source, Some(&bridge), &composition),
            Err(CooperationError::UnsafeCompositionClass)
        );
    }
}
