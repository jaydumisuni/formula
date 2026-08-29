use formula_core::{digest::ArtifactDigest, theory::FederationAdapterManifest};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FederationMode {
    CertifiedTranslation,
    CheckedResult,
    CandidateOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederationRequest {
    result_class: String,
    requires_authority: bool,
    checker_route: Option<ArtifactDigest>,
    translation: Option<ArtifactDigest>,
    allow_side_effects: bool,
}

impl FederationRequest {
    pub fn new(
        result_class: String,
        requires_authority: bool,
        checker_route: Option<ArtifactDigest>,
        translation: Option<ArtifactDigest>,
        allow_side_effects: bool,
    ) -> Self {
        Self {
            result_class,
            requires_authority,
            checker_route,
            translation,
            allow_side_effects,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FederationValidation {
    Accepted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FederationError {
    CandidateOnlyCannotAuthorize,
    UnsupportedResultClass,
    CheckerRouteMismatch,
    TranslationMismatch,
    SideEffectsNotAllowed,
}

pub fn validate_federation_adapter(
    adapter: &FederationAdapterManifest,
    mode: FederationMode,
    request: &FederationRequest,
) -> Result<FederationValidation, FederationError> {
    if request.requires_authority && mode == FederationMode::CandidateOnly {
        return Err(FederationError::CandidateOnlyCannotAuthorize);
    }
    if adapter
        .result_classes()
        .binary_search(&request.result_class)
        .is_err()
    {
        return Err(FederationError::UnsupportedResultClass);
    }
    if !request.allow_side_effects && !adapter.side_effects().is_empty() {
        return Err(FederationError::SideEffectsNotAllowed);
    }

    if matches!(
        mode,
        FederationMode::CheckedResult | FederationMode::CertifiedTranslation
    ) {
        let Some(checker_route) = request.checker_route else {
            return Err(FederationError::CheckerRouteMismatch);
        };
        if adapter
            .checker_routes()
            .binary_search(&checker_route)
            .is_err()
        {
            return Err(FederationError::CheckerRouteMismatch);
        }

        let Some(translation) = request.translation else {
            return Err(FederationError::TranslationMismatch);
        };
        if adapter.translations().binary_search(&translation).is_err() {
            return Err(FederationError::TranslationMismatch);
        }
    }

    Ok(FederationValidation::Accepted)
}
