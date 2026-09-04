use formula_core::digest::ArtifactDigest;
use formula_engine::{
    compiler::{CompilerError, CompilerInputs, CompilerV1},
    query::{
        ActivatedPackageBinding, KnownBinding, QueryIR, RequestedResultClass, ResourceContract,
        SideEffectPolicy, TargetRequest,
    },
    region::CompilerAuthoritySnapshot,
    reuse::ReuseRequest,
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn packages() -> ActivatedPackageBinding {
    ActivatedPackageBinding::new(d(1), vec![], vec![])
}

fn query() -> QueryIR {
    QueryIR::new(
        d(1),
        d(2),
        vec![KnownBinding::new("values", d(3))],
        vec![],
        vec![TargetRequest::new(d(4), RequestedResultClass::Count)],
        d(5),
        d(6),
        ResourceContract::new(100, 1024, 50),
        SideEffectPolicy::deny_all(),
        packages(),
    )
}

fn inputs() -> CompilerInputs {
    CompilerInputs::new(d(5), d(6), d(50), d(51))
}

#[test]
fn u1_reuse_compiles_without_candidate_space_or_discovery_work_cell() {
    let primitive = d(40);
    let snapshot = CompilerAuthoritySnapshot::new(
        d(1),
        d(2),
        packages(),
        vec![d(3), primitive],
        vec![primitive],
        vec![],
    );
    let q = query();
    let request = ReuseRequest::new(&q, primitive);
    let compiled = CompilerV1::compile_reuse(&q, &snapshot, inputs(), &request).unwrap();

    assert_eq!(compiled.resolved_capability().primitive(), primitive);
    assert!(compiled.work_cells().is_empty());
    assert_eq!(compiled.execution_plans().len(), 1);
    assert_eq!(compiled.metrics().primitive_discovery_candidate_spaces(), 0);
    assert_eq!(compiled.metrics().primitive_discovery_work_cells(), 0);
    assert_eq!(compiled.metrics().resolved_capability_count(), 1);
    assert_eq!(compiled.metrics().execution_work_items(), 1);
    assert_eq!(compiled.execution_plans()[0].primitive(), primitive);
    assert_eq!(
        compiled.execution_plans()[0].result_class(),
        RequestedResultClass::Count
    );
}

#[test]
fn reuse_fails_closed_when_required_capability_is_unavailable() {
    let primitive = d(40);
    let q = query();
    let request = ReuseRequest::new(&q, primitive);
    let snapshot =
        CompilerAuthoritySnapshot::new(d(1), d(2), packages(), vec![d(3)], vec![], vec![]);

    assert_eq!(
        CompilerV1::compile_reuse(&q, &snapshot, inputs(), &request).unwrap_err(),
        CompilerError::RequiredCapabilityUnavailable
    );
}

#[test]
fn reuse_request_and_authority_context_mismatches_fail_closed() {
    let primitive = d(40);
    let q = query();
    let snapshot = CompilerAuthoritySnapshot::new(
        d(1),
        d(2),
        packages(),
        vec![primitive],
        vec![primitive],
        vec![],
    );

    let other_query = q.clone().with_observer(d(99));
    let mismatched_request = ReuseRequest::new(&other_query, primitive);
    assert_eq!(
        CompilerV1::compile_reuse(&q, &snapshot, inputs(), &mismatched_request).unwrap_err(),
        CompilerError::ReuseRequestMismatch
    );

    assert_eq!(
        CompilerV1::compile_reuse(
            &q,
            &snapshot,
            inputs().with_expected_observer(d(98)),
            &ReuseRequest::new(&q, primitive),
        )
        .unwrap_err(),
        CompilerError::ObserverMismatch
    );

    let wrong_generation = CompilerAuthoritySnapshot::new(
        d(99),
        d(2),
        ActivatedPackageBinding::new(d(99), vec![], vec![]),
        vec![primitive],
        vec![primitive],
        vec![],
    );
    assert_eq!(
        CompilerV1::compile_reuse(
            &q,
            &wrong_generation,
            inputs(),
            &ReuseRequest::new(&q, primitive),
        )
        .unwrap_err(),
        CompilerError::GenerationMismatch
    );
}
