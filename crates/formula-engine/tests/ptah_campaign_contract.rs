use formula_core::digest::ArtifactDigest;
use formula_engine::{
    compiler::CompilerInputs,
    director::FormulaMathematicalDirector,
    ptah_campaign::{PtahCampaignBatch, PtahCampaignError},
    ptah_handoff::PtahCandidateReceipt,
    query::{
        ActivatedPackageBinding, QueryIR, RequestedResultClass, ResourceContract, SideEffectPolicy,
        TargetRequest,
    },
    region::CompilerAuthoritySnapshot,
};
fn d(n: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[n; 32])
}
fn build() -> PtahCampaignBatch {
    let packages = ActivatedPackageBinding::new(d(1), vec![d(30)], vec![]);
    let mut plans = vec![];
    for target in [d(3), d(9)] {
        let q = QueryIR::new(
            d(1),
            d(2),
            vec![],
            vec![],
            vec![TargetRequest::new(target, RequestedResultClass::Count)],
            d(4),
            d(5),
            ResourceContract::new(100, 1024, 50),
            SideEffectPolicy::deny_all(),
            packages.clone(),
        );
        let snapshot = CompilerAuthoritySnapshot::new(
            d(1),
            d(2),
            packages.clone(),
            vec![target],
            vec![],
            vec![],
        );
        plans.push(
            FormulaMathematicalDirector::plan(
                &q,
                &snapshot,
                CompilerInputs::new(d(4), d(5), d(6), d(7)),
            )
            .unwrap(),
        );
    }
    PtahCampaignBatch::from_directors(&plans).unwrap()
}
fn candidate(manifest: ArtifactDigest, cell: ArtifactDigest, bytes: &[u8]) -> PtahCandidateReceipt {
    PtahCandidateReceipt {
        manifest_digest: manifest,
        work_cell_digest: cell,
        claimed_digest: ArtifactDigest::of_bytes(bytes),
        bytes: bytes.to_vec(),
    }
}

#[test]
fn multi_cell_replay_with_exact_artifacts_and_no_authority() {
    let mut batch = build();
    let packets = batch.outgoing_packets();
    assert!(
        packets.len() >= 2,
        "two targets should generate independent work cells"
    );
    assert_eq!(batch.pending_cells().len(), packets.len());
    assert_eq!(
        batch.candidates(),
        Err(PtahCampaignError::IncompleteResults)
    );
    let original = batch.observation_bytes();
    for (digest, wire) in &packets {
        assert!(
            wire.windows(b"CANDIDATE_ONLY".len())
                .any(|x| x == b"CANDIDATE_ONLY")
        );
        let r = candidate(
            batch.manifest_for_cell(*digest).unwrap(),
            *digest,
            b"untrusted candidate\n",
        );
        batch.observe(r.clone()).unwrap();
        // Replaying the same real evidence is idempotent; a re-ack alone
        // would not have supplied the mandatory candidate bytes.
        batch.observe(r).unwrap();
    }
    assert!(batch.pending_cells().is_empty());
    assert_ne!(batch.observation_bytes(), original);
    assert_eq!(batch.candidates().unwrap().len(), packets.len());
    // Reconstruction after an interruption uses the same bound receipts,
    // not a saved "succeeded" flag or an invented candidate.
    let mut recovered = build();
    for (digest, _) in packets.iter().rev() {
        recovered
            .observe(candidate(
                recovered.manifest_for_cell(*digest).unwrap(),
                *digest,
                b"untrusted candidate\n",
            ))
            .unwrap();
    }
    assert_eq!(recovered.observation_bytes(), batch.observation_bytes());
}

#[test]
fn mismatched_or_conflicting_remote_readback_fails_closed() {
    let mut batch = build();
    let cell = batch.outgoing_packets()[0].0;
    let valid = candidate(batch.manifest_for_cell(cell).unwrap(), cell, b"candidate");
    batch.observe(valid.clone()).unwrap();
    assert_eq!(
        batch.observe(candidate(
            batch.manifest_for_cell(cell).unwrap(),
            cell,
            b"different"
        )),
        Err(PtahCampaignError::DuplicateConflictingResult)
    );
    assert!(batch.observe(candidate(d(99), cell, b"candidate")).is_err());
    assert_eq!(
        batch.observe(candidate(batch.manifest_digest(), d(88), b"candidate")),
        Err(PtahCampaignError::UnexpectedCell)
    );
    assert_eq!(
        batch.candidates(),
        Err(PtahCampaignError::IncompleteResults)
    );
}
