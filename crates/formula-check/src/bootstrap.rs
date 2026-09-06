use formula_core::{
    artifacts::StructuralIdentity,
    bootstrap::{
        BOOTSTRAP_CORE_SCHEMA_V1, BootstrapBytecode, BootstrapDecision,
        BootstrapEquivalenceLevel, BootstrapGenerationId, BootstrapInstruction,
        BootstrapProgramSource, BootstrapRebuildManifest, BootstrapSeedManifest,
        BootstrapValidationState,
    },
    digest::ArtifactDigest,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootstrapValidationFailure {
    UnsupportedSource,
    GeneratorEqualsValidator,
    SeedProvenanceMismatch,
    SourceDigestMismatch,
    CandidateArtifactBindingMismatch,
    IndependentArtifactMismatch,
    CandidateReferenceMismatch,
    EquivalenceNotByteForByte,
    InvalidValidationState,
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
    if rebuild.candidate_artifact() != candidate.structural_digest() {
        return Err(BootstrapValidationFailure::CandidateArtifactBindingMismatch);
    }
    if rebuild.equivalence() != BootstrapEquivalenceLevel::ByteForByte {
        return Err(BootstrapValidationFailure::EquivalenceNotByteForByte);
    }
    if rebuild.state() != BootstrapValidationState::Candidate {
        return Err(BootstrapValidationFailure::InvalidValidationState);
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
