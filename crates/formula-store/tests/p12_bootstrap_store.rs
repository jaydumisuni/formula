use formula_check::bootstrap::{
    canonical_build_recipe_identity, canonical_normalization_rules_identity,
    semantic_evidence_identity, validate_bootstrap_candidate,
};
use formula_core::{
    artifacts::StructuralIdentity,
    bootstrap::{
        BootstrapBytecode, BootstrapEquivalenceLevel, BootstrapGenerationId,
        BootstrapProgramSource, BootstrapRebuildManifest, BootstrapRole, BootstrapSeedManifest,
        BootstrapValidationState,
    },
    digest::ArtifactDigest,
    generation::UniverseGeneration,
};
use formula_store::{authority_store::AuthorityStore, bootstrap_store::BootstrapAuthorityStore};
use tempfile::tempdir;

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

fn authorization(
    predecessor: BootstrapGenerationId,
    successor: BootstrapGenerationId,
    generator: ArtifactDigest,
) -> (
    formula_check::bootstrap::BootstrapValidationAuthorization,
    BootstrapBytecode,
) {
    let source = BootstrapProgramSource::identity_checker_v1();
    let candidate = BootstrapBytecode::new(b"FBC1\x01\x02\x03\x04".to_vec());
    let rebuild = BootstrapRebuildManifest::new(
        predecessor,
        successor,
        generator,
        d("independent-validator"),
        source.structural_digest(),
        canonical_build_recipe_identity(),
        candidate.structural_digest(),
        candidate.structural_digest(),
        canonical_normalization_rules_identity(),
        BootstrapEquivalenceLevel::ByteForByte,
        semantic_evidence_identity(&source, &candidate),
        seed().structural_digest(),
        BootstrapValidationState::Candidate,
    );
    let authorization =
        validate_bootstrap_candidate(&rebuild, &source, &candidate, &seed()).unwrap();
    (authorization, candidate)
}

#[test]
fn bootstrap_root_and_successors_are_append_only_and_u_is_unchanged() {
    let dir = tempdir().unwrap();
    let mut universe = AuthorityStore::open(dir.path()).unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![d("math-base")], vec![]);
    let u0_digest = universe.initialize_genesis(&u0).unwrap();
    let mut bootstrap = BootstrapAuthorityStore::open(dir.path()).unwrap();

    assert!(dir.path().join("authority.sqlite").is_file());
    assert!(dir.path().join("bootstrap-authority.sqlite").is_file());
    assert_ne!(
        dir.path().join("authority.sqlite"),
        dir.path().join("bootstrap-authority.sqlite")
    );

    let t0 = bootstrap.create_bootstrap_root(&seed()).unwrap();
    assert_eq!(t0.ordinal(), 0);
    assert_eq!(bootstrap.active_bootstrap_generation().unwrap(), t0);
    assert_eq!(universe.active_generation().unwrap(), Some(u0_digest));

    let t1 = BootstrapGenerationId::new(1, d("t1"));
    let (auth1, candidate1) = authorization(t0, t1, d("stage0-generator"));
    assert_eq!(
        bootstrap
            .admit_bootstrap_successor(&auth1, &candidate1)
            .unwrap(),
        t1
    );
    assert_eq!(bootstrap.active_bootstrap_generation().unwrap(), t1);
    assert_eq!(universe.active_generation().unwrap(), Some(u0_digest));

    let t2 = BootstrapGenerationId::new(2, d("t2"));
    let (auth2, candidate2) = authorization(t1, t2, d("stage1-generator"));
    assert_eq!(
        bootstrap
            .admit_bootstrap_successor(&auth2, &candidate2)
            .unwrap(),
        t2
    );
    assert_eq!(bootstrap.active_bootstrap_generation().unwrap(), t2);
    assert_eq!(universe.active_generation().unwrap(), Some(u0_digest));

    assert_eq!(bootstrap.replay_bootstrap_generation(t0).unwrap().id(), t0);
    assert_eq!(bootstrap.replay_bootstrap_generation(t1).unwrap().id(), t1);
    assert_eq!(bootstrap.replay_bootstrap_generation(t2).unwrap().id(), t2);

    bootstrap.select_bootstrap_generation(t1).unwrap();
    assert_eq!(bootstrap.active_bootstrap_generation().unwrap(), t1);
    assert_eq!(bootstrap.replay_bootstrap_generation(t2).unwrap().id(), t2);
    assert_eq!(universe.active_generation().unwrap(), Some(u0_digest));
}

#[test]
fn candidate_tampering_cannot_advance_bootstrap_generation() {
    let dir = tempdir().unwrap();
    let mut bootstrap = BootstrapAuthorityStore::open(dir.path()).unwrap();
    let t0 = bootstrap.create_bootstrap_root(&seed()).unwrap();
    let t1 = BootstrapGenerationId::new(1, d("t1"));
    let (authorization, _candidate) = authorization(t0, t1, d("generator"));
    let tampered = BootstrapBytecode::new(b"FBC1\x01\x02\x03\xff".to_vec());

    assert!(
        bootstrap
            .admit_bootstrap_successor(&authorization, &tampered)
            .is_err()
    );
    assert_eq!(bootstrap.active_bootstrap_generation().unwrap(), t0);
}
