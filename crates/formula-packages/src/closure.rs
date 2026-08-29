use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    theory::{ClosureContext, StructureWitness, TheoryPackageManifest},
};
use std::collections::BTreeSet;

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
    witnesses: &[StructureWitness],
    packages: &[TheoryPackageManifest],
) -> CapabilityClosure {
    let active: BTreeSet<_> = context.activated_packages().iter().copied().collect();
    let proven_goals: BTreeSet<_> = witnesses
        .iter()
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
