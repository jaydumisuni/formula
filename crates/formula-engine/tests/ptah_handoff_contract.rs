use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use formula_engine::{
    compiler::CompilerInputs,
    director::FormulaMathematicalDirector,
    ptah_handoff::{PtahCandidateReceipt, PtahContractError, PtahHandoff},
    query::{
        ActivatedPackageBinding, KnownBinding, QueryIR, RequestedResultClass, ResourceContract,
        SideEffectPolicy, TargetRequest,
    },
    region::CompilerAuthoritySnapshot,
};
use std::collections::BTreeMap;
fn d(n: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[n; 32])
}
fn p9_vector_digest() -> ArtifactDigest {
    let x = [
        0_u8, 1, 2, 3, 4, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128, 129, 255, 1, 3,
    ];
    let v = CanonicalValue::Array(
        x.iter()
            .map(|n| CanonicalValue::Integer((*n).into()))
            .collect(),
    );
    CanonicalValue::Object(BTreeMap::from([
        (
            "schema".into(),
            CanonicalValue::String("formula-p9-canonical-u8-vector-v1".into()),
        ),
        ("values".into(), v),
    ]))
    .digest()
}
fn setup() -> (QueryIR, CompilerAuthoritySnapshot, CompilerInputs) {
    let packages = ActivatedPackageBinding::new(d(1), vec![], vec![]);
    let q = QueryIR::new(
        d(1),
        d(2),
        vec![KnownBinding::new("values", p9_vector_digest())],
        vec![],
        vec![TargetRequest::new(
            ArtifactDigest::of_bytes(b"p9-count-power-of-two-u8"),
            RequestedResultClass::Count,
        )],
        d(4),
        d(5),
        ResourceContract::new(100, 1024, 50),
        SideEffectPolicy::deny_all(),
        packages.clone(),
    );
    let snapshot = CompilerAuthoritySnapshot::new(
        d(1),
        d(2),
        packages,
        vec![p9_vector_digest()],
        vec![],
        vec![],
    );
    (q, snapshot, CompilerInputs::new(d(4), d(5), d(6), d(7)))
}
#[test]
fn handoff_binds_exact_director_plan() {
    let (q, s, inputs) = setup();
    let plan = FormulaMathematicalDirector::plan(&q, &s, inputs).unwrap();
    let packet = PtahHandoff::from_director(&plan, plan.work()[0].plan_digest()).unwrap();
    assert_eq!(packet.manifest_bytes(), plan.manifest_bytes());
    assert_eq!(packet.work_cell_bytes(), plan.work()[0].canonical_bytes());
    assert_eq!(packet.required_authority(), q.authority_contract());
    assert_eq!(
        packet.obligation_digest(),
        plan.compiled.obligations[0].digest()
    );
    assert_eq!(packet.validate(), Ok(()));
    let bytes = packet.wire_bytes();
    let wire = String::from_utf8(bytes).unwrap();
    if std::env::var_os("FORMULA_EXPORT_PTAH_FIXTURE").is_some() {
        println!("FORMULA_WIRE_FIXTURE={wire}");
    }
    assert!(wire.contains("formula-ptah-handoff-v1"));
    assert!(wire.contains("CANDIDATE_ONLY"));
    assert!(wire.contains(&packet.manifest_digest().as_str()));
    assert!(wire.contains(&packet.work_cell_digest().as_str()));
    assert_eq!(
        PtahHandoff::from_director(&plan, d(99)),
        Err(PtahContractError::WorkCellNotFound)
    );
}
#[test]
fn ptah_returns_candidate_only_and_cannot_swap_identity_or_bytes() {
    let (q, s, inputs) = setup();
    let plan = FormulaMathematicalDirector::plan(&q, &s, inputs).unwrap();
    let packet = PtahHandoff::from_director(&plan, plan.work()[0].plan_digest()).unwrap();
    let candidate_bytes = b"unverified-result".to_vec();
    let receipt = PtahCandidateReceipt {
        manifest_digest: packet.manifest_digest(),
        work_cell_digest: packet.work_cell_digest(),
        claimed_digest: ArtifactDigest::of_bytes(&candidate_bytes),
        bytes: candidate_bytes.clone(),
    };
    let unchecked = packet.inspect_candidate(receipt.clone()).unwrap();
    assert_eq!(unchecked.bytes(), candidate_bytes);
    assert_eq!(
        unchecked.digest(),
        ArtifactDigest::of_bytes(&candidate_bytes)
    );
    assert_eq!(
        packet.inspect_candidate(PtahCandidateReceipt {
            manifest_digest: d(99),
            ..receipt.clone()
        }),
        Err(PtahContractError::MismatchedManifest)
    );
    assert_eq!(
        packet.inspect_candidate(PtahCandidateReceipt {
            work_cell_digest: d(99),
            ..receipt.clone()
        }),
        Err(PtahContractError::MismatchedWorkCell)
    );
    assert_eq!(
        packet.inspect_candidate(PtahCandidateReceipt {
            bytes: b"mutated".to_vec(),
            ..receipt
        }),
        Err(PtahContractError::MismatchedCandidateDigest)
    );
}
