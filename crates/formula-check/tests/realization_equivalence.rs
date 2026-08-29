use formula_check::{
    realization::{check_u8_realization_equivalence, RealizationCheckRequest},
    u8::{BoolExpr, ByteExpr},
    verdict::{CheckFailure, CheckVerdict},
};
use formula_core::{certification::RealizationCheckManifest, digest::ArtifactDigest};

fn d(label: &[u8]) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label)
}

fn sub_one() -> ByteExpr {
    ByteExpr::SubWrap(Box::new(ByteExpr::X), Box::new(ByteExpr::Const(1)))
}

fn admitted_semantics() -> BoolExpr {
    BoolExpr::And(
        Box::new(BoolExpr::NeqZero(ByteExpr::X)),
        Box::new(BoolExpr::EqZero(ByteExpr::BitAnd(
            Box::new(ByteExpr::X),
            Box::new(sub_one()),
        ))),
    )
}

fn mutated_semantics() -> BoolExpr {
    BoolExpr::EqZero(ByteExpr::BitAnd(
        Box::new(ByteExpr::X),
        Box::new(sub_one()),
    ))
}

fn outputs(expr: &BoolExpr) -> Vec<bool> {
    (0u16..=255).map(|raw| expr.evaluate(raw as u8)).collect()
}

fn manifest(artifact_bytes: &[u8]) -> RealizationCheckManifest {
    RealizationCheckManifest::new(
        d(b"semantic-target"),
        d(b"realization-metadata"),
        d(b"generation"),
        d(b"world"),
        d(b"authority-contract"),
        d(b"observer"),
        ArtifactDigest::of_bytes(artifact_bytes),
    )
}

fn request<'a>(
    manifest: &'a RealizationCheckManifest,
    artifact_bytes: &'a [u8],
    semantic: &'a BoolExpr,
    realized_outputs: &'a [bool],
) -> RealizationCheckRequest<'a> {
    RealizationCheckRequest::new(
        manifest,
        d(b"semantic-target"),
        d(b"realization-metadata"),
        d(b"generation"),
        d(b"world"),
        d(b"authority-contract"),
        d(b"observer"),
        artifact_bytes,
        semantic,
        realized_outputs,
    )
}

#[test]
fn compiler_claim_without_independent_output_check_is_not_authority() {
    let compiler_reported_success = true;
    assert!(compiler_reported_success);

    let artifact = b"compiled-native-artifact";
    let manifest = manifest(artifact);
    let semantic = admitted_semantics();
    let verdict = check_u8_realization_equivalence(&request(
        &manifest,
        artifact,
        &semantic,
        &[],
    ));

    assert_eq!(
        verdict,
        CheckVerdict::Fail(CheckFailure::RealizationOutputCoverageMismatch)
    );
}

#[test]
fn exact_realization_outputs_for_all_256_inputs_pass() {
    let artifact = b"compiled-native-artifact";
    let manifest = manifest(artifact);
    let semantic = admitted_semantics();
    let realized = outputs(&semantic);

    assert_eq!(
        check_u8_realization_equivalence(&request(
            &manifest,
            artifact,
            &semantic,
            &realized,
        )),
        CheckVerdict::Pass
    );
}

#[test]
fn mutated_realization_missing_zero_guard_fails() {
    let artifact = b"compiled-native-artifact";
    let manifest = manifest(artifact);
    let semantic = admitted_semantics();
    let realized = outputs(&mutated_semantics());

    assert_eq!(
        check_u8_realization_equivalence(&request(
            &manifest,
            artifact,
            &semantic,
            &realized,
        )),
        CheckVerdict::Fail(CheckFailure::RealizationCounterexample(0))
    );
}

#[test]
fn binary_or_artifact_digest_mismatch_fails_before_execution_comparison() {
    let authentic_artifact = b"compiled-native-artifact";
    let tampered_artifact = b"tampered-native-artifact";
    let manifest = manifest(authentic_artifact);
    let semantic = admitted_semantics();
    let realized = outputs(&semantic);

    assert_eq!(
        check_u8_realization_equivalence(&request(
            &manifest,
            tampered_artifact,
            &semantic,
            &realized,
        )),
        CheckVerdict::Fail(CheckFailure::RealizationArtifactDigestMismatch)
    );
}

#[test]
fn manifest_binding_mismatch_fails_before_output_comparison() {
    let artifact = b"compiled-native-artifact";
    let manifest = RealizationCheckManifest::new(
        d(b"different-semantic-target"),
        d(b"realization-metadata"),
        d(b"generation"),
        d(b"world"),
        d(b"authority-contract"),
        d(b"observer"),
        ArtifactDigest::of_bytes(artifact),
    );
    let semantic = admitted_semantics();
    let realized = outputs(&semantic);

    assert_eq!(
        check_u8_realization_equivalence(&request(
            &manifest,
            artifact,
            &semantic,
            &realized,
        )),
        CheckVerdict::Fail(CheckFailure::RealizationBindingMismatch)
    );
}
