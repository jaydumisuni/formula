use formula_core::{digest::ArtifactDigest, theory::CanonicalMorphism};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MorphismRegistry {
    morphisms: Vec<CanonicalMorphism>,
}

impl MorphismRegistry {
    pub fn new(mut morphisms: Vec<CanonicalMorphism>) -> Self {
        morphisms
            .sort_by_key(|morphism| (morphism.source(), morphism.target(), morphism.morphism()));
        morphisms.dedup_by(|left, right| {
            left.source() == right.source()
                && left.target() == right.target()
                && left.morphism() == right.morphism()
        });
        Self { morphisms }
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
