use formula_core::digest::ArtifactDigest;
use formula_engine::director::FormulaMathematicalDirector;
use formula_engine::sup_handoff::{
    SupHandoffError, SupLearningRequest, SupNeuralIntent, UntrustedSupAdvice,
};

fn generation() -> String {
    ArtifactDigest::of_bytes(b"sup-generation-for-qualified-test").as_str()
}
fn intent<'a>(
    instruction: &'a str,
    baseline_metric: &'a str,
    acceptance: &'a [String],
    regressions: &'a [String],
) -> SupNeuralIntent<'a> {
    SupNeuralIntent {
        instruction,
        capability: "Formula advisory intent understanding, not mathematical certification",
        baseline_metric,
        baseline_evidence: ArtifactDigest::of_bytes(b"synthetic-contract-test-baseline"),
        acceptance_criteria: acceptance,
        forbidden_regressions: regressions,
    }
}
fn request() -> SupLearningRequest {
    let a = ["independent Formula math checker remains mandatory".into()];
    let r = ["never grant promotion or execution authority to SUP".into()];
    FormulaMathematicalDirector::request_neural_specialist(
        true,
        &generation(),
        &intent(
            "Investigate whether neural help is required for this bounded user-facing capability",
            "frozen independent mathematical verification baseline",
            &a,
            &r,
        ),
    )
    .unwrap()
    .unwrap()
}

#[test]
fn normal_formula_work_never_needs_sup() {
    assert_eq!(
        SupLearningRequest::optional_neural(false, "absent", &intent("", "", &[], &[])),
        Ok(None)
    );
}

#[test]
fn neural_request_uses_sup_existing_schema_and_is_reproducible() {
    let a = request();
    let b = request();
    assert_eq!(a, b);
    assert_eq!(a.digest(), ArtifactDigest::of_bytes(a.request_bytes()));
    assert_eq!(a.sup_generation().as_str(), generation());
    let wire = std::str::from_utf8(a.request_bytes()).unwrap();
    assert!(wire.contains(r#""target_product":"Formula""#));
    assert!(wire.contains(r#""capability_need":"neural""#));
    assert!(wire.contains(r#""autonomous_deploy":false"#));
    assert!(!wire.contains("source_truth_qualified"));
    assert!(!wire.contains("authority_granted"));
    if std::env::var_os("FORMULA_EXPORT_SUP_FIXTURE").is_some() {
        println!("FORMULA_SUP_REQUEST={wire}");
    }
}

#[test]
fn missing_neural_evidence_never_generates_a_handoff() {
    let criteria = ["checker must independently verify output".to_string()];
    assert_eq!(
        SupLearningRequest::optional_neural(
            true,
            "wrong",
            &intent("learn", "baseline", &criteria, &criteria)
        ),
        Err(SupHandoffError::InvalidSupGeneration)
    );
    assert_eq!(
        SupLearningRequest::optional_neural(
            true,
            &generation(),
            &intent("", "baseline", &criteria, &criteria)
        ),
        Err(SupHandoffError::MissingInstruction)
    );
    assert_eq!(
        SupLearningRequest::optional_neural(
            true,
            &generation(),
            &intent("x", "", &criteria, &criteria)
        ),
        Err(SupHandoffError::MissingBaseline)
    );
    assert_eq!(
        SupLearningRequest::optional_neural(
            true,
            &generation(),
            &intent("x", "baseline", &[], &criteria)
        ),
        Err(SupHandoffError::MissingAcceptanceCriteria)
    );
    assert_eq!(
        SupLearningRequest::optional_neural(
            true,
            &generation(),
            &intent("x", "baseline", &criteria, &[])
        ),
        Err(SupHandoffError::MissingRegressionCriteria)
    );
}

#[test]
fn sup_reply_remains_untrusted_advice_even_when_digest_matches() {
    let r = request();
    let bytes = br#"{"stage":"AWAIT_SOURCE","authority_granted":false}"#.to_vec();
    let digest = ArtifactDigest::of_bytes(&bytes);
    assert_eq!(
        UntrustedSupAdvice::observe(&r, r.digest(), bytes.clone(), digest)
            .unwrap()
            .bytes(),
        bytes
    );
    assert!(
        UntrustedSupAdvice::observe(
            &r,
            ArtifactDigest::of_bytes(b"wrong"),
            bytes.clone(),
            digest
        )
        .is_none()
    );
    assert!(UntrustedSupAdvice::observe(&r, r.digest(), b"mutated".to_vec(), digest).is_none());
}
