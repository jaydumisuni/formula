use formula_core::digest::ArtifactDigest;
use formula_engine::reduction::{
    ReductionEdge, ReductionError, ResultClass, compose_reduction_path,
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn edge(id: u8, preserved: Vec<ResultClass>, reconstruct: bool) -> ReductionEdge {
    ReductionEdge::new(
        format!("source-{id}"),
        format!("target-{id}"),
        preserved,
        d(id),
        reconstruct.then(|| d(id.wrapping_add(40))),
        vec![],
        Some(d(id.wrapping_add(80))),
    )
}

#[test]
fn decision_only_path_cannot_serve_stronger_result_classes() {
    let path = [edge(1, vec![ResultClass::Decision], false)];
    assert!(compose_reduction_path(&path, ResultClass::Decision).is_ok());
    assert_eq!(
        compose_reduction_path(&path, ResultClass::Witness),
        Err(ReductionError::RequestedResultNotPreserved)
    );
    assert_eq!(
        compose_reduction_path(&path, ResultClass::Count),
        Err(ReductionError::RequestedResultNotPreserved)
    );
    assert_eq!(
        compose_reduction_path(&path, ResultClass::Optimum),
        Err(ReductionError::RequestedResultNotPreserved)
    );
}

#[test]
fn witness_preservation_requires_reconstruction_on_every_edge() {
    let path = [edge(1, vec![ResultClass::Witness], false)];
    assert_eq!(
        compose_reduction_path(&path, ResultClass::Witness),
        Err(ReductionError::MissingReconstruction)
    );
}

#[test]
fn one_weak_edge_invalidates_the_whole_path() {
    let path = [
        edge(1, vec![ResultClass::Decision, ResultClass::Witness], true),
        edge(2, vec![ResultClass::Decision], false),
        edge(3, vec![ResultClass::Decision, ResultClass::Witness], true),
    ];

    assert_eq!(
        compose_reduction_path(&path, ResultClass::Witness),
        Err(ReductionError::RequestedResultNotPreserved)
    );
}

#[test]
fn exact_witness_path_composes_deterministically() {
    let path = [
        edge(1, vec![ResultClass::Witness, ResultClass::Decision], true),
        edge(2, vec![ResultClass::Decision, ResultClass::Witness], true),
    ];
    let first = compose_reduction_path(&path, ResultClass::Witness).unwrap();
    let second = compose_reduction_path(&path, ResultClass::Witness).unwrap();
    assert_eq!(first.digest(), second.digest());
    assert_eq!(first.requested(), ResultClass::Witness);
}
