use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    realization::{
        NativeRealizationManifest, NativeToolchainIdentity, RealizationDispatchContext,
        SpecializationIdentity,
    },
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn specialization(semantic_target: ArtifactDigest) -> SpecializationIdentity {
    SpecializationIdentity::new(
        semantic_target,
        d("u1"),
        d("world"),
        d("authority"),
        d("observer"),
    )
}

#[test]
fn specialization_identity_binds_exact_p8_contract() {
    let base = specialization(d("semantic"));
    let same = specialization(d("semantic"));
    let changed_semantic = specialization(d("other-semantic"));

    assert_eq!(base.structural_digest(), same.structural_digest());
    assert_ne!(base.structural_digest(), changed_semantic.structural_digest());
    assert_eq!(base.query_direction(), "u8_to_bool_forward");
    assert_eq!(base.input_domain(), "u8:0..=255");
    assert_eq!(base.output_domain(), "bool");
    assert_eq!(base.lowering_class(), "EXACT_EQUIVALENCE");
}

#[test]
fn toolchain_identity_binds_release_and_host_target() {
    let base = NativeToolchainIdentity::new("1.98.0".into(), "x86_64-unknown-linux-gnu".into());
    let same = NativeToolchainIdentity::new("1.98.0".into(), "x86_64-unknown-linux-gnu".into());
    let changed_release =
        NativeToolchainIdentity::new("1.99.0".into(), "x86_64-unknown-linux-gnu".into());
    let changed_target =
        NativeToolchainIdentity::new("1.98.0".into(), "aarch64-unknown-linux-gnu".into());

    assert_eq!(base.structural_digest(), same.structural_digest());
    assert_ne!(base.structural_digest(), changed_release.structural_digest());
    assert_ne!(base.structural_digest(), changed_target.structural_digest());
    assert_eq!(base.compiler(), "rustc");
    assert_eq!(base.rust_release(), "1.98.0");
    assert_eq!(base.optimization(), "-O");
    assert_eq!(base.host_target(), "x86_64-unknown-linux-gnu");
    assert_eq!(base.backend_family(), "standalone-rust-native");
}

fn manifest(
    semantic_target: ArtifactDigest,
    specialization_digest: ArtifactDigest,
    source_digest: ArtifactDigest,
    toolchain_digest: ArtifactDigest,
    binary_digest: ArtifactDigest,
) -> NativeRealizationManifest {
    NativeRealizationManifest::new(
        semantic_target,
        d("u1"),
        d("world"),
        d("authority"),
        d("observer"),
        specialization_digest,
        source_digest,
        toolchain_digest,
        binary_digest,
    )
}

#[test]
fn native_manifest_binds_source_binary_toolchain_and_specialization() {
    let base = manifest(
        d("semantic"),
        d("specialization"),
        d("source"),
        d("toolchain"),
        d("binary"),
    );
    let same = manifest(
        d("semantic"),
        d("specialization"),
        d("source"),
        d("toolchain"),
        d("binary"),
    );
    let changed_source = manifest(
        d("semantic"),
        d("specialization"),
        d("other-source"),
        d("toolchain"),
        d("binary"),
    );
    let changed_binary = manifest(
        d("semantic"),
        d("specialization"),
        d("source"),
        d("toolchain"),
        d("other-binary"),
    );
    let changed_toolchain = manifest(
        d("semantic"),
        d("specialization"),
        d("source"),
        d("other-toolchain"),
        d("binary"),
    );
    let changed_specialization = manifest(
        d("semantic"),
        d("other-specialization"),
        d("source"),
        d("toolchain"),
        d("binary"),
    );

    assert_eq!(base.structural_digest(), same.structural_digest());
    assert_ne!(base.structural_digest(), changed_source.structural_digest());
    assert_ne!(base.structural_digest(), changed_binary.structural_digest());
    assert_ne!(base.structural_digest(), changed_toolchain.structural_digest());
    assert_ne!(
        base.structural_digest(),
        changed_specialization.structural_digest()
    );
    assert_eq!(base.lowering_class(), "EXACT_EQUIVALENCE");
    assert_eq!(base.input_representation(), "u8");
    assert_eq!(base.output_representation(), "bool");
    assert_eq!(base.fallback_semantics(), "semantic_execution");
}

#[test]
fn dispatch_context_binds_authority_scope() {
    let base = RealizationDispatchContext::new(
        d("semantic"),
        d("u1"),
        d("world"),
        d("authority"),
        d("observer"),
    );
    let changed_generation = RealizationDispatchContext::new(
        d("semantic"),
        d("u2"),
        d("world"),
        d("authority"),
        d("observer"),
    );

    assert_ne!(base.structural_digest(), changed_generation.structural_digest());
}
