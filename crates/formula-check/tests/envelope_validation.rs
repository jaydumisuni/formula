use formula_check::{
    envelope::{validate_envelope, CheckRequest},
    identity::CheckerDescriptor,
    verdict::CheckFailure,
};
use formula_core::{
    artifacts::{AuthorityContract, Observer, StructuralIdentity},
    canonical::CanonicalValue,
    certification::{CertificateEnvelope, FrozenCandidate},
    digest::ArtifactDigest,
};

fn d(label: &[u8]) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label)
}

fn authority(family: &str, exactness: &str) -> AuthorityContract {
    AuthorityContract::new(
        "deterministic-proof".into(),
        vec![family.into()],
        exactness.into(),
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
    generation: ArtifactDigest,
    world: ArtifactDigest,
    dependencies: Vec<ArtifactDigest>,
    family: &str,
    family_version: &str,
    mode: &str,
    body: &[u8],
    checker: ArtifactDigest,
    trust_root: ArtifactDigest,
    authority: &AuthorityContract,
    observer: &Observer,
) -> CertificateEnvelope {
    CertificateEnvelope::new(
        candidate.structural_digest(),
        target,
        generation,
        world,
        CanonicalValue::String("universal".into()),
        "PROVED".into(),
        mode.into(),
        family.into(),
        family_version.into(),
        ArtifactDigest::of_bytes(body),
        d(b"producer"),
        checker,
        trust_root,
        dependencies,
        authority.structural_digest(),
        observer.structural_digest(),
        CanonicalValue::String("replay-v1".into()),
    )
}

fn validate<'a>(
    envelope: &'a CertificateEnvelope,
    candidate: &'a FrozenCandidate,
    dependencies: &'a [ArtifactDigest],
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
        dependencies,
        authority,
        observer,
        body,
        trust_root,
    ))
}

#[test]
fn allowed_exact_exhaustive_evidence_satisfies_exact_contract() {
    let authority = authority("u8-exhaustive", "exact");
    let observer = observer();
    let candidate = candidate(d(b"semantic-a"), &authority, &observer);
    let dependencies = [d(b"dependency-b"), d(b"dependency-a")];
    let descriptor = CheckerDescriptor::current();
    let body = b"certificate-body";
    let trust_root = d(b"trust-root");
    let envelope = envelope(
        &candidate,
        d(b"target"),
        d(b"generation"),
        d(b"world"),
        dependencies.to_vec(),
        "u8-exhaustive",
        "1",
        "EXHAUSTIVE",
        body,
        descriptor.identity(),
        trust_root,
        &authority,
        &observer,
    );

    let matched = validate(
        &envelope,
        &candidate,
        &dependencies,
        &authority,
        &observer,
        body,
        trust_root,
    )
    .expect("exact exhaustive evidence must satisfy the exact contract");
    assert_eq!(matched.evidence_family(), "u8-exhaustive");
    assert_eq!(matched.exactness(), "exact");
}

#[test]
fn forged_certificate_body_digest_is_rejected() {
    let authority = authority("u8-exhaustive", "exact");
    let observer = observer();
    let candidate = candidate(d(b"semantic-a"), &authority, &observer);
    let dependencies = [d(b"dependency-a"), d(b"dependency-b")];
    let descriptor = CheckerDescriptor::current();
    let trust_root = d(b"trust-root");
    let envelope = envelope(
        &candidate,
        d(b"target"),
        d(b"generation"),
        d(b"world"),
        dependencies.to_vec(),
        "u8-exhaustive",
        "1",
        "EXHAUSTIVE",
        b"authentic-body",
        descriptor.identity(),
        trust_root,
        &authority,
        &observer,
    );

    assert_eq!(
        validate(
            &envelope,
            &candidate,
            &dependencies,
            &authority,
            &observer,
            b"forged-body",
            trust_root,
        ),
        Err(CheckFailure::CertificateBodyDigestMismatch)
    );
}

#[test]
fn mismatched_target_digest_is_rejected() {
    let authority = authority("u8-exhaustive", "exact");
    let observer = observer();
    let candidate = candidate(d(b"semantic-a"), &authority, &observer);
    let dependencies = [d(b"dependency-a"), d(b"dependency-b")];
    let descriptor = CheckerDescriptor::current();
    let body = b"body";
    let trust_root = d(b"trust-root");
    let envelope = envelope(
        &candidate,
        d(b"different-target"),
        d(b"generation"),
        d(b"world"),
        dependencies.to_vec(),
        "u8-exhaustive",
        "1",
        "EXHAUSTIVE",
        body,
        descriptor.identity(),
        trust_root,
        &authority,
        &observer,
    );

    assert_eq!(
        validate(
            &envelope,
            &candidate,
            &dependencies,
            &authority,
            &observer,
            body,
            trust_root,
        ),
        Err(CheckFailure::TargetMismatch)
    );
}

#[test]
fn mismatched_world_or_generation_is_rejected() {
    let authority = authority("u8-exhaustive", "exact");
    let observer = observer();
    let candidate = candidate(d(b"semantic-a"), &authority, &observer);
    let dependencies = [d(b"dependency-a"), d(b"dependency-b")];
    let descriptor = CheckerDescriptor::current();
    let body = b"body";
    let trust_root = d(b"trust-root");

    let wrong_world = envelope(
        &candidate,
        d(b"target"),
        d(b"generation"),
        d(b"different-world"),
        dependencies.to_vec(),
        "u8-exhaustive",
        "1",
        "EXHAUSTIVE",
        body,
        descriptor.identity(),
        trust_root,
        &authority,
        &observer,
    );
    assert_eq!(
        validate(
            &wrong_world,
            &candidate,
            &dependencies,
            &authority,
            &observer,
            body,
            trust_root,
        ),
        Err(CheckFailure::WorldMismatch)
    );

    let wrong_generation = envelope(
        &candidate,
        d(b"target"),
        d(b"different-generation"),
        d(b"world"),
        dependencies.to_vec(),
        "u8-exhaustive",
        "1",
        "EXHAUSTIVE",
        body,
        descriptor.identity(),
        trust_root,
        &authority,
        &observer,
    );
    assert_eq!(
        validate(
            &wrong_generation,
            &candidate,
            &dependencies,
            &authority,
            &observer,
            body,
            trust_root,
        ),
        Err(CheckFailure::GenerationMismatch)
    );
}

#[test]
fn checker_identity_or_version_mismatch_is_rejected() {
    let authority = authority("u8-exhaustive", "exact");
    let observer = observer();
    let candidate = candidate(d(b"semantic-a"), &authority, &observer);
    let dependencies = [d(b"dependency-a"), d(b"dependency-b")];
    let body = b"body";
    let trust_root = d(b"trust-root");

    let wrong_checker = envelope(
        &candidate,
        d(b"target"),
        d(b"generation"),
        d(b"world"),
        dependencies.to_vec(),
        "u8-exhaustive",
        "1",
        "EXHAUSTIVE",
        body,
        d(b"formula-check-version-0"),
        trust_root,
        &authority,
        &observer,
    );
    assert_eq!(
        validate(
            &wrong_checker,
            &candidate,
            &dependencies,
            &authority,
            &observer,
            body,
            trust_root,
        ),
        Err(CheckFailure::CheckerIdentityMismatch)
    );

    let wrong_family_version = envelope(
        &candidate,
        d(b"target"),
        d(b"generation"),
        d(b"world"),
        dependencies.to_vec(),
        "u8-exhaustive",
        "999",
        "EXHAUSTIVE",
        body,
        CheckerDescriptor::current().identity(),
        trust_root,
        &authority,
        &observer,
    );
    assert_eq!(
        validate(
            &wrong_family_version,
            &candidate,
            &dependencies,
            &authority,
            &observer,
            body,
            trust_root,
        ),
        Err(CheckFailure::UnsupportedCertificateFamilyVersion)
    );
}

#[test]
fn changed_candidate_after_envelope_creation_is_rejected() {
    let authority = authority("u8-exhaustive", "exact");
    let observer = observer();
    let certified_candidate = candidate(d(b"semantic-a"), &authority, &observer);
    let changed_candidate = candidate(d(b"semantic-b"), &authority, &observer);
    let dependencies = [d(b"dependency-a"), d(b"dependency-b")];
    let body = b"body";
    let trust_root = d(b"trust-root");
    let envelope = envelope(
        &certified_candidate,
        d(b"target"),
        d(b"generation"),
        d(b"world"),
        dependencies.to_vec(),
        "u8-exhaustive",
        "1",
        "EXHAUSTIVE",
        body,
        CheckerDescriptor::current().identity(),
        trust_root,
        &authority,
        &observer,
    );

    assert_eq!(
        validate(
            &envelope,
            &changed_candidate,
            &dependencies,
            &authority,
            &observer,
            body,
            trust_root,
        ),
        Err(CheckFailure::FrozenCandidateMismatch)
    );
}

#[test]
fn missing_dependency_binding_is_rejected() {
    let authority = authority("u8-exhaustive", "exact");
    let observer = observer();
    let candidate = candidate(d(b"semantic-a"), &authority, &observer);
    let envelope_dependencies = [d(b"dependency-a")];
    let expected_dependencies = [d(b"dependency-a"), d(b"dependency-b")];
    let body = b"body";
    let trust_root = d(b"trust-root");
    let envelope = envelope(
        &candidate,
        d(b"target"),
        d(b"generation"),
        d(b"world"),
        envelope_dependencies.to_vec(),
        "u8-exhaustive",
        "1",
        "EXHAUSTIVE",
        body,
        CheckerDescriptor::current().identity(),
        trust_root,
        &authority,
        &observer,
    );

    assert_eq!(
        validate(
            &envelope,
            &candidate,
            &expected_dependencies,
            &authority,
            &observer,
            body,
            trust_root,
        ),
        Err(CheckFailure::DependencyMismatch)
    );
}

#[test]
fn strict_deterministic_contract_rejects_probabilistic_or_empirical_evidence() {
    let authority = authority("u8-exhaustive", "exact");
    let observer = observer();
    let candidate = candidate(d(b"semantic-a"), &authority, &observer);
    let dependencies = [d(b"dependency-a"), d(b"dependency-b")];
    let body = b"body";
    let trust_root = d(b"trust-root");

    for mode in ["PROBABILISTIC", "EMPIRICAL"] {
        let envelope = envelope(
            &candidate,
            d(b"target"),
            d(b"generation"),
            d(b"world"),
            dependencies.to_vec(),
            "u8-exhaustive",
            "1",
            mode,
            body,
            CheckerDescriptor::current().identity(),
            trust_root,
            &authority,
            &observer,
        );

        assert_eq!(
            validate(
                &envelope,
                &candidate,
                &dependencies,
                &authority,
                &observer,
                body,
                trust_root,
            ),
            Err(CheckFailure::AuthorityInsufficient)
        );
    }
}
