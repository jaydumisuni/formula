use formula_core::digest::ArtifactDigest;
use formula_engine::{
    candidate_space::{CandidateSpaceContext, SearchAuthority},
    discovery::{run_bounded_cegis, CandidateValidation, CegisOutcome, DiscoveryOracle},
    observational::{FrozenExprCandidate, ObservationalExprSpace, U8BoolGrammar},
    search_policy::{FairRoundRobin, HeuristicRanking},
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn context() -> CandidateSpaceContext {
    CandidateSpaceContext::new(d(1), d(2), d(3), d(4), d(5), d(6))
}

struct ScriptedOracle {
    validations: usize,
    seen_frozen: Vec<ArtifactDigest>,
}

impl DiscoveryOracle for ScriptedOracle {
    fn output_for_sample(&mut self, input: u8) -> bool {
        input != 0
    }

    fn validate_frozen_candidate(&mut self, candidate: &FrozenExprCandidate) -> CandidateValidation {
        self.seen_frozen.push(candidate.frozen().digest());
        self.validations += 1;
        if self.validations == 1 {
            CandidateValidation::Counterexample { input: 0, expected: false }
        } else {
            CandidateValidation::Equivalent
        }
    }
}

#[test]
fn cegis_freezes_before_validation_and_refines_on_counterexample() {
    let mut space = ObservationalExprSpace::new(context(), U8BoolGrammar::minimal(), 7);
    let mut oracle = ScriptedOracle { validations: 0, seen_frozen: vec![] };
    let result = run_bounded_cegis(&mut space, &mut oracle, &[1], 4);
    let CegisOutcome::Candidate(candidate, trace) = result else { panic!("expected candidate") };
    assert_eq!(candidate.frozen().authority(), SearchAuthority::CandidateOnly);
    assert_eq!(oracle.seen_frozen, trace.frozen_before_validation());
    assert_eq!(trace.counterexamples(), &[(0, false)]);
}

#[test]
fn iteration_exhaustion_is_resource_unknown_not_refutation() {
    struct NeverDone;
    impl DiscoveryOracle for NeverDone {
        fn output_for_sample(&mut self, input: u8) -> bool { input != 0 }
        fn validate_frozen_candidate(&mut self, _candidate: &FrozenExprCandidate) -> CandidateValidation {
            CandidateValidation::Counterexample { input: 0, expected: false }
        }
    }
    let mut space = ObservationalExprSpace::new(context(), U8BoolGrammar::minimal(), 5);
    let result = run_bounded_cegis(&mut space, &mut NeverDone, &[1], 1);
    assert!(matches!(result, CegisOutcome::ResourceBoundedUnknown(_)));
}

#[test]
fn fair_round_robin_prevents_starvation() {
    let mut fair = FairRoundRobin::new(vec!["a", "b", "c"]);
    assert_eq!(fair.next(), Some("a"));
    assert_eq!(fair.next(), Some("b"));
    assert_eq!(fair.next(), Some("c"));
    assert_eq!(fair.next(), Some("a"));
}

#[test]
fn heuristic_ranking_only_reorders_candidates() {
    let ranking = HeuristicRanking::new(vec![(d(10), 100), (d(11), 1)]);
    assert_eq!(ranking.ordered_candidates(), vec![d(10), d(11)]);
    assert_eq!(ranking.authority(), SearchAuthority::CandidateOnly);
}
