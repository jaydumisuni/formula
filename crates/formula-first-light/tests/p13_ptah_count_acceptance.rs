use formula_check::p13::{P13CountError, canonical_p9_vector_digest, verify_p9_count_candidate};
use formula_core::digest::ArtifactDigest;
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
use formula_first_light::reuse::canonical_second_query_vector;

fn d(n: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[n; 32])
}
fn test_packet() -> (PtahHandoff, Vec<u8>, ArtifactDigest) {
    let vector = canonical_second_query_vector();
    let values = vector.values().to_vec();
    assert_eq!(canonical_p9_vector_digest(&values), vector.digest());
    let packages = ActivatedPackageBinding::new(d(1), vec![], vec![]);
    let q = QueryIR::new(
        d(1),
        d(2),
        vec![KnownBinding::new("values", vector.digest())],
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
    let snapshot =
        CompilerAuthoritySnapshot::new(d(1), d(2), packages, vec![vector.digest()], vec![], vec![]);
    let plan = FormulaMathematicalDirector::plan(
        &q,
        &snapshot,
        CompilerInputs::new(d(4), d(5), d(6), d(7)),
    )
    .unwrap();
    let packet = PtahHandoff::from_director(&plan, plan.work()[0].plan_digest()).unwrap();
    assert_eq!(packet.query_digest(), q.digest());
    assert_eq!(packet.query_bytes(), q.canonical_bytes());
    (packet, values, vector.digest())
}

#[test]
fn p13_ptah_executed_candidate_is_independently_checked_by_formula() {
    let (packet, values, input_digest) = test_packet();
    // Exact identities and byte digest observed in KRATOS Ptah E01/E02/A05
    // p13_formula_workcell_executes_over_real_e01_e02_a05_tls_and_checks_count.
    assert_eq!(
        packet.manifest_digest().as_str(),
        "sha256:778e11206defae987198ad7174606ab063a64855a4d0775a10bce66087e2ec07"
    );
    assert_eq!(
        packet.work_cell_digest().as_str(),
        "sha256:d156d75e30f1da5b7fc32e1a5f55fec0f126c0b11ae0cd361ea1b925071a8d09"
    );
    let observed = b"9\n".to_vec();
    let digest = ArtifactDigest::of_bytes(&observed);
    assert_eq!(
        digest.as_str(),
        "sha256:2e6d31a5983a91251bfae5aefa1c0a19d8ba3cf601d0e8a706b4cfa9661a6b8a"
    );
    let receipt = PtahCandidateReceipt {
        manifest_digest: packet.manifest_digest(),
        work_cell_digest: packet.work_cell_digest(),
        claimed_digest: digest,
        bytes: observed,
    };
    // Transport integrity is not a mathematical verdict.
    let candidate = packet.inspect_candidate(receipt.clone()).unwrap();
    let checked =
        verify_p9_count_candidate(&values, input_digest, candidate.bytes(), candidate.digest())
            .unwrap();
    assert_eq!(checked.count(), 9);
    assert_eq!(checked.input_digest(), input_digest);

    // Correct stdout from another campaign cannot satisfy this exact handoff.
    assert_eq!(
        packet.inspect_candidate(PtahCandidateReceipt {
            manifest_digest: d(99),
            ..receipt
        }),
        Err(PtahContractError::MismatchedManifest)
    );
}

#[test]
fn p13_independent_checker_rejects_inexact_or_unbound_remote_candidates() {
    let (_, values, digest) = test_packet();
    let check = |bytes: &[u8], expected: ArtifactDigest| {
        verify_p9_count_candidate(&values, digest, bytes, expected)
    };
    assert_eq!(
        check(b"8\n", ArtifactDigest::of_bytes(b"8\n")),
        Err(P13CountError::WrongCount)
    );
    assert_eq!(
        check(b"09\n", ArtifactDigest::of_bytes(b"09\n")),
        Err(P13CountError::NoncanonicalCount)
    );
    assert_eq!(
        check(b"9\n", ArtifactDigest::of_bytes(b"8\n")),
        Err(P13CountError::CandidateDigestMismatch)
    );
    let mut changed = values.clone();
    changed[0] = 2;
    assert_eq!(
        verify_p9_count_candidate(&changed, digest, b"9\n", ArtifactDigest::of_bytes(b"9\n")),
        Err(P13CountError::InputMismatch)
    );
}
