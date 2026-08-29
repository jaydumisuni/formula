use formula_check::{
    envelope::{validate_envelope, CheckRequest},
    identity::CheckerDescriptor,
    realization::{check_u8_realization_equivalence, RealizationCheckRequest},
    u8::{BoolExpr, ByteExpr},
    verdict::{CheckFailure, CheckVerdict},
};
use formula_core::{
    artifacts::{AuthorityContract, Observer, StructuralIdentity},
    canonical::CanonicalValue,
    certification::{CertificateEnvelope, FrozenCandidate, RealizationCheckManifest},
    digest::ArtifactDigest,
};

fn d(label: &[u8]) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label)
}

fn authority() -> AuthorityContract {
    AuthorityContract::new(
        "deterministic-proof".into(),
        vec!["u8-exhaustive".into()],
        "exact".into(),
    )
}

fn observer() -> Observer {
    Observer::new(
        "full-value".into(),
        CanonicalValue::String("exact".into()),
    )
}

fn candidate(
    semantic: ArtifactDigest,
    authority: &AuthorityContract,
    observer: &Observer,
) -> FrozenCandidate {
    FrozenCandidate::new(
        "semantic-primitive".into(),
        vec![semantic],
        d(b"world"),
        d(b"generation"),
        vec![d(b"dependency-a"), d(b"dependency-b")],
        vec![d(b"target")],
        authority.structural_digest(),
        observer.structural_digest(),
    )
}

#[allow(clippy::too_many_arguments)]
fn envelope(
    candidate: &FrozenCandidate,
    target: ArtifactDigest,
    mode: &str,
    body: &[u8],
    authority: &AuthorityContract,
    observer: &Observer,
    trust_root: ArtifactDigest,
) -> CertificateEnvelope {
    CertificateEnvelope::new(
        candidate.structural_digest(),
        target,
        d(b"generation"),
        d(b"world"),
        CanonicalValue::String("universal".into()),
        "PROVED".into(),
        mode.into(),
        "u8-exhaustive".into(),
        "1".into(),
        ArtifactDigest::of_bytes(body),
        d(b"malicious-producer"),
        CheckerDescriptor::current().identity(),
        trust_root,
        vec![d(b"dependency-a"), d(b"dependency-b")],
        authority.structural_digest(),
        observer.structural_digest(),
        CanonicalValue::String("replay-v1".into()),
    )
}

fn validate<'a>(
    envelope: &'a CertificateEnvelope,
    candidate: &'a FrozenCandidate,
    authority: &'a AuthorityContract,
    observer: &'a Observer,
    body: &'a [u8],
    trust_root: ArtifactDigest,
) -> Result<formula_check::verdict::AuthorityMatch, CheckFailure> {
    validate_envelope(&CheckRequest::new(
        envelope,
        candidate,
        d(b"target"),
        d(b"generation"),
        d(b"world"),
        &[d(b"dependency-a"), d(b"dependency-b")],
        authority,
        observer,
        body,
        trust_root,
    ))
}

fn admitted_semantics() -> BoolExpr {
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

#[test]
fn malicious_producer_cannot_self_certify() {
    let authority = authority();
    let observer = observer();
    let certified = candidate(d(b"semantic-a"), &authority, &observer);
    let mutated = candidate(d(b"semantic-b"), &authority, &observer);
    let trust_root = d(b"trust-root");

    // A producer may claim PASS locally, but the checker owns authority.
    let forged_producer_claim = CheckVerdict::Pass;
    assert_eq!(forged_producer_claim, CheckVerdict::Pass);

    let authentic_body = b"authentic-certificate-body";
    let valid = envelope(
        &certified,
        d(b"target"),
        "EXHAUSTIVE",
        authentic_body,
        &authority,
        &observer,
        trust_root,
    );

    assert_eq!(
        validate(
            &valid,
            &certified,
            &authority,
            &observer,
            b"forged-certificate-body",
            trust_root,
        ),
        Err(CheckFailure::CertificateBodyDigestMismatch)
    );

    let wrong_target = envelope(
        &certified,
        d(b"different-target"),
        "EXHAUSTIVE",
        authentic_body,
        &authority,
        &observer,
        trust_root,
    );
    assert_eq!(
        validate(
            &wrong_target,
            &certified,
            &authority,
            &observer,
            authentic_body,
            trust_root,
        ),
        Err(CheckFailure::TargetMismatch)
    );

    assert_eq!(
        validate(
            &valid,
            &mutated,
            &authority,
            &observer,
            authentic_body,
            trust_root,
        ),
        Err(CheckFailure::FrozenCandidateMismatch)
    );

    for weaker_mode in ["PROBABILISTIC", "EMPIRICAL"] {
        let downgraded = envelope(
            &certified,
            d(b"target"),
            weaker_mode,
            authentic_body,
            &authority,
            &observer,
            trust_root,
        );
        assert_eq!(
            validate(
                &downgraded,
                &certified,
                &authority,
                &observer,
                authentic_body,
                trust_root,
            ),
            Err(CheckFailure::AuthorityInsufficient)
        );
    }

    let compiler_reported_success = true;
    assert!(compiler_reported_success);
    let artifact = b"compiled-native-artifact";
    let realization_manifest = RealizationCheckManifest::new(
        d(b"semantic-target"),
        d(b"realization-metadata"),
        d(b"generation"),
        d(b"world"),
        authority.structural_digest(),
        observer.structural_digest(),
        ArtifactDigest::of_bytes(artifact),
    );
    let semantic = admitted_semantics();
    let realization_request = RealizationCheckRequest::new(
        &realization_manifest,
        d(b"semantic-target"),
        d(b"realization-metadata"),
        d(b"generation"),
        d(b"world"),
        authority.structural_digest(),
        observer.structural_digest(),
        artifact,
        &semantic,
        &[],
    );
    assert_eq!(
        check_u8_realization_equivalence(&realization_request),
        CheckVerdict::Fail(CheckFailure::RealizationOutputCoverageMismatch)
    );
}
