use formula_core::{
    artifacts::StructuralIdentity,
    bootstrap::{BootstrapBytecode, BootstrapDecision, BootstrapProgramSource},
    digest::ArtifactDigest,
};
use formula_realize::bootstrap::{
    BootstrapExecutionError, BootstrapGenerationError, canonical_generator_image,
    compile_bootstrap_source, execute_bootstrap_bytecode, generator_image_from_bootstrap_artifact,
    rebuild_with_generator_image,
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

#[test]
fn canonical_identity_checker_compiles_to_exact_fbc1_bytecode() {
    let source = BootstrapProgramSource::identity_checker_v1();
    let bytecode = compile_bootstrap_source(&source).expect("canonical source compiles");
    assert_eq!(bytecode.bytes(), b"FBC1\x01\x02\x03\x04");
}

#[test]
fn image_driven_rebuild_matches_direct_compilation() {
    let source = BootstrapProgramSource::identity_checker_v1();
    let direct = compile_bootstrap_source(&source).unwrap();
    let image = canonical_generator_image();
    let rebuilt = rebuild_with_generator_image(&image, &source).unwrap();
    assert_eq!(direct, rebuilt);
    assert_ne!(image.structural_digest(), direct.structural_digest());
}

#[test]
fn admitted_stage1_artifact_seeds_the_stage2_generator_image() {
    let source = BootstrapProgramSource::identity_checker_v1();
    let admitted_stage1 = compile_bootstrap_source(&source).unwrap();
    let stage1_image = generator_image_from_bootstrap_artifact(&source, &admitted_stage1)
        .expect("validated admitted Stage1 artifact must seed Stage2 generator image");
    let stage2 = rebuild_with_generator_image(&stage1_image, &source).unwrap();

    assert_eq!(stage2, admitted_stage1);
    assert_eq!(stage1_image, canonical_generator_image());
}

#[test]
fn malformed_or_noncanonical_artifact_cannot_seed_stage2_generator() {
    let source = BootstrapProgramSource::identity_checker_v1();
    for bad in [
        BootstrapBytecode::new(vec![]),
        BootstrapBytecode::new(b"FBC1\x01\x02\x03".to_vec()),
        BootstrapBytecode::new(b"FBC1\x01\x02\x03\xff".to_vec()),
        BootstrapBytecode::new(b"NOPE\x01\x02\x03\x04".to_vec()),
    ] {
        assert_eq!(
            generator_image_from_bootstrap_artifact(&source, &bad),
            Err(BootstrapGenerationError::ArtifactCannotSeedGenerator)
        );
    }
}

#[test]
fn bootstrap_bytecode_executes_identity_semantics() {
    let bytecode =
        compile_bootstrap_source(&BootstrapProgramSource::identity_checker_v1()).unwrap();
    let same = d("same");
    assert_eq!(
        execute_bootstrap_bytecode(&bytecode, same, same),
        Ok(BootstrapDecision::Valid)
    );
    assert_eq!(
        execute_bootstrap_bytecode(&bytecode, d("actual"), d("expected")),
        Ok(BootstrapDecision::Reject)
    );
}

#[test]
fn malformed_or_unknown_bytecode_fails_closed() {
    for bad in [
        BootstrapBytecode::new(vec![]),
        BootstrapBytecode::new(b"FBC1\x01\x02\x03".to_vec()),
        BootstrapBytecode::new(b"FBC1\x01\x02\x03\xff".to_vec()),
        BootstrapBytecode::new(b"NOPE\x01\x02\x03\x04".to_vec()),
    ] {
        assert_eq!(
            execute_bootstrap_bytecode(&bad, d("a"), d("a")),
            Err(BootstrapExecutionError::MalformedOrUnsupportedBytecode)
        );
    }
}
