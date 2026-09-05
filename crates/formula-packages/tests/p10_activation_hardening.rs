use formula_core::{
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    self_expansion::{ActivationMode, ExpansionActivationRecord, PromotionClass},
};
use formula_packages::{
    expansion::{
        ExpansionError, PromotedRouteActivation, ScopedNogoodActivation, active_routes,
        applicable_nogoods,
    },
    grammar::derive_grammar_generation,
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

#[test]
fn unscoped_automatic_nogood_is_rejected() {
    let subject = d("p10:nogood");
    let evidence = d("p10:nogood:evidence");
    let generation = UniverseGeneration::new(0, None, vec![subject], vec![evidence]);
    let record = ExpansionActivationRecord::new(
        subject,
        PromotionClass::CounterexampleNogood,
        generation.digest(),
        d("p10:world"),
        ActivationMode::BoundedAutomatic,
        vec![evidence],
        vec![],
    );

    assert_eq!(
        ScopedNogoodActivation::new(&generation, &record),
        Err(ExpansionError::ScopeRequired)
    );
}

#[test]
fn nogood_scope_mismatch_does_not_prune_unrelated_context() {
    let subject = d("p10:scoped-nogood");
    let evidence = d("p10:scoped-nogood:evidence");
    let scope = d("p10:scope:a");
    let other_scope = d("p10:scope:b");
    let generation = UniverseGeneration::new(0, None, vec![subject], vec![evidence]);
    let record = ExpansionActivationRecord::new(
        subject,
        PromotionClass::CounterexampleNogood,
        generation.digest(),
        d("p10:world"),
        ActivationMode::BoundedAutomatic,
        vec![evidence],
        vec![scope],
    );
    let activation = ScopedNogoodActivation::new(&generation, &record).unwrap();

    assert!(applicable_nogoods(&[other_scope], &[activation.clone()]).is_empty());
    assert_eq!(
        applicable_nogoods(&[scope, other_scope], &[activation]),
        vec![subject]
    );
}

#[test]
fn promoted_route_requires_preservation_evidence_and_enters_active_routes() {
    let route = d("p10:route");
    let evidence = d("p10:route:evidence");
    let result_class = d("p10:result-class:exact-witness");
    let generation = UniverseGeneration::new(0, None, vec![route], vec![evidence]);
    let record = ExpansionActivationRecord::new(
        route,
        PromotionClass::Reduction,
        generation.digest(),
        d("p10:world"),
        ActivationMode::DefaultAutomatic,
        vec![evidence],
        vec![d("p10:route:scope")],
    );

    assert_eq!(
        PromotedRouteActivation::new(&generation, &record, vec![]),
        Err(ExpansionError::PreservationEvidenceRequired)
    );

    let activation =
        PromotedRouteActivation::new(&generation, &record, vec![result_class]).unwrap();
    assert_eq!(active_routes(&[activation.clone()]), vec![route]);

    let grammar = derive_grammar_generation(
        &generation,
        None,
        &[],
        &[activation],
        &[d("p10:theory-rule")],
    )
    .unwrap();
    assert_eq!(grammar.activated_route_rules(), &[route]);
}

#[test]
fn shadow_metaprimitive_is_recorded_but_never_enters_active_grammar() {
    let metaprimitive = d("p10:metaprimitive");
    let evidence = d("p10:metaprimitive:evidence");
    let generation = UniverseGeneration::new(0, None, vec![metaprimitive], vec![evidence]);
    let record = ExpansionActivationRecord::new(
        metaprimitive,
        PromotionClass::MetaprimitiveSearchMethod,
        generation.digest(),
        d("p10:world"),
        ActivationMode::ShadowOnly,
        vec![evidence],
        vec![d("p10:shadow:scope")],
    );

    let grammar =
        derive_grammar_generation(&generation, None, &[record], &[], &[d("p10:theory-rule")])
            .unwrap();

    assert!(grammar.activated_metaprimitives().is_empty());
    assert_eq!(grammar.shadow_metaprimitives(), &[metaprimitive]);
}

#[test]
fn wrong_generation_activation_is_rejected_before_use() {
    let subject = d("p10:wrong-generation-nogood");
    let evidence = d("p10:wrong-generation:evidence");
    let generation = UniverseGeneration::new(0, None, vec![subject], vec![evidence]);
    let wrong_generation =
        UniverseGeneration::new(1, Some(generation.digest()), vec![subject], vec![evidence]);
    let record = ExpansionActivationRecord::new(
        subject,
        PromotionClass::CounterexampleNogood,
        wrong_generation.digest(),
        d("p10:world"),
        ActivationMode::BoundedAutomatic,
        vec![evidence],
        vec![d("p10:scope")],
    );

    assert_eq!(
        ScopedNogoodActivation::new(&generation, &record),
        Err(ExpansionError::GenerationMismatch)
    );
}
