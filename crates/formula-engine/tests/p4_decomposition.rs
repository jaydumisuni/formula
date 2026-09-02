use formula_core::digest::ArtifactDigest;
use formula_engine::decomposition::{
    AggregationSemantics, ChildObligation, Decomposition, DecompositionError,
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn valid() -> Decomposition {
    Decomposition::new(
        d(1),
        d(2),
        vec![ChildObligation::new(d(3), d(2)), ChildObligation::new(d(4), d(2))],
        Some(d(5)),
        Some(AggregationSemantics::And),
        Some(d(6)),
        Some(d(7)),
    )
}

#[test]
fn valid_decomposition_is_deterministic() {
    let left = valid();
    let right = Decomposition::new(
        d(1),
        d(2),
        vec![ChildObligation::new(d(4), d(2)), ChildObligation::new(d(3), d(2))],
        Some(d(5)),
        Some(AggregationSemantics::And),
        Some(d(6)),
        Some(d(7)),
    );
    assert_eq!(left.validate(), Ok(()));
    assert_eq!(left.digest(), right.digest());
}

#[test]
fn decomposition_requires_children_aggregation_reconstruction_and_evidence() {
    let no_children = Decomposition::new(
        d(1), d(2), vec![], Some(d(5)), Some(AggregationSemantics::And), Some(d(6)), Some(d(7)),
    );
    assert_eq!(no_children.validate(), Err(DecompositionError::EmptyChildren));

    let no_aggregation = Decomposition::new(
        d(1), d(2), vec![ChildObligation::new(d(3), d(2))], Some(d(5)), None, Some(d(6)), Some(d(7)),
    );
    assert_eq!(
        no_aggregation.validate(),
        Err(DecompositionError::MissingAggregation)
    );

    let no_reconstruction = Decomposition::new(
        d(1), d(2), vec![ChildObligation::new(d(3), d(2))], Some(d(5)), Some(AggregationSemantics::Or), None, Some(d(7)),
    );
    assert_eq!(
        no_reconstruction.validate(),
        Err(DecompositionError::MissingReconstruction)
    );

    let no_evidence = Decomposition::new(
        d(1), d(2), vec![ChildObligation::new(d(3), d(2))], Some(d(5)), Some(AggregationSemantics::And), Some(d(6)), None,
    );
    assert_eq!(no_evidence.validate(), Err(DecompositionError::MissingEvidence));
}

#[test]
fn child_world_mismatch_fails_closed() {
    let decomposition = Decomposition::new(
        d(1),
        d(2),
        vec![ChildObligation::new(d(3), d(99))],
        Some(d(5)),
        Some(AggregationSemantics::And),
        Some(d(6)),
        Some(d(7)),
    );
    assert_eq!(
        decomposition.validate(),
        Err(DecompositionError::WorldMismatch)
    );
}
