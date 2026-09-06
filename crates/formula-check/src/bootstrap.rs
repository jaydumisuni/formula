use formula_core::{
    artifacts::StructuralIdentity,
    bootstrap::{
        BOOTSTRAP_CORE_SCHEMA_V1, BootstrapBytecode, BootstrapDecision, BootstrapEquivalenceLevel,
        BootstrapGenerationId, BootstrapInstruction, BootstrapProgramSource,
        BootstrapRebuildManifest, BootstrapSeedManifest, BootstrapValidationState,
    },
    digest::ArtifactDigest,
};

const BOOTSTRAP_BUILD_RECIPE_V1: &[u8] = b"formula-bootstrap-build-recipe:v1:FBC1";
const BOOTSTRAP_NORMALIZATION_NONE_V1: &[u8] = b"formula-bootstrap-normalization:none:v1";
const BOOTSTRAP_SEMANTIC_EVIDENCE_V1: &[u8] = b"formula-bootstrap-semantic-evidence:v1";
const BOOTSTRAP_SEMANTIC_CASES_V1: &[u8] = b"equal:VALID;different:REJECT";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootstrapValidationFailure {
    UnsupportedSource,
    GeneratorEqualsValidator,
    SeedProvenanceMismatch,
    SourceDigestMismatch,
    BuildRecipeDigestMismatch,
    CandidateArtifactBindingMismatch,
    IndependentArtifactMismatch,
    CandidateReferenceMismatch,
    NormalizationRulesMismatch,
    EquivalenceNotByteForByte,
    InvalidValidationState,
    SemanticEvidenceMismatch,
    SemanticEquivalenceFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BootstrapValidationAuthorization {
    predecessor: BootstrapGenerationId,
    successor: BootstrapGenerationId,
    rebuild_manifest: ArtifactDigest,
    seed_identity: ArtifactDigest,
    source_digest: ArtifactDigest,
    candidate_artifact: ArtifactDigest,
    generator_identity: ArtifactDigest,
    validator_identity: ArtifactDigest,
    semantic_evidence: ArtifactDigest,
}

impl BootstrapValidationAuthorization {
    pub fn predecessor(&self) -> BootstrapGenerationId {
        self.predecessor
    }

    pub fn successor(&self) -> BootstrapGenerationId {
        self.successor
    }

    pub fn rebuild_manifest(&self) -> ArtifactDigest {
        self.rebuild_manifest
    }

    pub fn seed_identity(&self) -> ArtifactDigest {
        self.seed_identity
    }

    pub fn source_digest(&self) -> ArtifactDigest {
        self.source_digest
    }

    pub fn candidate_artifact(&self) -> ArtifactDigest {
        self.candidate_artifact
    }

    pub fn generator_identity(&self) -> ArtifactDigest {
        self.generator_identity
    }

    pub fn validator_identity(&self) -> ArtifactDigest {
        self.validator_identity
    }

    pub fn semantic_evidence(&self) -> ArtifactDigest {
        self.semantic_evidence
    }
}

pub fn canonical_build_recipe_identity() -> ArtifactDigest {
    ArtifactDigest::of_bytes(BOOTSTRAP_BUILD_RECIPE_V1)
}

pub fn canonical_normalization_rules_identity() -> ArtifactDigest {
    ArtifactDigest::of_bytes(BOOTSTRAP_NORMALIZATION_NONE_V1)
}

pub fn semantic_evidence_identity(
    source: &BootstrapProgramSource,
    candidate: &BootstrapBytecode,
) -> ArtifactDigest {
    let mut bytes = BOOTSTRAP_SEMANTIC_EVIDENCE_V1.to_vec();
    bytes.extend_from_slice(source.structural_digest().as_str().as_bytes());
    bytes.extend_from_slice(candidate.structural_digest().as_str().as_bytes());
    bytes.extend_from_slice(BOOTSTRAP_SEMANTIC_CASES_V1);
    ArtifactDigest::of_bytes(&bytes)
}

pub fn reference_compile(
    source: &BootstrapProgramSource,
) -> Result<BootstrapBytecode, BootstrapValidationFailure> {
    validate_source(source)?;
    Ok(BootstrapBytecode::new(vec![
        b'F', b'B', b'C', b'1', 0x01, 0x02, 0x03, 0x04,
    ]))
}

pub fn reference_execute(
    source: &BootstrapProgramSource,
    actual: ArtifactDigest,
    expected: ArtifactDigest,
) -> Result<BootstrapDecision, BootstrapValidationFailure> {
    validate_source(source)?;
    Ok(if actual == expected {
        BootstrapDecision::Valid
    } else {
        BootstrapDecision::Reject
    })
}

pub fn validate_bootstrap_candidate(
    rebuild: &BootstrapRebuildManifest,
    source: &BootstrapProgramSource,
    candidate: &BootstrapBytecode,
    seed: &BootstrapSeedManifest,
) -> Result<BootstrapValidationAuthorization, BootstrapValidationFailure> {
    if rebuild.generator_identity() == rebuild.validator_identity() {
        return Err(BootstrapValidationFailure::GeneratorEqualsValidator);
    }
    if rebuild.seed_identity() != seed.structural_digest() {
        return Err(BootstrapValidationFailure::SeedProvenanceMismatch);
    }
    if rebuild.source_digest() != source.structural_digest() {
        return Err(BootstrapValidationFailure::SourceDigestMismatch);
    }
    if rebuild.build_recipe_digest() != canonical_build_recipe_identity() {
        return Err(BootstrapValidationFailure::BuildRecipeDigestMismatch);
    }
    if rebuild.candidate_artifact() != candidate.structural_digest() {
        return Err(BootstrapValidationFailure::CandidateArtifactBindingMismatch);
    }
    if rebuild.normalization_rules() != canonical_normalization_rules_identity() {
        return Err(BootstrapValidationFailure::NormalizationRulesMismatch);
    }
    if rebuild.equivalence() != BootstrapEquivalenceLevel::ByteForByte {
        return Err(BootstrapValidationFailure::EquivalenceNotByteForByte);
    }
    if rebuild.state() != BootstrapValidationState::Candidate {
        return Err(BootstrapValidationFailure::InvalidValidationState);
    }
    if rebuild.semantic_evidence() != semantic_evidence_identity(source, candidate) {
        return Err(BootstrapValidationFailure::SemanticEvidenceMismatch);
    }

    let independently_compiled = reference_compile(source)?;
    if rebuild.independent_artifact() != independently_compiled.structural_digest() {
        return Err(BootstrapValidationFailure::IndependentArtifactMismatch);
    }
    if candidate != &independently_compiled {
        return Err(BootstrapValidationFailure::CandidateReferenceMismatch);
    }

    let equal = ArtifactDigest::of_bytes(b"bootstrap-equal-case");
    if reference_execute(source, equal, equal)? != BootstrapDecision::Valid
        || reference_execute(
            source,
            ArtifactDigest::of_bytes(b"bootstrap-actual"),
            ArtifactDigest::of_bytes(b"bootstrap-expected"),
        )? != BootstrapDecision::Reject
    {
        return Err(BootstrapValidationFailure::SemanticEquivalenceFailed);
    }

    Ok(BootstrapValidationAuthorization {
        predecessor: rebuild.predecessor(),
        successor: rebuild.successor(),
        rebuild_manifest: rebuild.structural_digest(),
        seed_identity: seed.structural_digest(),
        source_digest: source.structural_digest(),
        candidate_artifact: candidate.structural_digest(),
        generator_identity: rebuild.generator_identity(),
        validator_identity: rebuild.validator_identity(),
        semantic_evidence: rebuild.semantic_evidence(),
    })
}

fn validate_source(source: &BootstrapProgramSource) -> Result<(), BootstrapValidationFailure> {
    if source.schema() != BOOTSTRAP_CORE_SCHEMA_V1
        || source.instructions()
            != [
                BootstrapInstruction::LoadActualDigest,
                BootstrapInstruction::LoadExpectedDigest,
                BootstrapInstruction::DigestEq,
                BootstrapInstruction::ReturnDecision,
            ]
    {
        return Err(BootstrapValidationFailure::UnsupportedSource);
    }
    Ok(())
}
