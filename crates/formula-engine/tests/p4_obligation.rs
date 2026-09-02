use formula_core::digest::ArtifactDigest;
use formula_engine::{
    obligation::{ObligationIR, ObligationOutcome, TerminalState},
    query::ResourceContract,
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn obligation() -> ObligationIR {
    ObligationIR::new(
        d(1),
        d(2),
        vec![d(3), d(4)],
        "witness",
        d(5),
        d(6),
        vec![d(7), d(8)],
        vec![d(9)],
        ResourceContract::new(100, 1024, 50),
        vec![
            TerminalState::Satisfied,
            TerminalState::Refuted,
            TerminalState::SemanticUnknown,
            TerminalState::ResourceBoundedUnknown,
        ],
    )
}

#[test]
fn obligation_identity_is_deterministic_for_set_like_inputs() {
    let left = obligation();
    let right = ObligationIR::new(
        d(1),
        d(2),
        vec![d(4), d(3), d(3)],
        "witness",
        d(5),
        d(6),
        vec![d(8), d(7)],
        vec![d(9)],
        ResourceContract::new(100, 1024, 50),
        vec![
            TerminalState::ResourceBoundedUnknown,
            TerminalState::SemanticUnknown,
            TerminalState::Refuted,
            TerminalState::Satisfied,
        ],
    );
    assert_eq!(left.digest(), right.digest());
}

#[test]
fn refuted_semantic_unknown_and_resource_unknown_are_distinct() {
    let obligation = obligation();
    let refuted = ObligationOutcome::new(obligation.digest(), TerminalState::Refuted);
    let semantic_unknown =
        ObligationOutcome::new(obligation.digest(), TerminalState::SemanticUnknown);
    let resource_unknown =
        ObligationOutcome::new(obligation.digest(), TerminalState::ResourceBoundedUnknown);

    assert_ne!(refuted.digest(), semantic_unknown.digest());
    assert_ne!(refuted.digest(), resource_unknown.digest());
    assert_ne!(semantic_unknown.digest(), resource_unknown.digest());
}

#[test]
fn resource_exhaustion_cannot_become_refuted() {
    let outcome = ObligationOutcome::resource_exhausted(obligation().digest());
    assert_eq!(outcome.state(), TerminalState::ResourceBoundedUnknown);
    assert_ne!(outcome.state(), TerminalState::Refuted);
}
