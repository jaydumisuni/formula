use formula_check::bootstrap::{
    BootstrapValidationFailure, reference_compile, reference_execute, validate_bootstrap_candidate,
};
use formula_core::{
    artifacts::StructuralIdentity,
    bootstrap::{
        BootstrapBytecode, BootstrapDecision, BootstrapEquivalenceLevel, BootstrapGenerationId,
        BootstrapProgramSource, BootstrapRebuildManifest, BootstrapRole, BootstrapSeedManifest,
        BootstrapValidationState,
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
        d("rustc"),
        d("cargo"),
        d("rust-toolchain"),
        "pinned".into(),
        "upstream".into(),
    )
}

fn rebuild(
    source: &BootstrapProgramSource,
    candidate: &BootstrapBytecode,
    generator: ArtifactDigest,
    validator: ArtifactDigest,
) -> BootstrapRebuildManifest {
    let independent = BootstrapBytecode::new(b"FBC1\x01\x02\x03\x04".to_vec());
    BootstrapRebuildManifest::new(
        BootstrapGenerationId::new(0, d("t0")),
        BootstrapGenerationId::new(1, d("t1")),
        generator,
        validator,
        source.structural_digest(),
        d("recipe"),
        candidate.structural_digest(),
        independent.structural_digest(),
        d("normalization:none"),
        BootstrapEquivalenceLevel::ByteForByte,
        d("semantic-evidence"),
        seed().structural_digest(),
        BootstrapValidationState::Candidate,
    )
}

#[test]
fn reference_path_compiles_and_executes_without_generator_dependency() {
    let source = BootstrapProgramSource::identity_checker_v1();
    let candidate = reference_compile(&source).expect("reference compiler accepts canonical source");
    assert_eq!(candidate.bytes(), b"FBC1\x01\x02\x03\x04");
    let same = d("same");
    assert_eq!(
        reference_execute(&source, same, same),
        Ok(BootstrapDecision::Valid)
    );
    assert_eq!(
        reference_execute(&source, d("a"), d("b")),
        Ok(BootstrapDecision::Reject)
    );
}

#[test]
fn distinct_generator_and_validator_are_mandatory() {
    let source = BootstrapProgramSource::identity_checker_v1();
    let candidate = BootstrapBytecode::new(b"FBC1\x01\x02\x03\x04".to_vec());
    let same = d("same-implementation");
    assert_eq!(
        validate_bootstrap_candidate(&rebuild(&source, &candidate, same, same), &source, &candidate, &seed()),
        Err(BootstrapValidationFailure::GeneratorEqualsValidator)
    );
}

#[test]
fn exact_seed_source_recipe_and_candidate_bindings_are_required() {
    let source = BootstrapProgramSource::identity_checker_v1();
    let candidate = BootstrapBytecode::new(b"FBC1\x01\x02\x03\x04".to_vec());
    let generator = d("generator");
    let validator = d("validator");

    let mut wrong_seed = rebuild(&source, &candidate, generator, validator);
    wrong_seed = BootstrapRebuildManifest::new(
        wrong_seed.predecessor(), wrong_seed.successor(), generator, validator,
        source.structural_digest(), d("recipe"), candidate.structural_digest(),
        candidate.structural_digest(), d("normalization:none"),
        BootstrapEquivalenceLevel::ByteForByte, d("semantic-evidence"), d("wrong-seed"),
        BootstrapValidationState::Candidate,
    );
    assert_eq!(
        validate_bootstrap_candidate(&wrong_seed, &source, &candidate, &seed()),
        Err(BootstrapValidationFailure::SeedProvenanceMismatch)
    );

    let wrong_source = BootstrapRebuildManifest::new(
        BootstrapGenerationId::new(0, d("t0")), BootstrapGenerationId::new(1, d("t1")),
        generator, validator, d("wrong-source"), d("recipe"), candidate.structural_digest(),
        candidate.structural_digest(), d("normalization:none"), BootstrapEquivalenceLevel::ByteForByte,
        d("semantic-evidence"), seed().structural_digest(), BootstrapValidationState::Candidate,
    );
    assert_eq!(
        validate_bootstrap_candidate(&wrong_source, &source, &candidate, &seed()),
        Err(BootstrapValidationFailure::SourceDigestMismatch)
    );

    let wrong_candidate = BootstrapBytecode::new(b"FBC1\x01\x02\x03\xff".to_vec());
    assert_eq!(
        validate_bootstrap_candidate(
            &rebuild(&source, &wrong_candidate, generator, validator),
            &source,
            &wrong_candidate,
            &seed(),
        ),
        Err(BootstrapValidationFailure::CandidateReferenceMismatch)
    );
}

#[test]
fn valid_candidate_receives_checker_owned_authorization() {
    let source = BootstrapProgramSource::identity_checker_v1();
    let candidate = BootstrapBytecode::new(b"FBC1\x01\x02\x03\x04".to_vec());
    let manifest = rebuild(&source, &candidate, d("generator"), d("validator"));
    let authorization = validate_bootstrap_candidate(&manifest, &source, &candidate, &seed())
        .expect("independent validation must authorize exact candidate");
    assert_eq!(authorization.predecessor(), manifest.predecessor());
    assert_eq!(authorization.successor(), manifest.successor());
    assert_eq!(authorization.candidate_artifact(), candidate.structural_digest());
    assert_eq!(authorization.generator_identity(), d("generator"));
    assert_eq!(authorization.validator_identity(), d("validator"));
}
