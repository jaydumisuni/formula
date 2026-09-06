use crate::{artifacts::StructuralIdentity, canonical::CanonicalValue, digest::ArtifactDigest};
use num_bigint::BigInt;
use std::collections::BTreeMap;

const BOOTSTRAP_SCHEMA_V1: &str = "formula-bootstrap-trust-v1";
pub const BOOTSTRAP_CORE_SCHEMA_V1: &str = "FORMULA_BOOTSTRAP_CORE_V1";

fn object(kind: &str) -> BTreeMap<String, CanonicalValue> {
    BTreeMap::from([
        ("kind".into(), CanonicalValue::String(kind.into())),
        (
            "schema".into(),
            CanonicalValue::String(BOOTSTRAP_SCHEMA_V1.into()),
        ),
    ])
}

fn bytes_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct BootstrapGenerationId {
    ordinal: u64,
    digest: ArtifactDigest,
}

impl BootstrapGenerationId {
    pub fn new(ordinal: u64, digest: ArtifactDigest) -> Self {
        Self { ordinal, digest }
    }

    pub fn ordinal(self) -> u64 {
        self.ordinal
    }

    pub fn digest(self) -> ArtifactDigest {
        self.digest
    }
}

impl StructuralIdentity for BootstrapGenerationId {
    fn canonical_value(&self) -> CanonicalValue {
        let mut value = object("BootstrapGenerationId");
        value.insert(
            "ordinal".into(),
            CanonicalValue::Integer(BigInt::from(self.ordinal)),
        );
        value.insert("digest".into(), CanonicalValue::Digest(self.digest));
        CanonicalValue::Object(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BootstrapRole {
    ExternalToolchainSeed,
    GeneratorImage,
    IdentityChecker,
}

impl BootstrapRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExternalToolchainSeed => "EXTERNAL_TOOLCHAIN_SEED",
            Self::GeneratorImage => "GENERATOR_IMAGE",
            Self::IdentityChecker => "IDENTITY_CHECKER",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BootstrapSeedManifest {
    role: BootstrapRole,
    toolchain_version: String,
    toolchain_commit: String,
    cargo_version: String,
    host: String,
    rustc_executable: ArtifactDigest,
    cargo_executable: ArtifactDigest,
    rust_toolchain_file: ArtifactDigest,
    reproducibility: String,
    provenance: String,
}

impl BootstrapSeedManifest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        role: BootstrapRole,
        toolchain_version: String,
        toolchain_commit: String,
        cargo_version: String,
        host: String,
        rustc_executable: ArtifactDigest,
        cargo_executable: ArtifactDigest,
        rust_toolchain_file: ArtifactDigest,
        reproducibility: String,
        provenance: String,
    ) -> Self {
        Self {
            role,
            toolchain_version,
            toolchain_commit,
            cargo_version,
            host,
            rustc_executable,
            cargo_executable,
            rust_toolchain_file,
            reproducibility,
            provenance,
        }
    }

    pub fn role(&self) -> BootstrapRole {
        self.role
    }

    pub fn toolchain_version(&self) -> &str {
        &self.toolchain_version
    }

    pub fn toolchain_commit(&self) -> &str {
        &self.toolchain_commit
    }

    pub fn cargo_version(&self) -> &str {
        &self.cargo_version
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn rustc_executable(&self) -> ArtifactDigest {
        self.rustc_executable
    }

    pub fn cargo_executable(&self) -> ArtifactDigest {
        self.cargo_executable
    }

    pub fn rust_toolchain_file(&self) -> ArtifactDigest {
        self.rust_toolchain_file
    }

    pub fn reproducibility(&self) -> &str {
        &self.reproducibility
    }

    pub fn provenance(&self) -> &str {
        &self.provenance
    }
}

impl StructuralIdentity for BootstrapSeedManifest {
    fn canonical_value(&self) -> CanonicalValue {
        let mut value = object("BootstrapSeedManifest");
        value.insert(
            "role".into(),
            CanonicalValue::String(self.role.as_str().into()),
        );
        value.insert(
            "toolchain_version".into(),
            CanonicalValue::String(self.toolchain_version.clone()),
        );
        value.insert(
            "toolchain_commit".into(),
            CanonicalValue::String(self.toolchain_commit.clone()),
        );
        value.insert(
            "cargo_version".into(),
            CanonicalValue::String(self.cargo_version.clone()),
        );
        value.insert("host".into(), CanonicalValue::String(self.host.clone()));
        value.insert(
            "rustc_executable".into(),
            CanonicalValue::Digest(self.rustc_executable),
        );
        value.insert(
            "cargo_executable".into(),
            CanonicalValue::Digest(self.cargo_executable),
        );
        value.insert(
            "rust_toolchain_file".into(),
            CanonicalValue::Digest(self.rust_toolchain_file),
        );
        value.insert(
            "reproducibility".into(),
            CanonicalValue::String(self.reproducibility.clone()),
        );
        value.insert(
            "provenance".into(),
            CanonicalValue::String(self.provenance.clone()),
        );
        CanonicalValue::Object(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BootstrapInstruction {
    LoadActualDigest,
    LoadExpectedDigest,
    DigestEq,
    ReturnDecision,
}

impl BootstrapInstruction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LoadActualDigest => "LOAD_ACTUAL_DIGEST",
            Self::LoadExpectedDigest => "LOAD_EXPECTED_DIGEST",
            Self::DigestEq => "DIGEST_EQ",
            Self::ReturnDecision => "RETURN_DECISION",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BootstrapProgramSource {
    schema: String,
    role: BootstrapRole,
    instructions: Vec<BootstrapInstruction>,
}

impl BootstrapProgramSource {
    pub fn new(
        schema: String,
        role: BootstrapRole,
        instructions: Vec<BootstrapInstruction>,
    ) -> Self {
        Self {
            schema,
            role,
            instructions,
        }
    }

    pub fn identity_checker_v1() -> Self {
        Self::new(
            BOOTSTRAP_CORE_SCHEMA_V1.into(),
            BootstrapRole::IdentityChecker,
            vec![
                BootstrapInstruction::LoadActualDigest,
                BootstrapInstruction::LoadExpectedDigest,
                BootstrapInstruction::DigestEq,
                BootstrapInstruction::ReturnDecision,
            ],
        )
    }

    pub fn schema(&self) -> &str {
        &self.schema
    }

    pub fn role(&self) -> BootstrapRole {
        self.role
    }

    pub fn instructions(&self) -> &[BootstrapInstruction] {
        &self.instructions
    }
}

impl StructuralIdentity for BootstrapProgramSource {
    fn canonical_value(&self) -> CanonicalValue {
        let mut value = object("BootstrapProgramSource");
        value.insert(
            "core_schema".into(),
            CanonicalValue::String(self.schema.clone()),
        );
        value.insert(
            "role".into(),
            CanonicalValue::String(self.role.as_str().into()),
        );
        value.insert(
            "instructions".into(),
            CanonicalValue::Array(
                self.instructions
                    .iter()
                    .map(|instruction| CanonicalValue::String(instruction.as_str().into()))
                    .collect(),
            ),
        );
        CanonicalValue::Object(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BootstrapBytecode {
    bytes: Vec<u8>,
}

impl BootstrapBytecode {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl StructuralIdentity for BootstrapBytecode {
    fn canonical_value(&self) -> CanonicalValue {
        let mut value = object("BootstrapBytecode");
        value.insert(
            "bytes_hex".into(),
            CanonicalValue::String(bytes_hex(&self.bytes)),
        );
        CanonicalValue::Object(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BootstrapGeneratorImage {
    schema: String,
    header: Vec<u8>,
    opcode_map: Vec<(BootstrapInstruction, u8)>,
}

impl BootstrapGeneratorImage {
    pub fn new(
        schema: String,
        header: Vec<u8>,
        mut opcode_map: Vec<(BootstrapInstruction, u8)>,
    ) -> Self {
        opcode_map.sort_by_key(|entry| entry.0);
        Self {
            schema,
            header,
            opcode_map,
        }
    }

    pub fn schema(&self) -> &str {
        &self.schema
    }

    pub fn header(&self) -> &[u8] {
        &self.header
    }

    pub fn opcode_for(&self, instruction: BootstrapInstruction) -> Option<u8> {
        self.opcode_map
            .iter()
            .find_map(|(candidate, opcode)| (*candidate == instruction).then_some(*opcode))
    }

    pub fn opcode_map(&self) -> &[(BootstrapInstruction, u8)] {
        &self.opcode_map
    }
}

impl StructuralIdentity for BootstrapGeneratorImage {
    fn canonical_value(&self) -> CanonicalValue {
        let mut value = object("BootstrapGeneratorImage");
        value.insert(
            "core_schema".into(),
            CanonicalValue::String(self.schema.clone()),
        );
        value.insert(
            "header_hex".into(),
            CanonicalValue::String(bytes_hex(&self.header)),
        );
        value.insert(
            "opcode_map".into(),
            CanonicalValue::Array(
                self.opcode_map
                    .iter()
                    .map(|(instruction, opcode)| {
                        CanonicalValue::Array(vec![
                            CanonicalValue::String(instruction.as_str().into()),
                            CanonicalValue::Integer(BigInt::from(*opcode)),
                        ])
                    })
                    .collect(),
            ),
        );
        CanonicalValue::Object(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BootstrapDecision {
    Valid,
    Reject,
}

impl BootstrapDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Valid => "VALID",
            Self::Reject => "REJECT",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BootstrapEquivalenceLevel {
    SourceSemantic,
    NormalizedArtifact,
    ByteForByte,
}

impl BootstrapEquivalenceLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourceSemantic => "SOURCE_SEMANTIC",
            Self::NormalizedArtifact => "NORMALIZED_ARTIFACT",
            Self::ByteForByte => "BYTE_FOR_BYTE",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BootstrapValidationState {
    Candidate,
    IndependentlyValidated,
    DiversityValidated,
    AdmittedBootstrapGeneration,
    Rejected,
}

impl BootstrapValidationState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Candidate => "CANDIDATE",
            Self::IndependentlyValidated => "INDEPENDENTLY_VALIDATED",
            Self::DiversityValidated => "DIVERSITY_VALIDATED",
            Self::AdmittedBootstrapGeneration => "ADMITTED_BOOTSTRAP_GENERATION",
            Self::Rejected => "REJECTED",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BootstrapRebuildManifest {
    predecessor: BootstrapGenerationId,
    successor: BootstrapGenerationId,
    generator_identity: ArtifactDigest,
    validator_identity: ArtifactDigest,
    source_digest: ArtifactDigest,
    build_recipe_digest: ArtifactDigest,
    candidate_artifact: ArtifactDigest,
    independent_artifact: ArtifactDigest,
    normalization_rules: ArtifactDigest,
    equivalence: BootstrapEquivalenceLevel,
    semantic_evidence: ArtifactDigest,
    seed_identity: ArtifactDigest,
    state: BootstrapValidationState,
}

impl BootstrapRebuildManifest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        predecessor: BootstrapGenerationId,
        successor: BootstrapGenerationId,
        generator_identity: ArtifactDigest,
        validator_identity: ArtifactDigest,
        source_digest: ArtifactDigest,
        build_recipe_digest: ArtifactDigest,
        candidate_artifact: ArtifactDigest,
        independent_artifact: ArtifactDigest,
        normalization_rules: ArtifactDigest,
        equivalence: BootstrapEquivalenceLevel,
        semantic_evidence: ArtifactDigest,
        seed_identity: ArtifactDigest,
        state: BootstrapValidationState,
    ) -> Self {
        Self {
            predecessor,
            successor,
            generator_identity,
            validator_identity,
            source_digest,
            build_recipe_digest,
            candidate_artifact,
            independent_artifact,
            normalization_rules,
            equivalence,
            semantic_evidence,
            seed_identity,
            state,
        }
    }

    pub fn predecessor(&self) -> BootstrapGenerationId {
        self.predecessor
    }
    pub fn successor(&self) -> BootstrapGenerationId {
        self.successor
    }
    pub fn generator_identity(&self) -> ArtifactDigest {
        self.generator_identity
    }
    pub fn validator_identity(&self) -> ArtifactDigest {
        self.validator_identity
    }
    pub fn source_digest(&self) -> ArtifactDigest {
        self.source_digest
    }
    pub fn build_recipe_digest(&self) -> ArtifactDigest {
        self.build_recipe_digest
    }
    pub fn candidate_artifact(&self) -> ArtifactDigest {
        self.candidate_artifact
    }
    pub fn independent_artifact(&self) -> ArtifactDigest {
        self.independent_artifact
    }
    pub fn normalization_rules(&self) -> ArtifactDigest {
        self.normalization_rules
    }
    pub fn equivalence(&self) -> BootstrapEquivalenceLevel {
        self.equivalence
    }
    pub fn semantic_evidence(&self) -> ArtifactDigest {
        self.semantic_evidence
    }
    pub fn seed_identity(&self) -> ArtifactDigest {
        self.seed_identity
    }
    pub fn state(&self) -> BootstrapValidationState {
        self.state
    }
}

impl StructuralIdentity for BootstrapRebuildManifest {
    fn canonical_value(&self) -> CanonicalValue {
        let mut value = object("BootstrapRebuildManifest");
        value.insert("predecessor".into(), self.predecessor.canonical_value());
        value.insert("successor".into(), self.successor.canonical_value());
        value.insert(
            "generator_identity".into(),
            CanonicalValue::Digest(self.generator_identity),
        );
        value.insert(
            "validator_identity".into(),
            CanonicalValue::Digest(self.validator_identity),
        );
        value.insert(
            "source_digest".into(),
            CanonicalValue::Digest(self.source_digest),
        );
        value.insert(
            "build_recipe_digest".into(),
            CanonicalValue::Digest(self.build_recipe_digest),
        );
        value.insert(
            "candidate_artifact".into(),
            CanonicalValue::Digest(self.candidate_artifact),
        );
        value.insert(
            "independent_artifact".into(),
            CanonicalValue::Digest(self.independent_artifact),
        );
        value.insert(
            "normalization_rules".into(),
            CanonicalValue::Digest(self.normalization_rules),
        );
        value.insert(
            "equivalence".into(),
            CanonicalValue::String(self.equivalence.as_str().into()),
        );
        value.insert(
            "semantic_evidence".into(),
            CanonicalValue::Digest(self.semantic_evidence),
        );
        value.insert(
            "seed_identity".into(),
            CanonicalValue::Digest(self.seed_identity),
        );
        value.insert(
            "state".into(),
            CanonicalValue::String(self.state.as_str().into()),
        );
        CanonicalValue::Object(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BootstrapNegativeControl {
    GeneratorEqualsValidator,
    SinglePathAdmissionAttempt,
    UnexpectedArtifactDifference,
    SeedProvenanceMismatch,
    SourceDigestMismatch,
    BuildRecipeDigestMismatch,
    NormalizationMasksSemanticDifference,
    MalformedOrUnsupportedBytecode,
    FailedEquivalencePromotionAttempt,
    UniverseMutationAttempt,
}

impl BootstrapNegativeControl {
    pub const ALL: [Self; 10] = [
        Self::GeneratorEqualsValidator,
        Self::SinglePathAdmissionAttempt,
        Self::UnexpectedArtifactDifference,
        Self::SeedProvenanceMismatch,
        Self::SourceDigestMismatch,
        Self::BuildRecipeDigestMismatch,
        Self::NormalizationMasksSemanticDifference,
        Self::MalformedOrUnsupportedBytecode,
        Self::FailedEquivalencePromotionAttempt,
        Self::UniverseMutationAttempt,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::GeneratorEqualsValidator => "NC-BS-01_GENERATOR_EQUALS_VALIDATOR",
            Self::SinglePathAdmissionAttempt => "NC-BS-02_SINGLE_PATH_ADMISSION_ATTEMPT",
            Self::UnexpectedArtifactDifference => "NC-BS-03_UNEXPECTED_ARTIFACT_DIFFERENCE",
            Self::SeedProvenanceMismatch => "NC-BS-04_SEED_PROVENANCE_MISMATCH",
            Self::SourceDigestMismatch => "NC-BS-05_SOURCE_DIGEST_MISMATCH",
            Self::BuildRecipeDigestMismatch => "NC-BS-06_BUILD_RECIPE_DIGEST_MISMATCH",
            Self::NormalizationMasksSemanticDifference => {
                "NC-BS-07_NORMALIZATION_MASKS_SEMANTIC_DIFFERENCE"
            }
            Self::MalformedOrUnsupportedBytecode => "NC-BS-08_MALFORMED_OR_UNSUPPORTED_BYTECODE",
            Self::FailedEquivalencePromotionAttempt => {
                "NC-BS-09_FAILED_EQUIVALENCE_PROMOTION_ATTEMPT"
            }
            Self::UniverseMutationAttempt => "NC-BS-10_UNIVERSE_MUTATION_ATTEMPT",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BootstrapNegativeControlEvidence {
    control: BootstrapNegativeControl,
    evidence: ArtifactDigest,
}

impl BootstrapNegativeControlEvidence {
    pub fn new(control: BootstrapNegativeControl, evidence: ArtifactDigest) -> Self {
        Self { control, evidence }
    }

    pub fn control(&self) -> BootstrapNegativeControl {
        self.control
    }

    pub fn evidence(&self) -> ArtifactDigest {
        self.evidence
    }
}

impl StructuralIdentity for BootstrapNegativeControlEvidence {
    fn canonical_value(&self) -> CanonicalValue {
        let mut value = object("BootstrapNegativeControlEvidence");
        value.insert(
            "control".into(),
            CanonicalValue::String(self.control.as_str().into()),
        );
        value.insert("evidence".into(), CanonicalValue::Digest(self.evidence));
        CanonicalValue::Object(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootstrapNegativeControlManifestError {
    MissingDuplicateOrUnexpectedControl,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BootstrapNegativeControlManifest {
    controls: Vec<BootstrapNegativeControlEvidence>,
}

impl BootstrapNegativeControlManifest {
    pub fn new(
        mut controls: Vec<BootstrapNegativeControlEvidence>,
    ) -> Result<Self, BootstrapNegativeControlManifestError> {
        controls.sort_by_key(BootstrapNegativeControlEvidence::control);
        if controls.len() != BootstrapNegativeControl::ALL.len()
            || controls
                .iter()
                .map(BootstrapNegativeControlEvidence::control)
                .ne(BootstrapNegativeControl::ALL)
        {
            return Err(BootstrapNegativeControlManifestError::MissingDuplicateOrUnexpectedControl);
        }
        Ok(Self { controls })
    }

    pub fn controls(&self) -> &[BootstrapNegativeControlEvidence] {
        &self.controls
    }

    pub fn is_complete(&self) -> bool {
        self.controls.len() == BootstrapNegativeControl::ALL.len()
            && self
                .controls
                .iter()
                .map(BootstrapNegativeControlEvidence::control)
                .eq(BootstrapNegativeControl::ALL)
    }
}

impl StructuralIdentity for BootstrapNegativeControlManifest {
    fn canonical_value(&self) -> CanonicalValue {
        let mut value = object("BootstrapNegativeControlManifest");
        value.insert(
            "controls".into(),
            CanonicalValue::Array(
                self.controls
                    .iter()
                    .map(StructuralIdentity::canonical_value)
                    .collect(),
            ),
        );
        CanonicalValue::Object(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BootstrapProofManifest {
    source_commit: String,
    predecessor_p11: ArtifactDigest,
    seed: BootstrapSeedManifest,
    t0: BootstrapGenerationId,
    t1: BootstrapGenerationId,
    t2: BootstrapGenerationId,
    program_source: ArtifactDigest,
    stage1: BootstrapRebuildManifest,
    stage2: BootstrapRebuildManifest,
    stage1_artifact: ArtifactDigest,
    stage2_artifact: ArtifactDigest,
    negative_controls: BootstrapNegativeControlManifest,
    universe_before: ArtifactDigest,
    universe_after: ArtifactDigest,
    checker_identity: ArtifactDigest,
    verifier_identity: ArtifactDigest,
}

impl BootstrapProofManifest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_commit: String,
        predecessor_p11: ArtifactDigest,
        seed: BootstrapSeedManifest,
        t0: BootstrapGenerationId,
        t1: BootstrapGenerationId,
        t2: BootstrapGenerationId,
        program_source: ArtifactDigest,
        stage1: BootstrapRebuildManifest,
        stage2: BootstrapRebuildManifest,
        stage1_artifact: ArtifactDigest,
        stage2_artifact: ArtifactDigest,
        negative_controls: BootstrapNegativeControlManifest,
        universe_before: ArtifactDigest,
        universe_after: ArtifactDigest,
        checker_identity: ArtifactDigest,
        verifier_identity: ArtifactDigest,
    ) -> Self {
        Self {
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

    pub fn source_commit(&self) -> &str {
        &self.source_commit
    }
    pub fn predecessor_p11(&self) -> ArtifactDigest {
        self.predecessor_p11
    }
    pub fn seed(&self) -> &BootstrapSeedManifest {
        &self.seed
    }
    pub fn t0(&self) -> BootstrapGenerationId {
        self.t0
    }
    pub fn t1(&self) -> BootstrapGenerationId {
        self.t1
    }
    pub fn t2(&self) -> BootstrapGenerationId {
        self.t2
    }
    pub fn program_source(&self) -> ArtifactDigest {
        self.program_source
    }
    pub fn stage1(&self) -> &BootstrapRebuildManifest {
        &self.stage1
    }
    pub fn stage2(&self) -> &BootstrapRebuildManifest {
        &self.stage2
    }
    pub fn stage1_artifact(&self) -> ArtifactDigest {
        self.stage1_artifact
    }
    pub fn stage2_artifact(&self) -> ArtifactDigest {
        self.stage2_artifact
    }
    pub fn negative_controls(&self) -> &BootstrapNegativeControlManifest {
        &self.negative_controls
    }
    pub fn universe_before(&self) -> ArtifactDigest {
        self.universe_before
    }
    pub fn universe_after(&self) -> ArtifactDigest {
        self.universe_after
    }
    pub fn checker_identity(&self) -> ArtifactDigest {
        self.checker_identity
    }
    pub fn verifier_identity(&self) -> ArtifactDigest {
        self.verifier_identity
    }
}

impl StructuralIdentity for BootstrapProofManifest {
    fn canonical_value(&self) -> CanonicalValue {
        let mut value = object("BootstrapProofManifest");
        value.insert(
            "source_commit".into(),
            CanonicalValue::String(self.source_commit.clone()),
        );
        value.insert(
            "predecessor_p11".into(),
            CanonicalValue::Digest(self.predecessor_p11),
        );
        value.insert("seed".into(), self.seed.canonical_value());
        value.insert("t0".into(), self.t0.canonical_value());
        value.insert("t1".into(), self.t1.canonical_value());
        value.insert("t2".into(), self.t2.canonical_value());
        value.insert(
            "program_source".into(),
            CanonicalValue::Digest(self.program_source),
        );
        value.insert("stage1".into(), self.stage1.canonical_value());
        value.insert("stage2".into(), self.stage2.canonical_value());
        value.insert(
            "stage1_artifact".into(),
            CanonicalValue::Digest(self.stage1_artifact),
        );
        value.insert(
            "stage2_artifact".into(),
            CanonicalValue::Digest(self.stage2_artifact),
        );
        value.insert(
            "negative_controls".into(),
            self.negative_controls.canonical_value(),
        );
        value.insert(
            "universe_before".into(),
            CanonicalValue::Digest(self.universe_before),
        );
        value.insert(
            "universe_after".into(),
            CanonicalValue::Digest(self.universe_after),
        );
        value.insert(
            "checker_identity".into(),
            CanonicalValue::Digest(self.checker_identity),
        );
        value.insert(
            "verifier_identity".into(),
            CanonicalValue::Digest(self.verifier_identity),
        );
        CanonicalValue::Object(value)
    }
}
