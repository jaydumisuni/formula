use formula_core::digest::ArtifactDigest;
use formula_engine::{
    query::RequestedResultClass,
    representation::{
        ExactnessClass, InformationLoss, PreservationMetadata, RepresentationEdge,
        RepresentationError, RepresentationNode,
    },
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn node(representation: u8) -> RepresentationNode {
    RepresentationNode::new(d(1), d(representation), d(2), ExactnessClass::Exact, d(3))
}

#[test]
fn exact_lossless_representation_edge_validates() {
    let source = node(10);
    let target = node(11);
    let edge = RepresentationEdge::new(
        source.digest(),
        target.digest(),
        d(12),
        Some(PreservationMetadata::new(
            ExactnessClass::Exact,
            vec![RequestedResultClass::Decision, RequestedResultClass::Witness],
        )),
        InformationLoss::None,
        Some(d(13)),
        Some(d(14)),
        vec![],
    );

    assert_eq!(
        edge.validate(&source, &target, RequestedResultClass::Witness),
        Ok(())
    );
}

#[test]
fn preservation_metadata_is_mandatory() {
    let source = node(10);
    let target = node(11);
    let edge = RepresentationEdge::new(
        source.digest(),
        target.digest(),
        d(12),
        None,
        InformationLoss::None,
        None,
        Some(d(14)),
        vec![],
    );

    assert_eq!(
        edge.validate(&source, &target, RequestedResultClass::Decision),
        Err(RepresentationError::MissingPreservationMetadata)
    );
}

#[test]
fn implicit_loss_cannot_serve_exact_witness() {
    let source = node(10);
    let target = node(11);
    let edge = RepresentationEdge::new(
        source.digest(),
        target.digest(),
        d(12),
        Some(PreservationMetadata::new(
            ExactnessClass::SoundOverApproximation,
            vec![RequestedResultClass::Decision],
        )),
        InformationLoss::Declared,
        None,
        Some(d(14)),
        vec![],
    );

    assert_eq!(
        edge.validate(&source, &target, RequestedResultClass::Witness),
        Err(RepresentationError::RequestedResultNotPreserved)
    );
}

#[test]
fn lossy_witness_route_requires_explicit_reconstruction() {
    let source = node(10);
    let target = node(11);
    let edge = RepresentationEdge::new(
        source.digest(),
        target.digest(),
        d(12),
        Some(PreservationMetadata::new(
            ExactnessClass::Exact,
            vec![RequestedResultClass::Witness],
        )),
        InformationLoss::Declared,
        None,
        Some(d(14)),
        vec![],
    );

    assert_eq!(
        edge.validate(&source, &target, RequestedResultClass::Witness),
        Err(RepresentationError::MissingReconstructionRoute)
    );
}

#[test]
fn world_and_observer_must_match() {
    let source = node(10);
    let wrong_world = RepresentationNode::new(d(1), d(11), d(99), ExactnessClass::Exact, d(3));
    let wrong_observer = RepresentationNode::new(d(1), d(11), d(2), ExactnessClass::Exact, d(98));
    let edge = RepresentationEdge::new(
        source.digest(),
        wrong_world.digest(),
        d(12),
        Some(PreservationMetadata::new(
            ExactnessClass::Exact,
            vec![RequestedResultClass::Decision],
        )),
        InformationLoss::None,
        None,
        Some(d(14)),
        vec![],
    );

    assert_eq!(
        edge.validate(&source, &wrong_world, RequestedResultClass::Decision),
        Err(RepresentationError::WorldMismatch)
    );

    let observer_edge = RepresentationEdge::new(
        source.digest(),
        wrong_observer.digest(),
        d(12),
        Some(PreservationMetadata::new(
            ExactnessClass::Exact,
            vec![RequestedResultClass::Decision],
        )),
        InformationLoss::None,
        None,
        Some(d(14)),
        vec![],
    );
    assert_eq!(
        observer_edge.validate(&source, &wrong_observer, RequestedResultClass::Decision),
        Err(RepresentationError::ObserverMismatch)
    );
}
