use crate::{
    u8::BoolExpr,
    verdict::{CheckFailure, CheckVerdict},
};
use formula_core::{certification::RealizationCheckManifest, digest::ArtifactDigest};

pub struct RealizationCheckRequest<'a> {
    manifest: &'a RealizationCheckManifest,
    semantic_target: ArtifactDigest,
    realization: ArtifactDigest,
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    authority_contract: ArtifactDigest,
    observer: ArtifactDigest,
    artifact_bytes: &'a [u8],
    semantic: &'a BoolExpr,
    realized_outputs: &'a [bool],
}

impl<'a> RealizationCheckRequest<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        manifest: &'a RealizationCheckManifest,
        semantic_target: ArtifactDigest,
        realization: ArtifactDigest,
        universe_generation: ArtifactDigest,
        world: ArtifactDigest,
        authority_contract: ArtifactDigest,
        observer: ArtifactDigest,
        artifact_bytes: &'a [u8],
        semantic: &'a BoolExpr,
        realized_outputs: &'a [bool],
    ) -> Self {
        Self {
            manifest,
            semantic_target,
            realization,
            universe_generation,
            world,
            authority_contract,
            observer,
            artifact_bytes,
            semantic,
            realized_outputs,
        }
    }
}

pub fn check_u8_realization_equivalence(request: &RealizationCheckRequest<'_>) -> CheckVerdict {
    let manifest = request.manifest;

    if manifest.semantic_target() != request.semantic_target
        || manifest.realization() != request.realization
        || manifest.universe_generation() != request.universe_generation
        || manifest.world() != request.world
        || manifest.authority_contract() != request.authority_contract
        || manifest.observer() != request.observer
    {
        return CheckVerdict::Fail(CheckFailure::RealizationBindingMismatch);
    }

    if ArtifactDigest::of_bytes(request.artifact_bytes) != manifest.realization_artifact_digest() {
        return CheckVerdict::Fail(CheckFailure::RealizationArtifactDigestMismatch);
    }

    if request.realized_outputs.len() != 256 {
        return CheckVerdict::Fail(CheckFailure::RealizationOutputCoverageMismatch);
    }

    for raw in 0u16..=255 {
        let input = raw as u8;
        if request.realized_outputs[raw as usize] != request.semantic.evaluate(input) {
            return CheckVerdict::Fail(CheckFailure::RealizationCounterexample(input));
        }
    }

    CheckVerdict::Pass
}
