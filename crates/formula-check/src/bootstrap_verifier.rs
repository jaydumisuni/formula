use crate::bootstrap::{reference_compile, reference_execute, validate_bootstrap_candidate};
use formula_core::{
    artifacts::StructuralIdentity,
    bootstrap::{
        BOOTSTRAP_CORE_SCHEMA_V1, BootstrapBytecode, BootstrapDecision, BootstrapEquivalenceLevel,
        BootstrapGenerationId, BootstrapGeneratorImage, BootstrapInstruction,
        BootstrapNegativeControlEvidence, BootstrapNegativeControlManifest, BootstrapProgramSource,
        BootstrapProofManifest, BootstrapRebuildManifest, BootstrapSeedManifest,
    },
    digest::ArtifactDigest,
};

pub const P12_CANONICAL_MARKERS: [&str; 10] = [
    "PASS P12_B0_SEED_EXPLICIT",
    "PASS P12_BOOTSTRAP_CORE_DETERMINISTIC",
    "PASS P12_GENERATOR_VALIDATOR_DIVERSE",
    "PASS P12_STAGE1_INDEPENDENTLY_VALIDATED",
    "PASS P12_STAGE2_SELF_REBUILD_VALIDATED",
    "PASS P12_BYTE_EQUIVALENCE",
    "PASS P12_SEMANTIC_EQUIVALENCE",
    "PASS P12_UNIVERSE_AUTHORITY_UNCHANGED",
    "PASS P12_NEGATIVE_CONTROLS",
    "PASS BOOTSTRAP_TRUST_REDUCED",
];

const P11_FROZEN_PROOF_HEAD: &[u8] = b"6f8ce7bb6702ea1baf119aab9950aa5ba0f87283";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BootstrapReplayEvidence {
    manifest_digest: ArtifactDigest,
    source_commit: String,
    predecessor_p11: ArtifactDigest,
    seed: BootstrapSeedManifest,
    t0: BootstrapGenerationId,
    t1: BootstrapGenerationId,
    t2: BootstrapGenerationId,
    program_source: BootstrapProgramSource,
    stage1: BootstrapRebuildManifest,
    stage2: BootstrapRebuildManifest,
    stage1_artifact: BootstrapBytecode,
    stage2_artifact: BootstrapBytecode,
    negative_controls: Vec<BootstrapNegativeControlEvidence>,
    universe_before: ArtifactDigest,
    universe_after: ArtifactDigest,
    checker_identity: ArtifactDigest,
    verifier_identity: ArtifactDigest,
}

impl BootstrapReplayEvidence {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        manifest_digest: ArtifactDigest,
        source_commit: String,
        predecessor_p11: ArtifactDigest,
        seed: BootstrapSeedManifest,
        t0: BootstrapGenerationId,
        t1: BootstrapGenerationId,
        t2: BootstrapGenerationId,
        program_source: BootstrapProgramSource,
        stage1: BootstrapRebuildManifest,
        stage2: BootstrapRebuildManifest,
        stage1_artifact: BootstrapBytecode,
        stage2_artifact: BootstrapBytecode,
        negative_controls: Vec<BootstrapNegativeControlEvidence>,
        universe_before: ArtifactDigest,
        universe_after: ArtifactDigest,
        checker_identity: ArtifactDigest,
        verifier_identity: ArtifactDigest,
    ) -> Self {
        Self {
            manifest_digest,
            source_commit,
            predecessor_p11,
            seed,
            t0,
            t1,
            t2,
            program_source,
            stage1,
            stage2,
            stage1_artifact,
            stage2_artifact,
            negative_controls,
            universe_before,
            universe_after,
            checker_identity,
            verifier_identity,
        }
    }

    #[doc(hidden)]
    pub fn set_manifest_digest_for_test(&mut self, value: ArtifactDigest) {
        self.manifest_digest = value;
    }

    #[doc(hidden)]
    pub fn set_negative_controls_for_test(&mut self, value: Vec<BootstrapNegativeControlEvidence>) {
        self.negative_controls = value;
    }

    #[doc(hidden)]
    pub fn set_stage2_generator_identity_for_test(&mut self, value: ArtifactDigest) {
        self.stage2 = BootstrapRebuildManifest::new(
            self.stage2.predecessor(),
            self.stage2.successor(),
            value,
            self.stage2.validator_identity(),
            self.stage2.source_digest(),
            self.stage2.build_recipe_digest(),
            self.stage2.candidate_artifact(),
            self.stage2.independent_artifact(),
            self.stage2.normalization_rules(),
            self.stage2.equivalence(),
            self.stage2.semantic_evidence(),
            self.stage2.seed_identity(),
            self.stage2.state(),
        );
    }

    #[doc(hidden)]
    pub fn set_t2_for_test(&mut self, value: BootstrapGenerationId) {
        self.t2 = value;
    }

    #[doc(hidden)]
    pub fn set_stage2_artifact_for_test(&mut self, value: BootstrapBytecode) {
        self.stage2_artifact = value;
    }

    #[doc(hidden)]
    pub fn set_universe_after_for_test(&mut self, value: ArtifactDigest) {
        self.universe_after = value;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootstrapProofFailure {
    ReplayBindingMismatch,
    NegativeControlsIncomplete,
    GeneratorValidatorNotDiverse,
    BootstrapStageMismatch,
    ArtifactMismatch,
    IndependentValidationFailed,
    Stage2SelfRebuildNotProved,
    ByteEquivalenceNotProved,
    SemanticEquivalenceNotProved,
    UniverseAuthorityChanged,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VerifiedBootstrapProof;

impl VerifiedBootstrapProof {
    pub fn markers(&self) -> &[&'static str; 10] {
        &P12_CANONICAL_MARKERS
    }
}

fn expected_checker_identity() -> ArtifactDigest {
    ArtifactDigest::of_bytes(b"formula-check:p12-bootstrap-independent-validator:v1")
}

fn expected_verifier_identity() -> ArtifactDigest {
    ArtifactDigest::of_bytes(b"formula-check:p12-bootstrap-final-replay:v1")
}

fn expected_p11_identity() -> ArtifactDigest {
    ArtifactDigest::of_bytes(P11_FROZEN_PROOF_HEAD)
}

fn canonical_stage0_image() -> BootstrapGeneratorImage {
    BootstrapGeneratorImage::new(
        BOOTSTRAP_CORE_SCHEMA_V1.into(),
        b"FBC1".to_vec(),
        vec![
            (BootstrapInstruction::LoadActualDigest, 0x01),
            (BootstrapInstruction::LoadExpectedDigest, 0x02),
            (BootstrapInstruction::DigestEq, 0x03),
            (BootstrapInstruction::ReturnDecision, 0x04),
        ],
    )
}

fn generator_from_admitted_artifact(
    source: &BootstrapProgramSource,
    artifact: &BootstrapBytecode,
) -> Option<BootstrapGeneratorImage> {
    if source.schema() != BOOTSTRAP_CORE_SCHEMA_V1
        || source.instructions()
            != [
                BootstrapInstruction::LoadActualDigest,
                BootstrapInstruction::LoadExpectedDigest,
                BootstrapInstruction::DigestEq,
                BootstrapInstruction::ReturnDecision,
            ]
        || artifact.bytes() != b"FBC1\x01\x02\x03\x04"
    {
        return None;
    }

    Some(BootstrapGeneratorImage::new(
        source.schema().into(),
        artifact.bytes()[..4].to_vec(),
        source
            .instructions()
            .iter()
            .copied()
            .zip(artifact.bytes()[4..].iter().copied())
            .collect(),
    ))
}

fn expected_successor_generation(
    predecessor: BootstrapGenerationId,
    generator: ArtifactDigest,
    validator: ArtifactDigest,
    source: ArtifactDigest,
    candidate: ArtifactDigest,
    seed: ArtifactDigest,
) -> Option<BootstrapGenerationId> {
    let ordinal = predecessor.ordinal().checked_add(1)?;
    let mut bytes = b"formula-bootstrap-successor:v1".to_vec();
    bytes.extend_from_slice(&predecessor.ordinal().to_be_bytes());
    for digest in [
        predecessor.digest(),
        generator,
        validator,
        source,
        candidate,
        seed,
    ] {
        bytes.push(0);
        bytes.extend_from_slice(digest.as_str().as_bytes());
    }
    Some(BootstrapGenerationId::new(
        ordinal,
        ArtifactDigest::of_bytes(&bytes),
    ))
}

pub fn verify_bootstrap_proof_manifest(
    manifest: &BootstrapProofManifest,
    replay: &BootstrapReplayEvidence,
) -> Result<VerifiedBootstrapProof, BootstrapProofFailure> {
    let controls = BootstrapNegativeControlManifest::new(replay.negative_controls.clone())
        .map_err(|_| BootstrapProofFailure::NegativeControlsIncomplete)?;
    if !controls.is_complete() {
        return Err(BootstrapProofFailure::NegativeControlsIncomplete);
    }

    if replay.stage1.generator_identity() == replay.stage1.validator_identity()
        || replay.stage2.generator_identity() == replay.stage2.validator_identity()
    {
        return Err(BootstrapProofFailure::GeneratorValidatorNotDiverse);
    }

    if replay.stage1_artifact.structural_digest() != replay.stage1.candidate_artifact()
        || replay.stage2_artifact.structural_digest() != replay.stage2.candidate_artifact()
        || replay.stage1_artifact.structural_digest() != manifest.stage1_artifact()
        || replay.stage2_artifact.structural_digest() != manifest.stage2_artifact()
    {
        return Err(BootstrapProofFailure::ArtifactMismatch);
    }

    if replay.universe_before != replay.universe_after {
        return Err(BootstrapProofFailure::UniverseAuthorityChanged);
    }

    if replay.t0.ordinal() != 0
        || replay.t0.digest() != replay.seed.structural_digest()
        || replay.stage1.predecessor() != replay.t0
        || replay.stage1.successor() != replay.t1
        || replay.stage2.predecessor() != replay.t1
        || replay.stage2.successor() != replay.t2
    {
        return Err(BootstrapProofFailure::BootstrapStageMismatch);
    }

    let expected_t1 = expected_successor_generation(
        replay.t0,
        replay.stage1.generator_identity(),
        replay.stage1.validator_identity(),
        replay.program_source.structural_digest(),
        replay.stage1_artifact.structural_digest(),
        replay.seed.structural_digest(),
    )
    .ok_or(BootstrapProofFailure::BootstrapStageMismatch)?;
    let expected_t2 = expected_successor_generation(
        replay.t1,
        replay.stage2.generator_identity(),
        replay.stage2.validator_identity(),
        replay.program_source.structural_digest(),
        replay.stage2_artifact.structural_digest(),
        replay.seed.structural_digest(),
    )
    .ok_or(BootstrapProofFailure::BootstrapStageMismatch)?;
    if replay.t1 != expected_t1 || replay.t2 != expected_t2 {
        return Err(BootstrapProofFailure::BootstrapStageMismatch);
    }

    if manifest.structural_digest() != replay.manifest_digest
        || manifest.source_commit() != replay.source_commit
        || manifest.predecessor_p11() != replay.predecessor_p11
        || manifest.seed() != &replay.seed
        || manifest.t0() != replay.t0
        || manifest.t1() != replay.t1
        || manifest.t2() != replay.t2
        || manifest.program_source() != replay.program_source.structural_digest()
        || manifest.stage1() != &replay.stage1
        || manifest.stage2() != &replay.stage2
        || manifest.negative_controls() != &controls
        || manifest.universe_before() != replay.universe_before
        || manifest.universe_after() != replay.universe_after
        || manifest.checker_identity() != replay.checker_identity
        || manifest.verifier_identity() != replay.verifier_identity
        || replay.predecessor_p11 != expected_p11_identity()
        || replay.checker_identity != expected_checker_identity()
        || replay.verifier_identity != expected_verifier_identity()
    {
        return Err(BootstrapProofFailure::ReplayBindingMismatch);
    }

    let stage0 = canonical_stage0_image();
    if replay.stage1.generator_identity() != stage0.structural_digest() {
        return Err(BootstrapProofFailure::ReplayBindingMismatch);
    }

    validate_bootstrap_candidate(
        &replay.stage1,
        &replay.program_source,
        &replay.stage1_artifact,
        &replay.seed,
    )
    .map_err(|_| BootstrapProofFailure::IndependentValidationFailed)?;

    let stage2_generator = generator_from_admitted_artifact(
        &replay.program_source,
        &replay.stage1_artifact,
    )
    .ok_or(BootstrapProofFailure::Stage2SelfRebuildNotProved)?;
    if replay.stage2.generator_identity() != stage2_generator.structural_digest() {
        return Err(BootstrapProofFailure::Stage2SelfRebuildNotProved);
    }

    validate_bootstrap_candidate(
        &replay.stage2,
        &replay.program_source,
        &replay.stage2_artifact,
        &replay.seed,
    )
    .map_err(|_| BootstrapProofFailure::IndependentValidationFailed)?;

    let independently_compiled = reference_compile(&replay.program_source)
        .map_err(|_| BootstrapProofFailure::ByteEquivalenceNotProved)?;
    if replay.stage1.equivalence() != BootstrapEquivalenceLevel::ByteForByte
        || replay.stage2.equivalence() != BootstrapEquivalenceLevel::ByteForByte
        || replay.stage1_artifact != independently_compiled
        || replay.stage2_artifact != independently_compiled
        || replay.stage1_artifact != replay.stage2_artifact
    {
        return Err(BootstrapProofFailure::ByteEquivalenceNotProved);
    }

    let same = ArtifactDigest::of_bytes(b"p12-verifier-same");
    let actual = ArtifactDigest::of_bytes(b"p12-verifier-actual");
    let expected = ArtifactDigest::of_bytes(b"p12-verifier-expected");
    if reference_execute(&replay.program_source, same, same) != Ok(BootstrapDecision::Valid)
        || reference_execute(&replay.program_source, actual, expected)
            != Ok(BootstrapDecision::Reject)
    {
        return Err(BootstrapProofFailure::SemanticEquivalenceNotProved);
    }

    Ok(VerifiedBootstrapProof)
}
