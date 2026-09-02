use formula_core::digest::ArtifactDigest;
use formula_engine::{
    compiler::{CompilerError, CompilerInputs, CompilerV1, ImplicitMorphism},
    decomposition::{AggregationSemantics, ChildObligation, Decomposition, DecompositionError},
    obligation::{ObligationOutcome, TerminalState},
    query::{
        ActivatedPackageBinding, QueryIR, RequestedResultClass, ResourceContract, SideEffectPolicy,
        TargetRequest,
    },
    reduction::{ReductionEdge, ReductionError, compose_reduction_path},
    region::{CompilerAuthoritySnapshot, RelevantRegion},
    replay::ReplayManifest,
    representation::InformationLoss,
    theory_profile::{OperationalEstimate, TheoryProfile},
    work_cell::{CheckpointPolicy, StopCondition, WorkCellPlan},
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn packages() -> ActivatedPackageBinding {
    ActivatedPackageBinding::new(d(1), vec![d(30)], vec![])
}

fn query() -> QueryIR {
    QueryIR::new(
        d(1),
        d(2),
        vec![],
        vec![],
        vec![TargetRequest::new(d(3), RequestedResultClass::Witness)],
        d(4),
        d(5),
        ResourceContract::new(10, 1024, 10),
        SideEffectPolicy::deny_all(),
        packages(),
    )
}

fn snapshot() -> CompilerAuthoritySnapshot {
    CompilerAuthoritySnapshot::new(d(1), d(2), packages(), vec![d(3)], vec![], vec![])
}

fn compiler_inputs() -> CompilerInputs {
    CompilerInputs::new(d(4), d(5), d(6), d(7))
}

#[test]
fn resource_exhaustion_cannot_become_refutation() {
    let outcome = ObligationOutcome::resource_exhausted(d(10));
    assert_eq!(outcome.state(), TerminalState::ResourceBoundedUnknown);
    assert_ne!(outcome.state(), TerminalState::Refuted);
}

#[test]
fn operational_estimate_cannot_discharge_exact_profile_fact() {
    let q = query();
    let region = RelevantRegion::from_snapshot(&q, &snapshot()).unwrap();
    let profile = TheoryProfile::compile(
        &region,
        &[],
        &[OperationalEstimate::new("finite", u64::MAX)],
    );
    assert!(!profile.satisfies_exact_property("finite"));
}

#[test]
fn lossy_implicit_morphism_is_rejected() {
    let inputs = compiler_inputs().with_implicit_morphisms(vec![ImplicitMorphism::new(
        d(20),
        InformationLoss::Declared,
        false,
    )]);
    assert_eq!(
        CompilerV1::compile(&query(), &snapshot(), inputs).unwrap_err(),
        CompilerError::ImplicitLossyMorphism
    );
}

#[test]
fn decision_only_reduction_cannot_serve_witness() {
    let edge = ReductionEdge::new(
        "source",
        "target",
        vec![RequestedResultClass::Decision],
        d(21),
        None,
        vec![],
        Some(d(22)),
    );
    assert_eq!(
        compose_reduction_path(&[edge], RequestedResultClass::Witness).unwrap_err(),
        ReductionError::RequestedResultNotPreserved
    );
}

#[test]
fn decomposition_without_reconstruction_is_rejected() {
    let decomposition = Decomposition::new(
        d(23),
        d(2),
        vec![ChildObligation::new(d(24), d(2))],
        None,
        Some(AggregationSemantics::And),
        None,
        Some(d(25)),
    );
    assert_eq!(
        decomposition.validate().unwrap_err(),
        DecompositionError::MissingReconstruction
    );
}

#[test]
fn work_cell_cannot_request_authority_write() {
    let plan = WorkCellPlan::new(
        d(26),
        vec![d(27)],
        vec![d(28)],
        vec![],
        d(29),
        d(5),
        ResourceContract::new(10, 1024, 10),
        d(31),
        CheckpointPolicy::AtStopBoundary,
        SideEffectPolicy::local_process_only(),
        vec![StopCondition::Satisfied],
    );
    assert!(!plan.side_effect_limits().allows_authority_write());
}

#[test]
fn replay_canonical_form_cannot_omit_frozen_policy_fields() {
    let replay = ReplayManifest::new(
        d(1),
        d(2),
        d(3),
        d(4),
        d(5),
        d(6),
        "compiler-v1",
        "scheduler-v1",
        ResourceContract::new(10, 1024, 10),
        d(7),
        d(8),
    );
    let text = String::from_utf8(replay.canonical_bytes()).unwrap();
    assert!(text.contains("compiler_policy_version"));
    assert!(text.contains("scheduler_policy_version"));
    assert!(text.contains("resource_contract"));
    assert!(text.contains("random_key"));
    assert!(text.contains("campaign_digest"));
}
