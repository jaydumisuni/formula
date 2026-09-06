use formula_check::bootstrap::validate_bootstrap_candidate;
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
use formula_store::authority_store::AuthorityStore;
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
        d("rustc"), d("cargo"), d("rust-toolchain"),
        "pinned".into(), "upstream".into(),
    )
}

fn authorization(
    predecessor: BootstrapGenerationId,
    successor: BootstrapGenerationId,
    generator: ArtifactDigest,
) -> (formula_check::bootstrap::BootstrapValidationAuthorization, BootstrapBytecode) {
    let source = BootstrapProgramSource::identity_checker_v1();
    let candidate = BootstrapBytecode::new(b"FBC1\x01\x02\x03\x04".to_vec());
    let rebuild = BootstrapRebuildManifest::new(
        predecessor,
        successor,
        generator,
        d("independent-validator"),
        source.structural_digest(),
        d("recipe"),
        candidate.structural_digest(),
        candidate.structural_digest(),
        d("normalization:none"),
        BootstrapEquivalenceLevel::ByteForByte,
        d("semantic-evidence"),
        seed().structural_digest(),
        BootstrapValidationState::Candidate,
    );
    let authorization = validate_bootstrap_candidate(&rebuild, &source, &candidate, &seed()).unwrap();
    (authorization, candidate)
}

#[test]
fn bootstrap_root_and_successors_are_append_only_and_u_is_unchanged() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![d("math-base")], vec![]);
    let u0_digest = store.initialize_genesis(&u0).unwrap();

    let t0 = store.create_bootstrap_root(&seed()).unwrap();
    assert_eq!(t0.ordinal(), 0);
    assert_eq!(store.active_bootstrap_generation().unwrap(), t0);
    assert_eq!(store.active_generation().unwrap(), Some(u0_digest));

    let t1 = BootstrapGenerationId::new(1, d("t1"));
    let (auth1, candidate1) = authorization(t0, t1, d("stage0-generator"));
    assert_eq!(store.admit_bootstrap_successor(&auth1, &candidate1).unwrap(), t1);
    assert_eq!(store.active_bootstrap_generation().unwrap(), t1);
    assert_eq!(store.active_generation().unwrap(), Some(u0_digest));

    let t2 = BootstrapGenerationId::new(2, d("t2"));
    let (auth2, candidate2) = authorization(t1, t2, d("stage1-generator"));
    assert_eq!(store.admit_bootstrap_successor(&auth2, &candidate2).unwrap(), t2);
    assert_eq!(store.active_bootstrap_generation().unwrap(), t2);
    assert_eq!(store.active_generation().unwrap(), Some(u0_digest));

    assert_eq!(store.replay_bootstrap_generation(t0).unwrap().id(), t0);
    assert_eq!(store.replay_bootstrap_generation(t1).unwrap().id(), t1);
    assert_eq!(store.replay_bootstrap_generation(t2).unwrap().id(), t2);

    store.select_bootstrap_generation(t1).unwrap();
    assert_eq!(store.active_bootstrap_generation().unwrap(), t1);
    assert_eq!(store.replay_bootstrap_generation(t2).unwrap().id(), t2);
    assert_eq!(store.active_generation().unwrap(), Some(u0_digest));
}

#[test]
fn candidate_tampering_cannot_advance_bootstrap_generation() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let t0 = store.create_bootstrap_root(&seed()).unwrap();
    let t1 = BootstrapGenerationId::new(1, d("t1"));
    let (authorization, _candidate) = authorization(t0, t1, d("generator"));
    let tampered = BootstrapBytecode::new(b"FBC1\x01\x02\x03\xff".to_vec());

    assert!(store.admit_bootstrap_successor(&authorization, &tampered).is_err());
    assert_eq!(store.active_bootstrap_generation().unwrap(), t0);
}
