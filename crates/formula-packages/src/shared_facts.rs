use formula_core::theory::{FactPolarity, SharedFact};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FactRequirement {
    Exact,
    ExistenceWitness,
    OverApproximation,
    UnderApproximation,
    LowerBound,
    UpperBound,
    NecessaryCondition,
    SufficientCondition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FactUseDecision {
    Allowed,
    Rejected,
}

pub fn fact_satisfies(fact: &SharedFact, requirement: FactRequirement) -> FactUseDecision {
    let allowed = match fact.polarity() {
        FactPolarity::Exact => true,
        FactPolarity::OverApproximation => matches!(
            requirement,
            FactRequirement::OverApproximation | FactRequirement::NecessaryCondition
        ),
        FactPolarity::UnderApproximation => matches!(
            requirement,
            FactRequirement::UnderApproximation
        ),
        FactPolarity::LowerBound => matches!(requirement, FactRequirement::LowerBound),
        FactPolarity::UpperBound => matches!(requirement, FactRequirement::UpperBound),
        FactPolarity::NecessaryCondition => {
            matches!(requirement, FactRequirement::NecessaryCondition)
        }
        FactPolarity::SufficientCondition => matches!(
            requirement,
            FactRequirement::SufficientCondition | FactRequirement::ExistenceWitness
        ),
        FactPolarity::HeuristicCandidate => false,
    };

    if allowed {
        FactUseDecision::Allowed
    } else {
        FactUseDecision::Rejected
    }
}
