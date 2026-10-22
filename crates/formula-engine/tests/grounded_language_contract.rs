use formula_core::digest::ArtifactDigest;
use formula_engine::{
    compiler::CompilerInputs,
    director::FormulaMathematicalDirector,
    grounded_language::{GroundedPhrase, GroundedPlanningError, GroundingError, resolve},
    query::{
        ActivatedPackageBinding, QueryIR, RequestedResultClass, ResourceContract, SideEffectPolicy,
        TargetRequest,
    },
    region::CompilerAuthoritySnapshot,
};
fn d(n: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[n; 32])
}
fn setup() -> (QueryIR, CompilerAuthoritySnapshot, CompilerInputs) {
    let p = ActivatedPackageBinding::new(d(1), vec![d(30)], vec![]);
    let q = QueryIR::new(
        d(1),
        d(2),
        vec![],
        vec![],
        vec![TargetRequest::new(d(3), RequestedResultClass::Count)],
        d(4),
        d(5),
        ResourceContract::new(100, 1024, 50),
        SideEffectPolicy::deny_all(),
        p.clone(),
    );
    let snapshot = CompilerAuthoritySnapshot::new(d(1), d(2), p, vec![d(3)], vec![], vec![]);
    (q, snapshot, CompilerInputs::new(d(4), d(5), d(6), d(7)))
}
#[test]
fn pregrounded_text_reuses_existing_compiler_without_adding_authority() {
    let (q, s, inputs) = setup();
    let ground =
        GroundedPhrase::new("count powers of two in values", q.clone(), Some(d(99))).unwrap();
    assert_eq!(ground.evidence(), d(99));
    assert_eq!(
        resolve(
            "  COUNT powers of two in values? ",
            std::slice::from_ref(&ground)
        )
        .unwrap(),
        q
    );
    let planned = FormulaMathematicalDirector::plan_grounded_text(
        "count powers of two in values",
        std::slice::from_ref(&ground),
        &s,
        inputs.clone(),
    )
    .unwrap();
    assert_eq!(planned.query_digest(), q.digest());
    assert_eq!(planned.work().len(), 1);
    let original = FormulaMathematicalDirector::plan(&q, &s, inputs).unwrap();
    assert_eq!(planned.manifest_digest(), original.manifest_digest());
    assert_eq!(planned.work(), original.work());
}
#[test]
fn ungrounded_or_ambiguous_text_is_never_compiled() {
    let (q, s, inputs) = setup();
    let b = GroundedPhrase::new("count powers of two in values", q.clone(), Some(d(99))).unwrap();
    for text in [
        "count unlisted domain values",
        "count powers of two in changed values",
        "count powers of two in values and delete source",
        "count powers of two in values!",
    ] {
        assert_eq!(
            resolve(text, std::slice::from_ref(&b)),
            Err(GroundingError::UnknownMeaning)
        );
    }
    assert_eq!(
        resolve("count powers of two in values", &[b.clone(), b.clone()]),
        Err(GroundingError::AmbiguousMeaning)
    );
    assert_eq!(
        GroundedPhrase::new("count powers of two in values", q.clone(), None),
        Err(GroundingError::MissingEvidence)
    );
    assert_eq!(
        GroundedPhrase::new("find powers of two in values", q.clone(), Some(d(9))),
        Err(GroundingError::ClassMismatch)
    );
    let other = CompilerAuthoritySnapshot::new(
        d(1),
        d(77),
        q.activated_packages().clone(),
        vec![],
        vec![],
        vec![],
    );
    assert!(matches!(
        FormulaMathematicalDirector::plan_grounded_text(
            "count powers of two in values",
            &[b],
            &other,
            inputs
        ),
        Err(GroundedPlanningError::Compiler(_))
    ));
    let _ = s;
}
