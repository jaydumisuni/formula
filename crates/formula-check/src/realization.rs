use crate::{
    u8::BoolExpr,
    verdict::{CheckFailure, CheckVerdict},
};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::RealizationCheckManifest,
    digest::ArtifactDigest,
    realization::{NativeRealizationManifest, NativeToolchainIdentity, SpecializationIdentity},
};

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

pub type RealizationPolicyFailure = CheckFailure;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealizationAuthorization {
    realization_manifest: ArtifactDigest,
    semantic_target: ArtifactDigest,
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    authority_contract: ArtifactDigest,
    observer: ArtifactDigest,
    specialization_digest: ArtifactDigest,
    source_digest: ArtifactDigest,
    toolchain_digest: ArtifactDigest,
    binary_digest: ArtifactDigest,
}

impl RealizationAuthorization {
    pub fn realization_manifest(&self) -> ArtifactDigest {
        self.realization_manifest
    }

    pub fn semantic_target(&self) -> ArtifactDigest {
        self.semantic_target
    }

    pub fn universe_generation(&self) -> ArtifactDigest {
        self.universe_generation
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn authority_contract(&self) -> ArtifactDigest {
        self.authority_contract
    }

    pub fn observer(&self) -> ArtifactDigest {
        self.observer
    }

    pub fn specialization_digest(&self) -> ArtifactDigest {
        self.specialization_digest
    }

    pub fn source_digest(&self) -> ArtifactDigest {
        self.source_digest
    }

    pub fn toolchain_digest(&self) -> ArtifactDigest {
        self.toolchain_digest
    }

    pub fn binary_digest(&self) -> ArtifactDigest {
        self.binary_digest
    }
}

#[allow(clippy::too_many_arguments)]
pub fn authorize_native_u8_realization_v1(
    native_manifest: &NativeRealizationManifest,
    specialization: &SpecializationIdentity,
    toolchain: &NativeToolchainIdentity,
    check_manifest: &RealizationCheckManifest,
    source_bytes: &[u8],
    binary_bytes: &[u8],
    semantic: &BoolExpr,
    realized_outputs: &[bool],
) -> Result<RealizationAuthorization, RealizationPolicyFailure> {
    let realization_manifest = native_manifest.structural_digest();

    if native_manifest.semantic_target() != specialization.semantic_target()
        || native_manifest.universe_generation() != specialization.universe_generation()
        || native_manifest.world() != specialization.world()
        || native_manifest.authority_contract() != specialization.authority_contract()
        || native_manifest.observer() != specialization.observer()
        || native_manifest.specialization_digest() != specialization.structural_digest()
        || native_manifest.toolchain_digest() != toolchain.structural_digest()
        || check_manifest.semantic_target() != native_manifest.semantic_target()
        || check_manifest.realization() != realization_manifest
        || check_manifest.universe_generation() != native_manifest.universe_generation()
        || check_manifest.world() != native_manifest.world()
        || check_manifest.authority_contract() != native_manifest.authority_contract()
        || check_manifest.observer() != native_manifest.observer()
        || check_manifest.realization_artifact_digest() != native_manifest.binary_digest()
    {
        return Err(CheckFailure::RealizationNativeBindingMismatch);
    }

    if ArtifactDigest::of_bytes(source_bytes) != native_manifest.source_digest() {
        return Err(CheckFailure::RealizationSourceDigestMismatch);
    }

    let request = RealizationCheckRequest::new(
        check_manifest,
        native_manifest.semantic_target(),
        realization_manifest,
        native_manifest.universe_generation(),
        native_manifest.world(),
        native_manifest.authority_contract(),
        native_manifest.observer(),
        binary_bytes,
        semantic,
        realized_outputs,
    );

    match check_u8_realization_equivalence(&request) {
        CheckVerdict::Pass => Ok(RealizationAuthorization {
            realization_manifest,
            semantic_target: native_manifest.semantic_target(),
            universe_generation: native_manifest.universe_generation(),
            world: native_manifest.world(),
            authority_contract: native_manifest.authority_contract(),
            observer: native_manifest.observer(),
            specialization_digest: native_manifest.specialization_digest(),
            source_digest: native_manifest.source_digest(),
            toolchain_digest: native_manifest.toolchain_digest(),
            binary_digest: native_manifest.binary_digest(),
        }),
        CheckVerdict::Fail(failure) => Err(failure),
    }
}
