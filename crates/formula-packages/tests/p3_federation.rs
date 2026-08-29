use formula_core::{digest::ArtifactDigest, theory::FederationAdapterManifest};
use formula_packages::federation::{
    validate_federation_adapter, FederationError, FederationMode, FederationRequest,
    FederationValidation,
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn adapter(side_effects: Vec<String>) -> FederationAdapterManifest {
    FederationAdapterManifest::new(
        "adapter.v1".into(),
        d("package"),
        vec![d("input")],
        vec![d("output")],
        vec![d("translation")],
        vec![d("checker-route")],
        side_effects,
        vec!["EXACT_WITNESS".into()],
        true,
    )
}

#[test]
fn candidate_only_adapter_cannot_create_authority() {
    let request = FederationRequest::new(
        "EXACT_WITNESS".into(),
        true,
        Some(d("checker-route")),
        Some(d("translation")),
        false,
    );
    assert_eq!(
        validate_federation_adapter(&adapter(vec![]), FederationMode::CandidateOnly, &request),
        Err(FederationError::CandidateOnlyCannotAuthorize)
    );
}

#[test]
fn checked_modes_require_exact_declared_route_translation_and_result_class() {
    let good = FederationRequest::new(
        "EXACT_WITNESS".into(),
        true,
        Some(d("checker-route")),
        Some(d("translation")),
        false,
    );
    assert_eq!(
        validate_federation_adapter(&adapter(vec![]), FederationMode::CheckedResult, &good),
        Ok(FederationValidation::Accepted)
    );

    let wrong_route = FederationRequest::new(
        "EXACT_WITNESS".into(),
        true,
        Some(d("other-checker")),
        Some(d("translation")),
        false,
    );
    assert_eq!(
        validate_federation_adapter(&adapter(vec![]), FederationMode::CheckedResult, &wrong_route),
        Err(FederationError::CheckerRouteMismatch)
    );

    let wrong_translation = FederationRequest::new(
        "EXACT_WITNESS".into(),
        true,
        Some(d("checker-route")),
        Some(d("other-translation")),
        false,
    );
    assert_eq!(
        validate_federation_adapter(
            &adapter(vec![]),
            FederationMode::CertifiedTranslation,
            &wrong_translation,
        ),
        Err(FederationError::TranslationMismatch)
    );

    let wrong_result = FederationRequest::new(
        "UNDECLARED".into(),
        true,
        Some(d("checker-route")),
        Some(d("translation")),
        false,
    );
    assert_eq!(
        validate_federation_adapter(&adapter(vec![]), FederationMode::CheckedResult, &wrong_result),
        Err(FederationError::UnsupportedResultClass)
    );
}

#[test]
fn undeclared_side_effect_permission_fails_closed() {
    let request = FederationRequest::new(
        "EXACT_WITNESS".into(),
        true,
        Some(d("checker-route")),
        Some(d("translation")),
        false,
    );
    assert_eq!(
        validate_federation_adapter(
            &adapter(vec!["filesystem-write".into()]),
            FederationMode::CheckedResult,
            &request,
        ),
        Err(FederationError::SideEffectsNotAllowed)
    );
}
