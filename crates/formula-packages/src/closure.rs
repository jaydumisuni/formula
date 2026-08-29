use crate::activation::ActivatedPackageSet;
use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    generation::UniverseGeneration,
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
    if active.iter().any(|digest| !package_map.contains_key(digest)) {
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
