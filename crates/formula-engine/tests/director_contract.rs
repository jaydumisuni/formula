use formula_core::digest::ArtifactDigest;
use formula_engine::{
    compiler::{CompilerError, CompilerInputs},
    director::{DirectorError, FormulaMathematicalDirector},
    query::{
        ActivatedPackageBinding, QueryIR, RequestedResultClass, ResourceContract, SideEffectPolicy,
        TargetRequest,
    },
    region::CompilerAuthoritySnapshot,
    reuse::ReuseRequest,
};
fn d(n: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[n; 32])
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
        ResourceContract::new(100, 1024, 50),
        SideEffectPolicy::deny_all(),
        packages(),
    )
}
fn snapshot() -> CompilerAuthoritySnapshot {
    CompilerAuthoritySnapshot::new(d(1), d(2), packages(), vec![d(3)], vec![], vec![])
}
fn inputs() -> CompilerInputs {
    CompilerInputs::new(d(4), d(5), d(6), d(7))
}

#[test]
fn director_emits_deterministic_existing_compiler_work() {
    let a = FormulaMathematicalDirector::plan(&query(), &snapshot(), inputs()).unwrap();
    let b = FormulaMathematicalDirector::plan(&query(), &snapshot(), inputs()).unwrap();
    assert_eq!(a.query_digest(), query().digest());
    assert_eq!(a.manifest_digest(), b.manifest_digest());
    assert_eq!(a.manifest_bytes(), b.manifest_bytes());
    assert_eq!(a.work(), b.work());
    assert_eq!(a.work().len(), a.compiled.work_cells.len());
    for work in a.work() {
        assert!(work.verify_bytes());
        assert_eq!(work.required_authority(), query().authority_contract());
        assert!(
            a.compiled
                .obligations
                .iter()
                .any(|o| o.digest() == work.obligation_digest())
        );
        assert!(
            a.compiled
                .work_cells
                .iter()
                .any(|c| c.digest() == work.plan_digest())
        );
    }
    for cell in &a.compiled.work_cells {
        assert_eq!(cell.side_effect_limits(), SideEffectPolicy::deny_all());
    }
}

#[test]
fn director_keeps_existing_compiler_fail_closed_guards() {
    let changed_world =
        CompilerAuthoritySnapshot::new(d(1), d(90), packages(), vec![], vec![], vec![]);
    assert_eq!(
        FormulaMathematicalDirector::plan(&query(), &changed_world, inputs()).unwrap_err(),
        DirectorError::Compiler(CompilerError::WorldMismatch)
    );
    assert_eq!(
        FormulaMathematicalDirector::plan(
            &query(),
            &snapshot(),
            inputs().with_expected_authority(d(92))
        )
        .unwrap_err(),
        DirectorError::Compiler(CompilerError::AuthorityMismatch)
    );
    assert_eq!(
        FormulaMathematicalDirector::plan(
            &query(),
            &snapshot(),
            inputs().with_expected_observer(d(93))
        )
        .unwrap_err(),
        DirectorError::Compiler(CompilerError::ObserverMismatch)
    );
}

#[test]
fn director_prefers_exact_admitted_reuse_without_rediscovery() {
    let primitive = d(77);
    let q = query().with_targets(vec![TargetRequest::new(d(3), RequestedResultClass::Count)]);
    let admitted = CompilerAuthoritySnapshot::new(
        d(1),
        d(2),
        packages(),
        vec![primitive],
        vec![primitive],
        vec![],
    );
    let request = ReuseRequest::new(&q, primitive);
    let compiled =
        FormulaMathematicalDirector::plan_reuse(&q, &admitted, inputs(), &request).unwrap();
    assert!(compiled.work_cells().is_empty());
    assert_eq!(compiled.metrics().primitive_discovery_work_cells(), 0);
    assert_eq!(compiled.metrics().primitive_discovery_candidate_spaces(), 0);
    assert_eq!(compiled.metrics().resolved_capability_count(), 1);

    let not_admitted = snapshot();
    assert_eq!(
        FormulaMathematicalDirector::plan_reuse(&q, &not_admitted, inputs(), &request).unwrap_err(),
        DirectorError::Compiler(CompilerError::RequiredCapabilityUnavailable)
    );
}

#[test]
fn director_rejects_unavailable_side_effect_privileges() {
    let unsafe_request = query().with_side_effect_policy(SideEffectPolicy::local_process_only());
    assert_eq!(
        FormulaMathematicalDirector::plan(&unsafe_request, &snapshot(), inputs()).unwrap_err(),
        DirectorError::UnauthorizedSideEffects
    );
}

#[test]
fn reuse_cannot_bypass_director_side_effect_policy() {
    let primitive = d(77);
    let q = query().with_targets(vec![TargetRequest::new(d(3), RequestedResultClass::Count)]);
    let snapshot = CompilerAuthoritySnapshot::new(
        d(1),
        d(2),
        packages(),
        vec![primitive],
        vec![primitive],
        vec![],
    );
    let unsafe_query = q.with_side_effect_policy(SideEffectPolicy::local_process_only());
    let request = ReuseRequest::new(&unsafe_query, primitive);
    assert_eq!(
        FormulaMathematicalDirector::plan_reuse(&unsafe_query, &snapshot, inputs(), &request),
        Err(DirectorError::UnauthorizedSideEffects)
    );
}
