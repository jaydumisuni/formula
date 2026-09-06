use formula_core::{
    artifacts::StructuralIdentity,
    bootstrap::{BootstrapBytecode, BootstrapDecision, BootstrapProgramSource},
    digest::ArtifactDigest,
};
use formula_realize::bootstrap::{
    BootstrapExecutionError, canonical_generator_image, compile_bootstrap_source,
    execute_bootstrap_bytecode, rebuild_with_generator_image,
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
fn bootstrap_bytecode_executes_identity_semantics() {
    let bytecode = compile_bootstrap_source(&BootstrapProgramSource::identity_checker_v1()).unwrap();
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
