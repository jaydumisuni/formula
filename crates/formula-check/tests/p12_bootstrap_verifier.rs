use formula_check::{
    bootstrap::{
        canonical_build_recipe_identity, canonical_normalization_rules_identity, reference_compile,
        semantic_evidence_identity,
    },
    bootstrap_verifier::{
        BootstrapProofFailure, BootstrapReplayEvidence, P12_CANONICAL_MARKERS,
        verify_bootstrap_proof_manifest,
    },
};
use formula_core::{
    artifacts::StructuralIdentity,
    bootstrap::{
        BOOTSTRAP_CORE_SCHEMA_V1, BootstrapBytecode, BootstrapEquivalenceLevel,
        BootstrapGenerationId, BootstrapGeneratorImage, BootstrapInstruction,
        BootstrapNegativeControl, BootstrapNegativeControlEvidence,
        BootstrapNegativeControlManifest, BootstrapProgramSource, BootstrapProofManifest,
        BootstrapRebuildManifest, BootstrapRole, BootstrapSeedManifest, BootstrapValidationState,
    },
    digest::ArtifactDigest,
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn seed() -> BootstrapSeedManifest {
    BootstrapSeedManifest::new(
        BootstrapRole::ExternalToolchainSeed,
        "rust-1.98.0".into(),
        "88d9e12ae178fab0fb5cc050a94da85685d449ea".into(),
        "cargo-1.98.0".into(),
        "x86_64-unknown-linux-gnu".into(),
        d("rustc-sha256"),
        d("cargo-sha256"),
        d("rust-toolchain-sha256"),
        "pinned-rust-1.98.0".into(),
        "workflow-sha256".into(),
    )
}

fn checker_identity() -> ArtifactDigest {
    d("formula-check:p12-bootstrap-independent-validator:v1")
}

fn verifier_identity() -> ArtifactDigest {
    d("formula-check:p12-bootstrap-final-replay:v1")
}

fn p11_identity() -> ArtifactDigest {
    ArtifactDigest::of_bytes(b"6f8ce7bb6702ea1baf119aab9950aa5ba0f87283")
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

fn generator_from_artifact(
    source: &BootstrapProgramSource,
    artifact: &BootstrapBytecode,
) -> BootstrapGeneratorImage {
    BootstrapGeneratorImage::new(
        source.schema().into(),
        artifact.bytes()[..4].to_vec(),
        source
            .instructions()
            .iter()
            .copied()
            .zip(artifact.bytes()[4..].iter().copied())
            .collect(),
    )
}

fn successor_generation(
    predecessor: BootstrapGenerationId,
    generator: ArtifactDigest,
    validator: ArtifactDigest,
    source: ArtifactDigest,
    candidate: ArtifactDigest,
    seed: ArtifactDigest,
) -> BootstrapGenerationId {
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
    BootstrapGenerationId::new(predecessor.ordinal() + 1, ArtifactDigest::of_bytes(&bytes))
}

fn rebuild(
    predecessor: BootstrapGenerationId,
    successor: BootstrapGenerationId,
    generator: ArtifactDigest,
    validator: ArtifactDigest,
    source: &BootstrapProgramSource,
    candidate: &BootstrapBytecode,
    seed: &BootstrapSeedManifest,
) -> BootstrapRebuildManifest {
    let independent = reference_compile(source).unwrap();
    BootstrapRebuildManifest::new(
        predecessor,
        successor,
        generator,
        validator,
        source.structural_digest(),
        canonical_build_recipe_identity(),
        candidate.structural_digest(),
        independent.structural_digest(),
        canonical_normalization_rules_identity(),
        BootstrapEquivalenceLevel::ByteForByte,
        semantic_evidence_identity(source, candidate),
        seed.structural_digest(),
        BootstrapValidationState::Candidate,
    )
}

fn negative_controls() -> Vec<BootstrapNegativeControlEvidence> {
    BootstrapNegativeControl::ALL
        .into_iter()
        .map(|control| BootstrapNegativeControlEvidence::new(control, d(control.as_str())))
        .collect()
}

fn fixture() -> (BootstrapProofManifest, BootstrapReplayEvidence) {
    let seed = seed();
    let source = BootstrapProgramSource::identity_checker_v1();
    let stage1 = reference_compile(&source).unwrap();
    let stage0 = canonical_stage0_image();
    let validator = checker_identity();
    let t0 = BootstrapGenerationId::new(0, seed.structural_digest());
    let t1 = successor_generation(
        t0,
        stage0.structural_digest(),
        validator,
        source.structural_digest(),
        stage1.structural_digest(),
        seed.structural_digest(),
    );
    let stage1_rebuild = rebuild(
        t0,
        t1,
        stage0.structural_digest(),
        validator,
        &source,
        &stage1,
        &seed,
    );

    let stage1_generator = generator_from_artifact(&source, &stage1);
    let stage2 = reference_compile(&source).unwrap();
    let t2 = successor_generation(
        t1,
        stage1_generator.structural_digest(),
        validator,
        source.structural_digest(),
        stage2.structural_digest(),
        seed.structural_digest(),
    );
    let stage2_rebuild = rebuild(
        t1,
        t2,
        stage1_generator.structural_digest(),
        validator,
        &source,
        &stage2,
        &seed,
    );

    let controls = negative_controls();
    let control_manifest = BootstrapNegativeControlManifest::new(controls.clone()).unwrap();
    let u = d("p12-universe-generation");
    let manifest = BootstrapProofManifest::new(
        "p12-source-commit".into(),
        p11_identity(),
        seed.clone(),
        t0,
        t1,
        t2,
        source.structural_digest(),
        stage1_rebuild.clone(),
        stage2_rebuild.clone(),
        stage1.structural_digest(),
        stage2.structural_digest(),
        control_manifest,
        u,
        u,
        checker_identity(),
        verifier_identity(),
    );
    let replay = BootstrapReplayEvidence::new(
        manifest.structural_digest(),
        "p12-source-commit".into(),
        p11_identity(),
        seed,
        t0,
        t1,
        t2,
        source,
        stage1_rebuild,
        stage2_rebuild,
        stage1,
        stage2,
        controls,
        u,
        u,
        checker_identity(),
        verifier_identity(),
    );
    (manifest, replay)
}

#[test]
fn exact_replay_emits_only_the_frozen_p12_marker_contract() {
    let (manifest, replay) = fixture();
    let verified = verify_bootstrap_proof_manifest(&manifest, &replay).unwrap();
    assert_eq!(verified.markers(), &P12_CANONICAL_MARKERS);
    assert_eq!(
        P12_CANONICAL_MARKERS,
        [
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
        ]
    );
}

#[test]
fn manifest_mutation_fails_closed() {
    let (manifest, mut replay) = fixture();
    replay.set_manifest_digest_for_test(d("mutated-proof-manifest"));
    assert_eq!(
        verify_bootstrap_proof_manifest(&manifest, &replay),
        Err(BootstrapProofFailure::ReplayBindingMismatch)
    );
}

#[test]
fn incomplete_negative_control_manifest_fails_closed() {
    let (manifest, mut replay) = fixture();
    let mut controls = negative_controls();
    controls.pop();
    replay.set_negative_controls_for_test(controls);
    assert_eq!(
        verify_bootstrap_proof_manifest(&manifest, &replay),
        Err(BootstrapProofFailure::NegativeControlsIncomplete)
    );
}

#[test]
fn generator_cannot_equal_validator() {
    let (manifest, mut replay) = fixture();
    replay.set_stage2_generator_identity_for_test(checker_identity());
    assert_eq!(
        verify_bootstrap_proof_manifest(&manifest, &replay),
        Err(BootstrapProofFailure::GeneratorValidatorNotDiverse)
    );
}

#[test]
fn t_stage_mismatch_fails_closed() {
    let (manifest, mut replay) = fixture();
    replay.set_t2_for_test(BootstrapGenerationId::new(9, d("wrong-t2")));
    assert_eq!(
        verify_bootstrap_proof_manifest(&manifest, &replay),
        Err(BootstrapProofFailure::BootstrapStageMismatch)
    );
}

#[test]
fn candidate_artifact_mismatch_fails_closed() {
    let (manifest, mut replay) = fixture();
    replay.set_stage2_artifact_for_test(BootstrapBytecode::new(b"FBC1\x01\x02\x03\xff".to_vec()));
    assert_eq!(
        verify_bootstrap_proof_manifest(&manifest, &replay),
        Err(BootstrapProofFailure::ArtifactMismatch)
    );
}

#[test]
fn universe_generation_mutation_fails_closed() {
    let (manifest, mut replay) = fixture();
    replay.set_universe_after_for_test(d("mutated-universe"));
    assert_eq!(
        verify_bootstrap_proof_manifest(&manifest, &replay),
        Err(BootstrapProofFailure::UniverseAuthorityChanged)
    );
}
