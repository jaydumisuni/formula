use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    first_light::{
        FirstLightNativeEvidence, FirstLightProofManifest, FirstLightReuseEvidence,
        FirstLightTargetEvidence, NegativeControlManifest,
    },
};

const PASS_MARKERS: [&str; 15] = [
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
];

pub fn verifier_identity_v1() -> ArtifactDigest {
    ArtifactDigest::of_bytes(b"formula-first-light-verifier-v1")
}

pub fn checker_identity_v1() -> ArtifactDigest {
    ArtifactDigest::of_bytes(b"formula-check-v1")
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FirstLightReplayEvidence {
    pub source_commit: String,
    pub u0_digest: ArtifactDigest,
    pub u1_digest: ArtifactDigest,
    pub u1_parent: ArtifactDigest,
    pub world: ArtifactDigest,
    pub activated_package_set: ArtifactDigest,
    pub fl_a: FirstLightTargetEvidence,
    pub fl_b: FirstLightTargetEvidence,
    pub fl_c: FirstLightTargetEvidence,
    pub promotion_digest: ArtifactDigest,
    pub closure_before: ArtifactDigest,
    pub closure_after: ArtifactDigest,
    pub closure_delta: ArtifactDigest,
    pub native: FirstLightNativeEvidence,
    pub reuse: FirstLightReuseEvidence,
    pub reuse_candidate_spaces: u64,
    pub reuse_discovery_work_cells: u64,
    pub reuse_result_exact: bool,
    pub negative_controls: NegativeControlManifest,
    pub verifier_identity: ArtifactDigest,
    pub checker_identity: ArtifactDigest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FirstLightVerificationError {
    UniverseParentMismatch,
    RediscoveryDetected,
    ReuseResultNotExact,
    NegativeControlsMismatch,
    ManifestEvidenceMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FirstLightVerification {
    _private: (),
}

impl FirstLightVerification {
    pub fn markers(&self) -> &'static [&'static str; 15] {
        &PASS_MARKERS
    }
}

pub fn verify_first_light_manifest_v1(
    manifest: &FirstLightProofManifest,
    replay: &FirstLightReplayEvidence,
) -> Result<FirstLightVerification, FirstLightVerificationError> {
    if replay.u1_parent != replay.u0_digest || replay.u1_digest == replay.u0_digest {
        return Err(FirstLightVerificationError::UniverseParentMismatch);
    }

    if replay.reuse_candidate_spaces != 0 || replay.reuse_discovery_work_cells != 0 {
        return Err(FirstLightVerificationError::RediscoveryDetected);
    }

    if !replay.reuse_result_exact {
        return Err(FirstLightVerificationError::ReuseResultNotExact);
    }

    if !replay.negative_controls.is_complete()
        || replay.negative_controls.structural_digest() != manifest.negative_controls()
    {
        return Err(FirstLightVerificationError::NegativeControlsMismatch);
    }

    if replay.verifier_identity != verifier_identity_v1()
        || replay.checker_identity != checker_identity_v1()
        || manifest.verifier_identity() != verifier_identity_v1()
        || manifest.checker_identity() != checker_identity_v1()
        || replay.source_commit != manifest.source_commit()
        || replay.u0_digest != manifest.u0_digest()
        || replay.u1_digest != manifest.u1_digest()
        || replay.world != manifest.world()
        || replay.activated_package_set != manifest.activated_package_set()
        || &replay.fl_a != manifest.fl_a()
        || &replay.fl_b != manifest.fl_b()
        || &replay.fl_c != manifest.fl_c()
        || replay.promotion_digest != manifest.promotion_digest()
        || replay.closure_before != manifest.closure_before()
        || replay.closure_after != manifest.closure_after()
        || replay.closure_delta != manifest.closure_delta()
        || &replay.native != manifest.native()
        || &replay.reuse != manifest.reuse()
        || replay.native.realization() != replay.reuse.realization()
    {
        return Err(FirstLightVerificationError::ManifestEvidenceMismatch);
    }

    Ok(FirstLightVerification { _private: () })
}
