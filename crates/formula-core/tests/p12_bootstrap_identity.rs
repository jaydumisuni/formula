use formula_core::{
    artifacts::StructuralIdentity,
    bootstrap::{
        BootstrapEquivalenceLevel, BootstrapGenerationId, BootstrapInstruction,
        BootstrapNegativeControl, BootstrapNegativeControlEvidence,
        BootstrapNegativeControlManifest, BootstrapNegativeControlManifestError,
        BootstrapProgramSource, BootstrapRebuildManifest, BootstrapRole, BootstrapSeedManifest,
        BootstrapValidationState,
    },
    digest::ArtifactDigest,
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

#[test]
fn canonical_identity_checker_source_is_exactly_four_instructions() {
    let source = BootstrapProgramSource::identity_checker_v1();
    assert_eq!(
        source.instructions(),
        &[
            BootstrapInstruction::LoadActualDigest,
            BootstrapInstruction::LoadExpectedDigest,
            BootstrapInstruction::DigestEq,
            BootstrapInstruction::ReturnDecision,
        ]
    );
    assert_eq!(source.schema(), "FORMULA_BOOTSTRAP_CORE_V1");
}

#[test]
fn seed_identity_binds_every_toolchain_provenance_field() {
    let seed = BootstrapSeedManifest::new(
        BootstrapRole::ExternalToolchainSeed,
        "rust-1.98.0".into(),
        "88d9e12ae178fab0fb5cc050a94da85685d449ea".into(),
        "cargo-1.98.0".into(),
        "x86_64-unknown-linux-gnu".into(),
        d("rustc-bin"),
        d("cargo-bin"),
        d("rust-toolchain"),
        "dtolnay/rust-toolchain 1.98.0".into(),
        "Rust upstream toolchain provenance".into(),
    );
    let baseline = seed.structural_digest();
    assert_ne!(
        baseline,
        BootstrapSeedManifest::new(
            BootstrapRole::ExternalToolchainSeed,
            "rust-1.98.0".into(),
            "88d9e12ae178fab0fb5cc050a94da85685d449ea".into(),
            "cargo-1.98.0".into(),
            "x86_64-unknown-linux-gnu".into(),
            d("different-rustc"),
            d("cargo-bin"),
            d("rust-toolchain"),
            "dtolnay/rust-toolchain 1.98.0".into(),
            "Rust upstream toolchain provenance".into(),
        )
        .structural_digest()
    );
}

#[test]
fn rebuild_identity_binds_t_generations_generator_validator_and_equivalence() {
    let rebuild = BootstrapRebuildManifest::new(
        BootstrapGenerationId::new(0, d("t0")),
        BootstrapGenerationId::new(1, d("t1")),
        d("generator"),
        d("validator"),
        d("source"),
        d("recipe"),
        d("candidate"),
        d("independent"),
        d("normalization"),
        BootstrapEquivalenceLevel::ByteForByte,
        d("semantic-evidence"),
        d("seed"),
        BootstrapValidationState::Candidate,
    );
    let baseline = rebuild.structural_digest();
    assert_ne!(
        baseline,
        BootstrapRebuildManifest::new(
            BootstrapGenerationId::new(0, d("t0")),
            BootstrapGenerationId::new(1, d("t1")),
            d("generator-2"),
            d("validator"),
            d("source"),
            d("recipe"),
            d("candidate"),
            d("independent"),
            d("normalization"),
            BootstrapEquivalenceLevel::ByteForByte,
            d("semantic-evidence"),
            d("seed"),
            BootstrapValidationState::Candidate,
        )
        .structural_digest()
    );
}

#[test]
fn negative_control_manifest_requires_exact_nc_bs_01_through_10() {
    let complete = BootstrapNegativeControlManifest::new(
        BootstrapNegativeControl::ALL
            .into_iter()
            .map(|control| BootstrapNegativeControlEvidence::new(control, d(control.as_str())))
            .collect(),
    )
    .expect("all ten controls are complete");
    assert!(complete.is_complete());
    assert_eq!(complete.controls().len(), 10);

    assert_eq!(
        BootstrapNegativeControlManifest::new(
            BootstrapNegativeControl::ALL[..9]
                .iter()
                .copied()
                .map(|control| BootstrapNegativeControlEvidence::new(control, d(control.as_str())))
                .collect(),
        ),
        Err(BootstrapNegativeControlManifestError::MissingDuplicateOrUnexpectedControl)
    );
}
