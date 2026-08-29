use formula_core::{
    canonical::CanonicalValue,
    digest::ArtifactDigest,
    theory::{FactPolarity, SharedFact},
};
use formula_packages::shared_facts::{fact_satisfies, FactRequirement, FactUseDecision};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn fact(polarity: FactPolarity) -> SharedFact {
    SharedFact::new(
        d("world"),
        d("subject"),
        CanonicalValue::String("fact".into()),
        polarity,
        d("evidence"),
    )
}

#[test]
fn over_approximation_cannot_discharge_exact_or_existence_requirements() {
    let over = fact(FactPolarity::OverApproximation);
    assert_eq!(
        fact_satisfies(&over, FactRequirement::Exact),
        FactUseDecision::Rejected
    );
    assert_eq!(
        fact_satisfies(&over, FactRequirement::ExistenceWitness),
        FactUseDecision::Rejected
    );
    assert_eq!(
        fact_satisfies(&over, FactRequirement::OverApproximation),
        FactUseDecision::Allowed
    );
}

#[test]
fn bounds_and_conditions_only_flow_to_semantically_compatible_consumers() {
    assert_eq!(
        fact_satisfies(&fact(FactPolarity::LowerBound), FactRequirement::LowerBound),
        FactUseDecision::Allowed
    );
    assert_eq!(
        fact_satisfies(&fact(FactPolarity::LowerBound), FactRequirement::UpperBound),
        FactUseDecision::Rejected
    );
    assert_eq!(
        fact_satisfies(
            &fact(FactPolarity::NecessaryCondition),
            FactRequirement::SufficientCondition
        ),
        FactUseDecision::Rejected
    );
    assert_eq!(
        fact_satisfies(
            &fact(FactPolarity::SufficientCondition),
            FactRequirement::ExistenceWitness
        ),
        FactUseDecision::Allowed
    );
}

#[test]
fn exact_facts_may_satisfy_exact_and_weaker_directional_consumers() {
    let exact = fact(FactPolarity::Exact);
    for requirement in [
        FactRequirement::Exact,
        FactRequirement::ExistenceWitness,
        FactRequirement::OverApproximation,
        FactRequirement::UnderApproximation,
        FactRequirement::NecessaryCondition,
        FactRequirement::SufficientCondition,
    ] {
        assert_eq!(fact_satisfies(&exact, requirement), FactUseDecision::Allowed);
    }
}
