use formula_check::{
    realization::{RealizationPolicyFailure, authorize_native_u8_realization_v1},
    u8::{BoolExpr, ByteExpr},
    verdict::CheckFailure,
};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::RealizationCheckManifest,
    digest::ArtifactDigest,
    realization::{NativeRealizationManifest, NativeToolchainIdentity, SpecializationIdentity},
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn semantic() -> BoolExpr {
    BoolExpr::And(
        Box::new(BoolExpr::NeqZero(ByteExpr::X)),
        Box::new(BoolExpr::EqZero(ByteExpr::BitAnd(
            Box::new(ByteExpr::X),
            Box::new(ByteExpr::SubWrap(
                Box::new(ByteExpr::X),
                Box::new(ByteExpr::Const(1)),
            )),
        ))),
    )
}

fn outputs(semantic: &BoolExpr) -> Vec<bool> {
    (0u16..=255)
        .map(|raw| semantic.evaluate(raw as u8))
        .collect()
}

struct Fixture {
    specialization: SpecializationIdentity,
    toolchain: NativeToolchainIdentity,
    native_manifest: NativeRealizationManifest,
    check_manifest: RealizationCheckManifest,
    source: Vec<u8>,
    binary: Vec<u8>,
    semantic: BoolExpr,
    outputs: Vec<bool>,
}

fn fixture() -> Fixture {
    let semantic_target = d("semantic-target");
    let generation = d("u1");
    let world = d("world");
    let authority = d("authority");
    let observer = d("observer");
    let source = b"fn main() { /* generated */ }\n".to_vec();
    let binary = b"native-binary-bytes".to_vec();
    let specialization =
        SpecializationIdentity::new(semantic_target, generation, world, authority, observer);
    let toolchain =
        NativeToolchainIdentity::new("1.98.0".into(), "x86_64-unknown-linux-gnu".into());
    let native_manifest = NativeRealizationManifest::new(
        semantic_target,
        generation,
        world,
        authority,
        observer,
        specialization.structural_digest(),
        ArtifactDigest::of_bytes(&source),
        toolchain.structural_digest(),
        ArtifactDigest::of_bytes(&binary),
    );
    let check_manifest = RealizationCheckManifest::new(
        semantic_target,
        native_manifest.structural_digest(),
        generation,
        world,
        authority,
        observer,
        ArtifactDigest::of_bytes(&binary),
    );
    let semantic = semantic();
    let outputs = outputs(&semantic);

    Fixture {
        specialization,
        toolchain,
        native_manifest,
        check_manifest,
        source,
        binary,
        semantic,
        outputs,
    }
}

fn authorize(
    fixture: &Fixture,
) -> Result<formula_check::realization::RealizationAuthorization, RealizationPolicyFailure> {
    authorize_native_u8_realization_v1(
        &fixture.native_manifest,
        &fixture.specialization,
        &fixture.toolchain,
        &fixture.check_manifest,
        &fixture.source,
        &fixture.binary,
        &fixture.semantic,
        &fixture.outputs,
    )
}

#[test]
fn exact_native_bindings_receive_opaque_authorization() {
    let fixture = fixture();

    let authorization = authorize(&fixture).unwrap();

    assert_eq!(
        authorization.realization_manifest(),
        fixture.native_manifest.structural_digest()
    );
    assert_eq!(
        authorization.binary_digest(),
        fixture.native_manifest.binary_digest()
    );
    assert_eq!(
        authorization.universe_generation(),
        fixture.native_manifest.universe_generation()
    );
    assert_eq!(
        authorization.semantic_target(),
        fixture.native_manifest.semantic_target()
    );
}

#[test]
fn changed_source_or_binary_bytes_fail_closed() {
    let fixture = fixture();
    let mut changed_source = fixture.source.clone();
    changed_source.push(b'!');
    let mut changed_binary = fixture.binary.clone();
    changed_binary.push(0xff);

    let source_error = authorize_native_u8_realization_v1(
        &fixture.native_manifest,
        &fixture.specialization,
        &fixture.toolchain,
        &fixture.check_manifest,
        &changed_source,
        &fixture.binary,
        &fixture.semantic,
        &fixture.outputs,
    )
    .unwrap_err();
    let binary_error = authorize_native_u8_realization_v1(
        &fixture.native_manifest,
        &fixture.specialization,
        &fixture.toolchain,
        &fixture.check_manifest,
        &fixture.source,
        &changed_binary,
        &fixture.semantic,
        &fixture.outputs,
    )
    .unwrap_err();

    assert_eq!(source_error, CheckFailure::RealizationSourceDigestMismatch);
    assert_eq!(
        binary_error,
        CheckFailure::RealizationArtifactDigestMismatch
    );
}

#[test]
fn changed_specialization_toolchain_or_checker_binding_fails_closed() {
    let fixture = fixture();
    let changed_specialization = SpecializationIdentity::new(
        fixture.native_manifest.semantic_target(),
        d("u2"),
        fixture.native_manifest.world(),
        fixture.native_manifest.authority_contract(),
        fixture.native_manifest.observer(),
    );
    let changed_toolchain =
        NativeToolchainIdentity::new("1.99.0".into(), "x86_64-unknown-linux-gnu".into());
    let changed_check_manifest = RealizationCheckManifest::new(
        fixture.native_manifest.semantic_target(),
        fixture.native_manifest.structural_digest(),
        d("u2"),
        fixture.native_manifest.world(),
        fixture.native_manifest.authority_contract(),
        fixture.native_manifest.observer(),
        fixture.native_manifest.binary_digest(),
    );

    for error in [
        authorize_native_u8_realization_v1(
            &fixture.native_manifest,
            &changed_specialization,
            &fixture.toolchain,
            &fixture.check_manifest,
            &fixture.source,
            &fixture.binary,
            &fixture.semantic,
            &fixture.outputs,
        )
        .unwrap_err(),
        authorize_native_u8_realization_v1(
            &fixture.native_manifest,
            &fixture.specialization,
            &changed_toolchain,
            &fixture.check_manifest,
            &fixture.source,
            &fixture.binary,
            &fixture.semantic,
            &fixture.outputs,
        )
        .unwrap_err(),
        authorize_native_u8_realization_v1(
            &fixture.native_manifest,
            &fixture.specialization,
            &fixture.toolchain,
            &changed_check_manifest,
            &fixture.source,
            &fixture.binary,
            &fixture.semantic,
            &fixture.outputs,
        )
        .unwrap_err(),
    ] {
        assert_eq!(error, CheckFailure::RealizationNativeBindingMismatch);
    }
}

#[test]
fn inherited_exhaustive_checker_controls_authorization() {
    let fixture = fixture();
    let short_outputs = &fixture.outputs[..255];
    let mut wrong_outputs = fixture.outputs.clone();
    wrong_outputs[64] = !wrong_outputs[64];

    let coverage_error = authorize_native_u8_realization_v1(
        &fixture.native_manifest,
        &fixture.specialization,
        &fixture.toolchain,
        &fixture.check_manifest,
        &fixture.source,
        &fixture.binary,
        &fixture.semantic,
        short_outputs,
    )
    .unwrap_err();
    let counterexample_error = authorize_native_u8_realization_v1(
        &fixture.native_manifest,
        &fixture.specialization,
        &fixture.toolchain,
        &fixture.check_manifest,
        &fixture.source,
        &fixture.binary,
        &fixture.semantic,
        &wrong_outputs,
    )
    .unwrap_err();

    assert_eq!(
        coverage_error,
        CheckFailure::RealizationOutputCoverageMismatch
    );
    assert_eq!(
        counterexample_error,
        CheckFailure::RealizationCounterexample(64)
    );
}
