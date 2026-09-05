use formula_core::{
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    self_expansion::{ActivationMode, ExpansionActivationRecord, PromotionClass},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExpansionError {
    GenerationMismatch,
    SubjectNotAdmitted,
    EvidenceNotAuthorityBound(ArtifactDigest),
    UnsupportedClass,
    ScopeRequired,
    PreservationEvidenceRequired,
    TerminalActivation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopedNogoodActivation {
    subject: ArtifactDigest,
    generation: ArtifactDigest,
    world: ArtifactDigest,
    mode: ActivationMode,
    evidence: Vec<ArtifactDigest>,
    scope: Vec<ArtifactDigest>,
}

impl ScopedNogoodActivation {
    pub fn new(
        generation: &UniverseGeneration,
        record: &ExpansionActivationRecord,
    ) -> Result<Self, ExpansionError> {
        if record.promotion_class() != PromotionClass::CounterexampleNogood {
            return Err(ExpansionError::UnsupportedClass);
        }
        validate_record(generation, record)?;
        if is_terminal(record.mode()) {
            return Err(ExpansionError::TerminalActivation);
        }
        if is_automatic(record.mode()) && record.scope().is_empty() {
            return Err(ExpansionError::ScopeRequired);
        }

        Ok(Self {
            subject: record.subject(),
            generation: record.generation(),
            world: record.world(),
            mode: record.mode(),
            evidence: record.evidence().to_vec(),
            scope: record.scope().to_vec(),
        })
    }

    pub fn subject(&self) -> ArtifactDigest {
        self.subject
    }

    pub fn generation(&self) -> ArtifactDigest {
        self.generation
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn mode(&self) -> ActivationMode {
        self.mode
    }

    pub fn evidence(&self) -> &[ArtifactDigest] {
        &self.evidence
    }

    pub fn scope(&self) -> &[ArtifactDigest] {
        &self.scope
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromotedRouteActivation {
    subject: ArtifactDigest,
    generation: ArtifactDigest,
    world: ArtifactDigest,
    promotion_class: PromotionClass,
    mode: ActivationMode,
    evidence: Vec<ArtifactDigest>,
    scope: Vec<ArtifactDigest>,
    preserved_result_classes: Vec<ArtifactDigest>,
}

impl PromotedRouteActivation {
    pub fn new(
        generation: &UniverseGeneration,
        record: &ExpansionActivationRecord,
        mut preserved_result_classes: Vec<ArtifactDigest>,
    ) -> Result<Self, ExpansionError> {
        if !matches!(
            record.promotion_class(),
            PromotionClass::Reduction | PromotionClass::MorphismTheoryInterpretation
        ) {
            return Err(ExpansionError::UnsupportedClass);
        }
        validate_record(generation, record)?;
        if is_terminal(record.mode()) {
            return Err(ExpansionError::TerminalActivation);
        }
        if is_automatic(record.mode())
            && (preserved_result_classes.is_empty() || record.evidence().is_empty())
        {
            return Err(ExpansionError::PreservationEvidenceRequired);
        }
        preserved_result_classes.sort_unstable();
        preserved_result_classes.dedup();

        Ok(Self {
            subject: record.subject(),
            generation: record.generation(),
            world: record.world(),
            promotion_class: record.promotion_class(),
            mode: record.mode(),
            evidence: record.evidence().to_vec(),
            scope: record.scope().to_vec(),
            preserved_result_classes,
        })
    }

    pub fn subject(&self) -> ArtifactDigest {
        self.subject
    }

    pub fn generation(&self) -> ArtifactDigest {
        self.generation
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn promotion_class(&self) -> PromotionClass {
        self.promotion_class
    }

    pub fn mode(&self) -> ActivationMode {
        self.mode
    }

    pub fn evidence(&self) -> &[ArtifactDigest] {
        &self.evidence
    }

    pub fn scope(&self) -> &[ArtifactDigest] {
        &self.scope
    }

    pub fn preserved_result_classes(&self) -> &[ArtifactDigest] {
        &self.preserved_result_classes
    }
}

pub fn applicable_nogoods(
    context_scope: &[ArtifactDigest],
    activations: &[ScopedNogoodActivation],
) -> Vec<ArtifactDigest> {
    let mut context = context_scope.to_vec();
    context.sort_unstable();
    context.dedup();

    let mut applicable = activations
        .iter()
        .filter(|activation| is_automatic(activation.mode()))
        .filter(|activation| {
            activation
                .scope()
                .iter()
                .all(|required| context.binary_search(required).is_ok())
        })
        .map(ScopedNogoodActivation::subject)
        .collect::<Vec<_>>();
    applicable.sort_unstable();
    applicable.dedup();
    applicable
}

pub fn active_routes(activations: &[PromotedRouteActivation]) -> Vec<ArtifactDigest> {
    let mut active = activations
        .iter()
        .filter(|activation| is_automatic(activation.mode()))
        .map(PromotedRouteActivation::subject)
        .collect::<Vec<_>>();
    active.sort_unstable();
    active.dedup();
    active
}

pub(crate) fn validate_record(
    generation: &UniverseGeneration,
    record: &ExpansionActivationRecord,
) -> Result<(), ExpansionError> {
    if record.generation() != generation.digest() {
        return Err(ExpansionError::GenerationMismatch);
    }
    if !generation.admitted().contains(&record.subject()) {
        return Err(ExpansionError::SubjectNotAdmitted);
    }
    for evidence in record.evidence() {
        if !generation.authority_bindings().contains(evidence) {
            return Err(ExpansionError::EvidenceNotAuthorityBound(*evidence));
        }
    }
    Ok(())
}

pub(crate) fn is_automatic(mode: ActivationMode) -> bool {
    matches!(
        mode,
        ActivationMode::BoundedAutomatic | ActivationMode::DefaultAutomatic
    )
}

pub(crate) fn is_terminal(mode: ActivationMode) -> bool {
    matches!(
        mode,
        ActivationMode::Superseded | ActivationMode::Quarantined
    )
}
