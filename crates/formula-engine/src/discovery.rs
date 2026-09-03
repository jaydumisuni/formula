use crate::observational::{FrozenExprCandidate, ObservationalExprSpace};
use formula_core::digest::ArtifactDigest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CandidateValidation {
    Equivalent,
    Counterexample { input: u8, expected: bool },
}

pub trait DiscoveryOracle {
    fn output_for_sample(&mut self, input: u8) -> bool;
    fn validate_frozen_candidate(&mut self, candidate: &FrozenExprCandidate)
    -> CandidateValidation;
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CegisTrace {
    frozen_before_validation: Vec<ArtifactDigest>,
    counterexamples: Vec<(u8, bool)>,
}

impl CegisTrace {
    pub fn frozen_before_validation(&self) -> &[ArtifactDigest] {
        &self.frozen_before_validation
    }

    pub fn counterexamples(&self) -> &[(u8, bool)] {
        &self.counterexamples
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CegisOutcome {
    Candidate(FrozenExprCandidate, CegisTrace),
    SemanticUnknown(CegisTrace),
    ResourceBoundedUnknown(CegisTrace),
}

pub fn run_bounded_cegis<O: DiscoveryOracle>(
    space: &mut ObservationalExprSpace,
    oracle: &mut O,
    initial_samples: &[u8],
    max_iterations: usize,
) -> CegisOutcome {
    let mut samples = initial_samples.to_vec();
    samples.sort_unstable();
    samples.dedup();
    for input in samples {
        let expected = oracle.output_for_sample(input);
        space.restrict_exact_sample(input, expected);
    }

    let mut trace = CegisTrace::default();
    for _ in 0..max_iterations {
        let Some(candidate) = space.extract_min_cost() else {
            return CegisOutcome::SemanticUnknown(trace);
        };

        trace
            .frozen_before_validation
            .push(candidate.frozen().digest());

        match oracle.validate_frozen_candidate(&candidate) {
            CandidateValidation::Equivalent => {
                return CegisOutcome::Candidate(candidate, trace);
            }
            CandidateValidation::Counterexample { input, expected } => {
                if !trace.counterexamples.contains(&(input, expected)) {
                    trace.counterexamples.push((input, expected));
                }
                space.refine_counterexample(input, expected);
            }
        }
    }

    CegisOutcome::ResourceBoundedUnknown(trace)
}
