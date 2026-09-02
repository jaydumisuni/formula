use formula_core::digest::ArtifactDigest;
use formula_engine::{
    query::{
        ActivatedPackageBinding, QueryIR, RequestedResultClass, ResourceContract,
        SideEffectPolicy, TargetRequest,
    },
    region::{CompilerAuthoritySnapshot, RegionError, RelevantRegion},
    theory_profile::{OperationalEstimate, ProfileFact, TheoryProfile},
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn packages() -> ActivatedPackageBinding {
    ActivatedPackageBinding::new(d(1), vec![d(8), d(9)], vec![d(10)])
}

fn query() -> QueryIR {
    QueryIR::new(
        d(1),
        d(2),
        vec![],
        vec![],
        vec![TargetRequest::new(d(5), RequestedResultClass::Decision)],
        d(6),
        d(7),
        ResourceContract::new(100, 1024, 50),
        SideEffectPolicy::deny_all(),
        packages(),
    )
}

fn snapshot() -> CompilerAuthoritySnapshot {
    CompilerAuthoritySnapshot::new(
        d(1),
        d(2),
        packages(),
        vec![d(30), d(31)],
        vec![d(40), d(41)],
        vec![d(50)],
    )
}

#[test]
fn region_is_deterministic_projection_of_exact_authority_snapshot() {
    let left = RelevantRegion::from_snapshot(&query(), &snapshot()).unwrap();
    let right = RelevantRegion::from_snapshot(
        &query(),
        &CompilerAuthoritySnapshot::new(
            d(1),
            d(2),
            packages(),
            vec![d(31), d(30), d(30)],
            vec![d(41), d(40)],
            vec![d(50)],
        ),
    )
    .unwrap();

    assert_eq!(left.digest(), right.digest());
    assert_eq!(left.universe_generation(), d(1));
    assert_eq!(left.world(), d(2));
}

#[test]
fn region_rejects_context_mismatch() {
    let wrong_generation = CompilerAuthoritySnapshot::new(
        d(99), d(2), packages(), vec![], vec![], vec![],
    );
    assert_eq!(
        RelevantRegion::from_snapshot(&query(), &wrong_generation),
        Err(RegionError::GenerationMismatch)
    );

    let wrong_world = CompilerAuthoritySnapshot::new(
        d(1), d(98), packages(), vec![], vec![], vec![],
    );
    assert_eq!(
        RelevantRegion::from_snapshot(&query(), &wrong_world),
        Err(RegionError::WorldMismatch)
    );

    let wrong_packages = CompilerAuthoritySnapshot::new(
        d(1),
        d(2),
        ActivatedPackageBinding::new(d(1), vec![d(88)], vec![]),
        vec![],
        vec![],
        vec![],
    );
    assert_eq!(
        RelevantRegion::from_snapshot(&query(), &wrong_packages),
        Err(RegionError::PackageContextMismatch)
    );
}

#[test]
fn theory_profile_keeps_certified_facts_separate_from_estimates() {
    let region = RelevantRegion::from_snapshot(&query(), &snapshot()).unwrap();
    let profile = TheoryProfile::compile(
        &region,
        &[ProfileFact::new("finite", d(60)), ProfileFact::new("exact", d(61))],
        &[
            OperationalEstimate::new("likely_sparse", 900),
            OperationalEstimate::new("finite", 999),
        ],
    );

    assert!(profile.satisfies_exact_property("finite"));
    assert!(profile.satisfies_exact_property("exact"));
    assert!(!profile.satisfies_exact_property("likely_sparse"));
    assert_eq!(profile.operational_estimates().len(), 2);

    let estimates_only = TheoryProfile::compile(
        &region,
        &[],
        &[OperationalEstimate::new("finite", 1000)],
    );
    assert!(!estimates_only.satisfies_exact_property("finite"));
}

#[test]
fn theory_profile_identity_is_order_independent_but_semantic() {
    let region = RelevantRegion::from_snapshot(&query(), &snapshot()).unwrap();
    let left = TheoryProfile::compile(
        &region,
        &[ProfileFact::new("a", d(60)), ProfileFact::new("b", d(61))],
        &[OperationalEstimate::new("x", 1), OperationalEstimate::new("y", 2)],
    );
    let right = TheoryProfile::compile(
        &region,
        &[ProfileFact::new("b", d(61)), ProfileFact::new("a", d(60))],
        &[OperationalEstimate::new("y", 2), OperationalEstimate::new("x", 1)],
    );
    assert_eq!(left.digest(), right.digest());

    let changed = TheoryProfile::compile(
        &region,
        &[ProfileFact::new("a", d(60))],
        &[OperationalEstimate::new("x", 1)],
    );
    assert_ne!(left.digest(), changed.digest());
}
