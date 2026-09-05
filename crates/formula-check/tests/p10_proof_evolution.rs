use formula_check::proof_evolution::{
    ProofEvolutionFailure, authorize_repair_v1, authorize_transport_v1, classify_freshness,
    repair_evidence_v1, transport_evidence_v1,
};
use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    self_expansion::{
        EvidenceFreshness, ProofRepairPlan, ProofTransportPlan, SemanticChange, SemanticChangeClass,
    },
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

#[test]
fn unrelated_dependency_change_keeps_evidence_fresh() {
    let change = SemanticChange::new(
        d("p10:old"),
        d("p10:new"),
        SemanticChangeClass::ConservativeExtension,
        vec![d("p10:dep:changed")],
        vec![d("p10:dep:changed")],
        vec![d("p10:relation")],
    );

    assert_eq!(
        classify_freshness(&change, &[d("p10:dep:unrelated")], None),
        EvidenceFreshness::UnchangedFresh
    );
}

#[test]
fn non_conservative_change_never_silently_transports() {
    let relation = d("p10:relation:nonconservative");
    let old = d("p10:old");
    let change = SemanticChange::new(
        old,
        d("p10:new"),
        SemanticChangeClass::NonConservativeChange,
        vec![d("p10:dep")],
        vec![d("p10:dep")],
        vec![relation],
    );

    assert_eq!(
        classify_freshness(&change, &[d("p10:dep")], Some(relation)),
        EvidenceFreshness::ReproveRequired
    );
}

#[test]
fn exact_transport_is_checker_authorized_and_receives_new_identity() {
    let checker = d("p10:checker");
    let source_evidence = d("p10:evidence:source");
    let source_target = d("p10:target:source");
    let destination_target = d("p10:target:destination");
    let relation = d("p10:relation:defeq");
    let dependency = d("p10:dep");
    let change = SemanticChange::new(
        source_target,
        destination_target,
        SemanticChangeClass::DefinitionalEquivalent,
        vec![dependency],
        vec![dependency],
        vec![relation],
    );
    let plan = ProofTransportPlan::new(
        source_evidence,
        source_target,
        destination_target,
        relation,
        vec![dependency],
        checker,
    );

    let authorization =
        authorize_transport_v1(&change, &plan, checker, source_evidence, &[dependency]).unwrap();
    let record = transport_evidence_v1(&authorization, &plan).unwrap();

    assert_eq!(record.destination_target(), destination_target);
    assert_eq!(record.source_evidence(), source_evidence);
    assert_eq!(record.checker(), checker);
    assert_ne!(record.structural_digest(), source_evidence);

    assert_eq!(
        authorize_transport_v1(
            &change,
            &plan,
            d("p10:wrong-checker"),
            source_evidence,
            &[dependency]
        ),
        Err(ProofEvolutionFailure::CheckerMismatch)
    );
}

#[test]
fn repair_record_requires_exact_checker_authorization() {
    let checker = d("p10:repair-checker");
    let source_evidence = d("p10:repair-source");
    let dependency = d("p10:repair-dep");
    let obligation = d("p10:repair-obligation");
    let change = SemanticChange::new(
        d("p10:repair-old"),
        d("p10:repair-new"),
        SemanticChangeClass::TheoremStrengthening,
        vec![dependency],
        vec![dependency],
        vec![d("p10:repair-relation")],
    );
    let plan = ProofRepairPlan::new(
        source_evidence,
        change.structural_digest(),
        vec![dependency],
        vec![obligation],
        checker,
    );

    let authorization =
        authorize_repair_v1(&change, &plan, checker, source_evidence, &[dependency]).unwrap();
    let record = repair_evidence_v1(&authorization, &plan).unwrap();
    assert_eq!(record.source_evidence(), source_evidence);
    assert_eq!(record.semantic_change(), change.structural_digest());
    assert_eq!(record.repair_obligations(), &[obligation]);

    let wrong_plan = ProofRepairPlan::new(
        source_evidence,
        d("p10:wrong-change"),
        vec![dependency],
        vec![obligation],
        checker,
    );
    assert_eq!(
        repair_evidence_v1(&authorization, &wrong_plan),
        Err(ProofEvolutionFailure::AuthorizationMismatch)
    );
}
