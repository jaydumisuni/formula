use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    theory::CanonicalMorphism,
};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphismRegistryError {
    MorphismNotAdmitted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MorphismRegistry {
    generation: ArtifactDigest,
    morphisms: Vec<CanonicalMorphism>,
}

impl MorphismRegistry {
    pub fn new(
        generation: &UniverseGeneration,
        mut morphisms: Vec<CanonicalMorphism>,
    ) -> Result<Self, MorphismRegistryError> {
        if morphisms
            .iter()
            .any(|morphism| !generation.admitted().contains(&morphism.structural_digest()))
        {
            return Err(MorphismRegistryError::MorphismNotAdmitted);
        }

        morphisms
            .sort_by_key(|morphism| (morphism.source(), morphism.target(), morphism.morphism()));
        morphisms.dedup_by(|left, right| {
            left.source() == right.source()
                && left.target() == right.target()
                && left.morphism() == right.morphism()
        });
        Ok(Self {
            generation: generation.digest(),
            morphisms,
        })
    }

    pub fn generation(&self) -> ArtifactDigest {
        self.generation
    }

    pub fn morphisms(&self) -> &[CanonicalMorphism] {
        &self.morphisms
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommonParentResolution {
    ProvenUnique { parent: ArtifactDigest },
    Ambiguous,
    Unknown,
}

pub fn resolve_common_parent(
    registry: &MorphismRegistry,
    left: ArtifactDigest,
    right: ArtifactDigest,
) -> CommonParentResolution {
    if left == right {
        return CommonParentResolution::ProvenUnique { parent: left };
    }

    let left_targets: BTreeSet<_> = registry
        .morphisms()
        .iter()
        .filter(|morphism| {
            morphism.source() == left && morphism.is_canonical() && morphism.is_lossless()
        })
        .map(CanonicalMorphism::target)
        .collect();
    let right_targets: BTreeSet<_> = registry
        .morphisms()
        .iter()
        .filter(|morphism| {
            morphism.source() == right && morphism.is_canonical() && morphism.is_lossless()
        })
        .map(CanonicalMorphism::target)
        .collect();

    let common: Vec<_> = left_targets.intersection(&right_targets).copied().collect();
    match common.as_slice() {
        [] => CommonParentResolution::Unknown,
        [parent] => CommonParentResolution::ProvenUnique { parent: *parent },
        _ => CommonParentResolution::Ambiguous,
    }
}
