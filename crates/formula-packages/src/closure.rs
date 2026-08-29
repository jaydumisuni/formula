use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    theory::{ClosureContext, StructureWitness, TheoryPackageManifest},
};
use std::collections::BTreeSet;

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

pub fn derive_capabilities(
    context: &ClosureContext,
    witnesses: &[AdmittedStructureWitness],
    packages: &[TheoryPackageManifest],
) -> CapabilityClosure {
    let active: BTreeSet<_> = context.activated_packages().iter().copied().collect();
    let proven_goals: BTreeSet<_> = witnesses
        .iter()
        .filter(|admitted| admitted.generation() == context.generation())
        .map(AdmittedStructureWitness::witness)
        .filter(|witness| witness.world() == context.world())
        .map(StructureWitness::goal)
        .collect();

    let mut capabilities = BTreeSet::new();
    for package in packages {
        if !active.contains(&package.structural_digest()) {
            continue;
        }
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

    CapabilityClosure {
        context_digest: context.structural_digest(),
        capabilities,
    }
}
