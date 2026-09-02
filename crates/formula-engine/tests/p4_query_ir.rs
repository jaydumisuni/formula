use formula_core::digest::ArtifactDigest;
use formula_engine::query::{
    ActivatedPackageBinding, KnownBinding, Metavariable, QueryIR, RequestedResultClass,
    ResourceContract, SideEffectPolicy, TargetRequest,
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn fixture() -> QueryIR {
    QueryIR::new(
        d(1),
        d(2),
        vec![KnownBinding::new("a", d(3)), KnownBinding::new("b", d(4))],
        vec![Metavariable::new("x", "value")],
        vec![TargetRequest::new(d(5), RequestedResultClass::Witness)],
        d(6),
        d(7),
        ResourceContract::new(10_000, 64 * 1024 * 1024, 1_000),
        SideEffectPolicy::deny_all(),
        ActivatedPackageBinding::new(d(1), vec![d(8), d(9)], vec![d(10)]),
    )
}

#[test]
fn identical_semantic_queries_have_identical_identity() {
    let left = fixture();
    let right = QueryIR::new(
        d(1),
        d(2),
        vec![KnownBinding::new("b", d(4)), KnownBinding::new("a", d(3))],
        vec![Metavariable::new("x", "value")],
        vec![TargetRequest::new(d(5), RequestedResultClass::Witness)],
        d(6),
        d(7),
        ResourceContract::new(10_000, 64 * 1024 * 1024, 1_000),
        SideEffectPolicy::deny_all(),
        ActivatedPackageBinding::new(d(1), vec![d(9), d(8), d(8)], vec![d(10)]),
    );

    assert_eq!(left.canonical_bytes(), right.canonical_bytes());
    assert_eq!(left.digest(), right.digest());
}

#[test]
fn every_semantic_query_input_is_identity_binding() {
    let base = fixture().digest();

    let variants = [
        fixture().with_universe_generation(d(11)).digest(),
        fixture().with_world(d(12)).digest(),
        fixture().with_observer(d(13)).digest(),
        fixture().with_authority_contract(d(14)).digest(),
        fixture()
            .with_known_bindings(vec![KnownBinding::new("a", d(30))])
            .digest(),
        fixture()
            .with_targets(vec![TargetRequest::new(d(31), RequestedResultClass::Witness)])
            .digest(),
        fixture()
            .with_resource_contract(ResourceContract::new(20_000, 64 * 1024 * 1024, 1_000))
            .digest(),
        fixture()
            .with_side_effect_policy(SideEffectPolicy::local_process_only())
            .digest(),
        fixture()
            .with_activated_packages(ActivatedPackageBinding::new(d(1), vec![d(32)], vec![]))
            .digest(),
    ];

    for variant in variants {
        assert_ne!(base, variant);
    }
}

#[test]
fn resource_contract_does_not_rewrite_requested_authority() {
    let query = fixture().with_resource_contract(ResourceContract::new(1, 1, 1));
    assert_eq!(query.authority_contract(), d(7));
    assert_eq!(query.requested_result_class(), RequestedResultClass::Witness);
}
