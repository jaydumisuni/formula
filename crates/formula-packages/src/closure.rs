use crate::activation::ActivatedPackageSet;
use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    promotion::{PromotionRecord, PromotionState},
    theory::{ClosureContext, StructureWitness, TheoryPackageManifest},
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WitnessAdmissionError {
    WitnessNotAdmitted,
    EvidenceNotAuthorityBound,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedStructureWitness {
    generation: ArtifactDigest,
    witness: StructureWitness,
}

impl AdmittedStructureWitness {
    pub fn new(
        generation: &UniverseGeneration,
        witness: StructureWitness,
    ) -> Result<Self, WitnessAdmissionError> {
        if !generation.admitted().contains(&witness.structural_digest()) {
            return Err(WitnessAdmissionError::WitnessNotAdmitted);
        }
        if !generation
            .authority_bindings()
            .contains(&witness.evidence())
        {
            return Err(WitnessAdmissionError::EvidenceNotAuthorityBound);
        }

        Ok(Self {
            generation: generation.digest(),
            witness,
        })
    }

    pub fn generation(&self) -> ArtifactDigest {
        self.generation
    }

    pub fn witness(&self) -> &StructureWitness {
        &self.witness
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClosureError {
    GenerationMismatch,
    ActivatedPackageMismatch,
    MissingPackageManifest,
    SemanticActivationGenerationMismatch,
    SemanticActivationStateMismatch,
    SemanticActivationPrimitiveNotAdmitted,
    SemanticActivationEvidenceNotAuthorityBound,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityClosure {
    context_digest: ArtifactDigest,
    capabilities: BTreeSet<ArtifactDigest>,
}

impl CapabilityClosure {
    pub fn context_digest(&self) -> ArtifactDigest {
        self.context_digest
    }

    pub fn contains(&self, capability: ArtifactDigest) -> bool {
        self.capabilities.contains(&capability)
    }

    pub fn capabilities(&self) -> impl Iterator<Item = ArtifactDigest> + '_ {
        self.capabilities.iter().copied()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityClosureDelta {
    before_context_digest: ArtifactDigest,
    after_context_digest: ArtifactDigest,
    added: BTreeSet<ArtifactDigest>,
    removed: BTreeSet<ArtifactDigest>,
}

impl CapabilityClosureDelta {
    pub fn between(before: &CapabilityClosure, after: &CapabilityClosure) -> Self {
        Self {
            before_context_digest: before.context_digest,
            after_context_digest: after.context_digest,
            added: after
                .capabilities
                .difference(&before.capabilities)
                .copied()
                .collect(),
            removed: before
                .capabilities
                .difference(&after.capabilities)
                .copied()
                .collect(),
        }
    }

    pub fn before_context_digest(&self) -> ArtifactDigest {
        self.before_context_digest
    }

    pub fn after_context_digest(&self) -> ArtifactDigest {
        self.after_context_digest
    }

    pub fn added(&self) -> impl Iterator<Item = ArtifactDigest> + '_ {
        self.added.iter().copied()
    }

    pub fn removed(&self) -> impl Iterator<Item = ArtifactDigest> + '_ {
        self.removed.iter().copied()
    }
}

pub fn derive_capabilities(
    context: &ClosureContext,
    activated: &ActivatedPackageSet,
    witnesses: &[AdmittedStructureWitness],
    packages: &[TheoryPackageManifest],
) -> Result<CapabilityClosure, ClosureError> {
    if activated.generation() != context.generation() {
        return Err(ClosureError::GenerationMismatch);
    }

    let active: BTreeSet<_> = activated.digests().iter().copied().collect();
    let context_active: BTreeSet<_> = context.activated_packages().iter().copied().collect();
    if active != context_active {
        return Err(ClosureError::ActivatedPackageMismatch);
    }

    let package_map: BTreeMap<_, _> = packages
        .iter()
        .map(|package| (package.structural_digest(), package))
        .collect();
    if active
        .iter()
        .any(|digest| !package_map.contains_key(digest))
    {
        return Err(ClosureError::MissingPackageManifest);
    }

    let proven_goals: BTreeSet<_> = witnesses
        .iter()
        .filter(|admitted| admitted.generation() == context.generation())
        .map(AdmittedStructureWitness::witness)
        .filter(|witness| witness.world() == context.world())
        .map(StructureWitness::goal)
        .collect();

    let mut capabilities = BTreeSet::new();
    for digest in &active {
        let package = package_map
            .get(digest)
            .expect("active package manifest prevalidated");
        for contract in package.capabilities() {
            if contract
                .required_goals()
                .iter()
                .all(|goal| proven_goals.contains(goal))
            {
                capabilities.insert(contract.capability());
            }
        }
    }

    Ok(CapabilityClosure {
        context_digest: context.structural_digest(),
        capabilities,
    })
}

pub fn derive_capabilities_with_semantic_activations(
    context: &ClosureContext,
    activated: &ActivatedPackageSet,
    witnesses: &[AdmittedStructureWitness],
    packages: &[TheoryPackageManifest],
    generation: &UniverseGeneration,
    activations: &[PromotionRecord],
) -> Result<CapabilityClosure, ClosureError> {
    let mut closure = derive_capabilities(context, activated, witnesses, packages)?;
    let generation_digest = generation.digest();
    if generation_digest != context.generation() {
        return Err(ClosureError::SemanticActivationGenerationMismatch);
    }

    for activation in activations {
        if activation.state() != PromotionState::Activated {
            return Err(ClosureError::SemanticActivationStateMismatch);
        }
        if activation.generation() != generation_digest {
            return Err(ClosureError::SemanticActivationGenerationMismatch);
        }
        for evidence in activation.evidence() {
            if !generation.authority_bindings().contains(evidence) {
                return Err(ClosureError::SemanticActivationEvidenceNotAuthorityBound);
            }
        }
        for primitive in activation.semantic_artifacts() {
            if !generation.admitted().contains(primitive) {
                return Err(ClosureError::SemanticActivationPrimitiveNotAdmitted);
            }
            closure.capabilities.insert(*primitive);
        }
    }

    Ok(closure)
}
