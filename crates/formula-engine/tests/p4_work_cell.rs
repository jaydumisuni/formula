use formula_core::digest::ArtifactDigest;
use formula_engine::{
    query::{ResourceContract, SideEffectPolicy},
    work_cell::{CheckpointPolicy, StopCondition, WorkCellPlan},
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn plan() -> WorkCellPlan {
    WorkCellPlan::new(
        d(1),
        vec![d(2), d(3)],
        vec![d(4), d(5)],
        vec![d(6)],
        d(7),
        d(8),
        ResourceContract::new(100, 1024, 50),
        d(9),
        CheckpointPolicy::AtStopBoundary,
        SideEffectPolicy::deny_all(),
        vec![
            StopCondition::Satisfied,
            StopCondition::Refuted,
            StopCondition::ResourceBoundedUnknown,
        ],
    )
}

#[test]
fn work_cell_identity_is_deterministic_for_set_like_inputs() {
    let left = plan();
    let right = WorkCellPlan::new(
        d(1),
        vec![d(3), d(2), d(2)],
        vec![d(5), d(4), d(4)],
        vec![d(6), d(6)],
        d(7),
        d(8),
        ResourceContract::new(100, 1024, 50),
        d(9),
        CheckpointPolicy::AtStopBoundary,
        SideEffectPolicy::deny_all(),
        vec![
            StopCondition::ResourceBoundedUnknown,
            StopCondition::Refuted,
            StopCondition::Satisfied,
        ],
    );

    assert_eq!(left.canonical_bytes(), right.canonical_bytes());
    assert_eq!(left.digest(), right.digest());
}

#[test]
fn every_execution_constraint_is_identity_binding() {
    let base = plan().digest();
    let variants = [
        plan().with_obligation(d(20)).digest(),
        plan().with_semantic_inputs(vec![d(21)]).digest(),
        plan().with_allowed_packages(vec![d(22)]).digest(),
        plan().with_allowed_capabilities(vec![d(23)]).digest(),
        plan().with_evidence_requirement(d(24)).digest(),
        plan().with_required_authority(d(25)).digest(),
        plan()
            .with_resource_budget(ResourceContract::new(200, 1024, 50))
            .digest(),
        plan().with_deterministic_replay_key(d(26)).digest(),
        plan()
            .with_checkpoint_policy(CheckpointPolicy::Never)
            .digest(),
        plan()
            .with_side_effect_limits(SideEffectPolicy::local_process_only())
            .digest(),
        plan()
            .with_stop_conditions(vec![StopCondition::Satisfied])
            .digest(),
    ];

    for variant in variants {
        assert_ne!(base, variant);
    }
}

#[test]
fn work_cell_side_effects_cannot_grant_authority_write() {
    let plan = plan();
    assert!(!plan.side_effect_limits().allows_authority_write());
    assert_eq!(plan.required_authority(), d(8));
}
