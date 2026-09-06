use formula_check::{
    exact_arithmetic::{ExactArithmeticError, IntegerOperation, check_decimal_integer_result},
    federation_verifier::{
        FederationReplayClaims, FederationReplayEvidence, P11_CANONICAL_MARKERS,
        verify_federation_breadth_manifest,
    },
    sat_lrat::{LratCheckError, SatCnf, check_lrat_rup_unsat},
};
use formula_core::{
    artifacts::StructuralIdentity,
    canonical::CanonicalValue,
    digest::ArtifactDigest,
    federation::BridgeContract,
    federation_proof::{
        FederationBreadthProofManifest, FederationNegativeControl,
        FederationNegativeControlEvidence, FederationNegativeControlManifest, FederationRouteKind,
        FederationRouteProof,
    },
    theory::{
        CompositionClaim, CompositionClass, FactPolarity, FederationAdapterManifest, SharedFact,
    },
};
use formula_first_light::p11::{
    P10_FROZEN_PROOF_HEAD, checker_identity, p10_frozen_proof_identity, source_commit,
    verifier_identity,
};
use formula_packages::{
    cooperation::{CooperationError, apply_bridge, certify_federation_fact},
    federation::{FederationError, FederationMode, FederationRequest, validate_federation_adapter},
};
use num_bigint::BigInt;
use std::str::FromStr;

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn digest_parts(label: &str, parts: &[ArtifactDigest]) -> ArtifactDigest {
    let mut bytes = label.as_bytes().to_vec();
    for part in parts {
        bytes.push(0);
        bytes.extend_from_slice(part.as_str().as_bytes());
    }
    ArtifactDigest::of_bytes(&bytes)
}

fn nc(
    control: FederationNegativeControl,
    label: &str,
    parts: &[ArtifactDigest],
) -> FederationNegativeControlEvidence {
    FederationNegativeControlEvidence::new(control, digest_parts(label, parts))
}

fn n(value: &str) -> BigInt {
    BigInt::from_str(value).expect("valid bigint")
}

#[test]
fn p11_heterogeneous_federation_breadth() {
    assert_eq!(
        P10_FROZEN_PROOF_HEAD,
        "3aeb61daf4d575db0f018245ee271597ad475e7b"
    );
    assert_eq!(
        p10_frozen_proof_identity(),
        ArtifactDigest::of_bytes(P10_FROZEN_PROOF_HEAD.as_bytes())
    );

    let world = d("p11:world");
    let sat_package = d("package:sat-lrat:v1");
    let arithmetic_package = d("package:exact-arithmetic:v1");
    assert_ne!(sat_package, arithmetic_package);

    let cnf = SatCnf::from_dimacs("p cnf 1 2\n1 0\n-1 0\n").unwrap();
    let lrat = check_lrat_rup_unsat(&cnf, "3 0 1 2 0\n").unwrap();
    let sat_subject = d("subject:branch-selection");
    let sat_translation = d("translation:dimacs-to-branch:v1");
    let sat_checker = d("checker:lrat-rup:v1");
    let sat_adapter = FederationAdapterManifest::new(
        "sat-lrat-v1".into(),
        sat_package,
        vec![cnf.structural_digest()],
        vec![sat_subject],
        vec![sat_translation],
        vec![sat_checker],
        vec![],
        vec!["UNSAT_BRANCH".into()],
        true,
    );
    let sat_request = FederationRequest::new(
        "UNSAT_BRANCH".into(),
        true,
        Some(sat_checker),
        Some(sat_translation),
        false,
    );
    let sat_fact = SharedFact::new(
        world,
        sat_subject,
        CanonicalValue::String("BRANCH_A".into()),
        FactPolarity::Exact,
        lrat.evidence_digest(),
    );
    let sat_certified = certify_federation_fact(
        &sat_adapter,
        FederationMode::CheckedResult,
        &sat_request,
        sat_adapter.structural_digest(),
        cnf.structural_digest(),
        lrat.evidence_digest(),
        sat_fact,
    )
    .unwrap();

    let arithmetic_branch_subject = d("subject:arithmetic-branch");
    let bridge = BridgeContract::new(
        sat_package,
        arithmetic_package,
        sat_subject,
        arithmetic_branch_subject,
        FactPolarity::Exact,
        FactPolarity::Exact,
        d("translation:branch-to-arithmetic:v1"),
        d("evidence:branch-bridge:v1"),
    );
    let composition = CompositionClaim::new(
        sat_package,
        arithmetic_package,
        CompositionClass::CertifiedCombination,
        d("evidence:sat-arithmetic-composition:v1"),
    );
    let bridged_branch = apply_bridge(&sat_certified, Some(&bridge), &composition).unwrap();
    assert_eq!(
        bridged_branch.payload(),
        &CanonicalValue::String("BRANCH_A".into())
    );

    let operation = IntegerOperation::Add(
        n("340282366920938463463374607431768211507"),
        n("18446744073709551629"),
    );
    let arithmetic_receipt =
        check_decimal_integer_result(&operation, "340282366920938463481821351505477763136")
            .unwrap();
    let arithmetic_subject = d("subject:arithmetic-result");
    let arithmetic_translation = d("translation:decimal-bigint:v1");
    let arithmetic_checker = d("checker:exact-bigint:v1");
    let arithmetic_adapter = FederationAdapterManifest::new(
        "exact-arithmetic-v1".into(),
        arithmetic_package,
        vec![operation.structural_digest()],
        vec![arithmetic_subject],
        vec![arithmetic_translation],
        vec![arithmetic_checker],
        vec![],
        vec!["EXACT_INTEGER_RESULT".into()],
        true,
    );
    let arithmetic_request = FederationRequest::new(
        "EXACT_INTEGER_RESULT".into(),
        true,
        Some(arithmetic_checker),
        Some(arithmetic_translation),
        false,
    );
    let arithmetic_fact = SharedFact::new(
        world,
        arithmetic_subject,
        CanonicalValue::String(arithmetic_receipt.result_decimal().into()),
        FactPolarity::Exact,
        arithmetic_receipt.evidence_digest(),
    );
    let arithmetic_certified = certify_federation_fact(
        &arithmetic_adapter,
        FederationMode::CheckedResult,
        &arithmetic_request,
        arithmetic_adapter.structural_digest(),
        operation.structural_digest(),
        arithmetic_receipt.evidence_digest(),
        arithmetic_fact,
    )
    .unwrap();

    let final_target = digest_parts(
        "p11:heterogeneous-final-target",
        &[
            bridged_branch.structural_digest(),
            arithmetic_certified.structural_digest(),
        ],
    );
    assert_ne!(final_target, bridged_branch.structural_digest());
    assert_ne!(final_target, arithmetic_certified.structural_digest());

    let mut negatives = Vec::new();

    assert_eq!(
        certify_federation_fact(
            &sat_adapter,
            FederationMode::CandidateOnly,
            &sat_request,
            sat_adapter.structural_digest(),
            cnf.structural_digest(),
            lrat.evidence_digest(),
            SharedFact::new(
                world,
                sat_subject,
                CanonicalValue::String("BRANCH_A".into()),
                FactPolarity::Exact,
                lrat.evidence_digest(),
            ),
        ),
        Err(CooperationError::CandidateOnlyCannotCertify)
    );
    negatives.push(nc(
        FederationNegativeControl::CandidateOnlyAuthorityAttempt,
        "nc11-01",
        &[sat_adapter.structural_digest()],
    ));

    assert_eq!(
        check_lrat_rup_unsat(&cnf, "3 0 1 99 0\n"),
        Err(LratCheckError::UnknownClauseId(99))
    );
    negatives.push(nc(
        FederationNegativeControl::ForgedLratHint,
        "nc11-02",
        &[cnf.structural_digest()],
    ));

    assert_eq!(
        check_lrat_rup_unsat(&cnf, "3 1 0 1 2 0\n"),
        Err(LratCheckError::MissingEmptyClause)
    );
    negatives.push(nc(
        FederationNegativeControl::LratMissingEmptyClause,
        "nc11-03",
        &[cnf.structural_digest()],
    ));

    assert_eq!(
        check_lrat_rup_unsat(&cnf, "3 0 -1 2 0\n"),
        Err(LratCheckError::UnsupportedRatStep)
    );
    negatives.push(nc(
        FederationNegativeControl::UnsupportedRatProofFailsClosed,
        "nc11-04",
        &[cnf.structural_digest()],
    ));

    let wrong_sat_route = FederationRequest::new(
        "UNSAT_BRANCH".into(),
        true,
        Some(d("checker:wrong")),
        Some(sat_translation),
        false,
    );
    assert_eq!(
        certify_federation_fact(
            &sat_adapter,
            FederationMode::CheckedResult,
            &wrong_sat_route,
            sat_adapter.structural_digest(),
            cnf.structural_digest(),
            lrat.evidence_digest(),
            SharedFact::new(
                world,
                sat_subject,
                CanonicalValue::String("BRANCH_A".into()),
                FactPolarity::Exact,
                lrat.evidence_digest(),
            ),
        ),
        Err(CooperationError::Adapter(
            FederationError::CheckerRouteMismatch
        ))
    );
    negatives.push(nc(
        FederationNegativeControl::WrongSatCheckerRoute,
        "nc11-05",
        &[sat_adapter.structural_digest()],
    ));

    assert_eq!(
        check_decimal_integer_result(&operation, "0"),
        Err(ExactArithmeticError::IncorrectResult)
    );
    negatives.push(nc(
        FederationNegativeControl::IncorrectExactArithmeticResult,
        "nc11-06",
        &[operation.structural_digest()],
    ));

    assert_eq!(
        check_decimal_integer_result(&operation, "+1"),
        Err(ExactArithmeticError::MalformedDecimal)
    );
    negatives.push(nc(
        FederationNegativeControl::MalformedExactArithmeticDecimal,
        "nc11-07",
        &[operation.structural_digest()],
    ));

    let wrong_arithmetic_translation = FederationRequest::new(
        "EXACT_INTEGER_RESULT".into(),
        true,
        Some(arithmetic_checker),
        Some(d("translation:wrong")),
        false,
    );
    assert_eq!(
        certify_federation_fact(
            &arithmetic_adapter,
            FederationMode::CheckedResult,
            &wrong_arithmetic_translation,
            arithmetic_adapter.structural_digest(),
            operation.structural_digest(),
            arithmetic_receipt.evidence_digest(),
            SharedFact::new(
                world,
                arithmetic_subject,
                CanonicalValue::String(arithmetic_receipt.result_decimal().into()),
                FactPolarity::Exact,
                arithmetic_receipt.evidence_digest(),
            ),
        ),
        Err(CooperationError::Adapter(
            FederationError::TranslationMismatch
        ))
    );
    negatives.push(nc(
        FederationNegativeControl::WrongArithmeticTranslation,
        "nc11-08",
        &[arithmetic_adapter.structural_digest()],
    ));

    assert_eq!(
        certify_federation_fact(
            &arithmetic_adapter,
            FederationMode::CheckedResult,
            &arithmetic_request,
            arithmetic_adapter.structural_digest(),
            d("stale:semantic-input"),
            arithmetic_receipt.evidence_digest(),
            SharedFact::new(
                world,
                arithmetic_subject,
                CanonicalValue::String(arithmetic_receipt.result_decimal().into()),
                FactPolarity::Exact,
                arithmetic_receipt.evidence_digest(),
            ),
        ),
        Err(CooperationError::SemanticInputMismatch)
    );
    negatives.push(nc(
        FederationNegativeControl::StaleSemanticInputDigest,
        "nc11-09",
        &[operation.structural_digest()],
    ));

    let over_fact = SharedFact::new(
        world,
        sat_subject,
        CanonicalValue::String("BRANCH_A".into()),
        FactPolarity::OverApproximation,
        lrat.evidence_digest(),
    );
    let over_certified = certify_federation_fact(
        &sat_adapter,
        FederationMode::CheckedResult,
        &sat_request,
        sat_adapter.structural_digest(),
        cnf.structural_digest(),
        lrat.evidence_digest(),
        over_fact,
    )
    .unwrap();
    let strengthening_bridge = BridgeContract::new(
        sat_package,
        arithmetic_package,
        sat_subject,
        arithmetic_branch_subject,
        FactPolarity::OverApproximation,
        FactPolarity::Exact,
        d("translation:unsafe-strengthening"),
        d("evidence:unsafe-strengthening"),
    );
    assert_eq!(
        apply_bridge(&over_certified, Some(&strengthening_bridge), &composition),
        Err(CooperationError::PolarityUpgrade)
    );
    negatives.push(nc(
        FederationNegativeControl::SharedFactPolarityUpgrade,
        "nc11-10",
        &[over_certified.structural_digest()],
    ));

    assert_eq!(
        apply_bridge(&sat_certified, None, &composition),
        Err(CooperationError::MissingBridgeContract)
    );
    negatives.push(nc(
        FederationNegativeControl::MissingBridgeContract,
        "nc11-11",
        &[sat_certified.structural_digest()],
    ));

    let reverse_bridge = BridgeContract::new(
        arithmetic_package,
        sat_package,
        arithmetic_branch_subject,
        sat_subject,
        FactPolarity::Exact,
        FactPolarity::Exact,
        d("translation:reverse"),
        d("evidence:reverse"),
    );
    assert_eq!(
        apply_bridge(&sat_certified, Some(&reverse_bridge), &composition),
        Err(CooperationError::BridgeSourcePackageMismatch)
    );
    negatives.push(nc(
        FederationNegativeControl::WrongBridgeDirection,
        "nc11-12",
        &[reverse_bridge.structural_digest()],
    ));

    let unsafe_composition = CompositionClaim::new(
        sat_package,
        arithmetic_package,
        CompositionClass::HeuristicOnly,
        d("evidence:heuristic-composition"),
    );
    assert_eq!(
        apply_bridge(&sat_certified, Some(&bridge), &unsafe_composition),
        Err(CooperationError::UnsafeCompositionClass)
    );
    negatives.push(nc(
        FederationNegativeControl::UnsafeCompositionClass,
        "nc11-13",
        &[unsafe_composition.structural_digest()],
    ));

    let producer_identity_request =
        FederationRequest::new("UNSAT_BRANCH".into(), true, None, None, false);
    assert_eq!(
        validate_federation_adapter(
            &sat_adapter,
            FederationMode::CandidateOnly,
            &producer_identity_request,
        ),
        Err(FederationError::CandidateOnlyCannotAuthorize)
    );
    negatives.push(nc(
        FederationNegativeControl::ProducerIdentityCannotAuthorize,
        "nc11-14",
        &[sat_package, sat_adapter.structural_digest()],
    ));

    let negative_controls = FederationNegativeControlManifest::new(negatives).unwrap();
    assert!(negative_controls.is_complete());

    let sat_route = FederationRouteProof::new(
        FederationRouteKind::SatLrat,
        sat_package,
        sat_adapter.structural_digest(),
        cnf.structural_digest(),
        lrat.evidence_digest(),
        sat_certified.structural_digest(),
    );
    let arithmetic_route = FederationRouteProof::new(
        FederationRouteKind::ExactArithmetic,
        arithmetic_package,
        arithmetic_adapter.structural_digest(),
        operation.structural_digest(),
        arithmetic_receipt.evidence_digest(),
        arithmetic_certified.structural_digest(),
    );
    let manifest = FederationBreadthProofManifest::new(
        source_commit().into(),
        p10_frozen_proof_identity(),
        world,
        sat_route.clone(),
        arithmetic_route.clone(),
        bridge.structural_digest(),
        composition.structural_digest(),
        bridged_branch.structural_digest(),
        final_target,
        negative_controls.clone(),
        checker_identity(),
        verifier_identity(),
    );
    let replay = FederationReplayEvidence::new(
        manifest.structural_digest(),
        p10_frozen_proof_identity(),
        world,
        sat_route,
        arithmetic_route,
        bridge.structural_digest(),
        composition.structural_digest(),
        bridged_branch.structural_digest(),
        final_target,
        arithmetic_certified.structural_digest(),
        sat_package,
        arithmetic_package,
        negative_controls,
        checker_identity(),
        verifier_identity(),
        FederationReplayClaims::all_proved(),
    );

    let verification = verify_federation_breadth_manifest(&manifest, &replay).unwrap();
    assert_eq!(verification.markers(), &P11_CANONICAL_MARKERS);

    println!("P11_SOURCE_SHA={}", source_commit());
    println!("P11_P10_PREDECESSOR={}", p10_frozen_proof_identity());
    println!("P11_MANIFEST={}", manifest.structural_digest());
    println!("P11_SAT_PACKAGE={}", sat_package);
    println!("P11_SAT_ADAPTER={}", sat_adapter.structural_digest());
    println!("P11_ARITHMETIC_PACKAGE={}", arithmetic_package);
    println!(
        "P11_ARITHMETIC_ADAPTER={}",
        arithmetic_adapter.structural_digest()
    );
    println!("P11_BRIDGE={}", bridge.structural_digest());
    println!(
        "P11_NEGATIVE_CONTROLS={}",
        manifest.negative_controls().structural_digest()
    );
    println!("P11_FINAL_TARGET={final_target}");
    for marker in P11_CANONICAL_MARKERS {
        println!("{marker}");
    }
}
