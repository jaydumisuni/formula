use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    theory::{CompositionClaim, CompositionClass, TheoryPackageManifest},
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActivationError {
    UnknownPackage,
    MissingDependency,
    InterferenceUnproven,
    CompositionNotAdmissible,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivatedPackageSet {
    digests: Vec<ArtifactDigest>,
}

impl ActivatedPackageSet {
    pub fn digests(&self) -> &[ArtifactDigest] {
        &self.digests
    }
}

pub fn validate_activation(
    packages: &[TheoryPackageManifest],
    claims: &[CompositionClaim],
    requested: &[ArtifactDigest],
) -> Result<ActivatedPackageSet, ActivationError> {
    let mut requested = requested.to_vec();
    requested.sort_unstable();
    requested.dedup();
    let requested_set: BTreeSet<_> = requested.iter().copied().collect();

    let package_map: BTreeMap<_, _> = packages
        .iter()
        .map(|package| (package.structural_digest(), package))
        .collect();

    let selected: Vec<_> = requested
        .iter()
        .map(|digest| package_map.get(digest).copied().ok_or(ActivationError::UnknownPackage))
        .collect::<Result<_, _>>()?;

    for package in &selected {
        if package
            .dependencies()
            .iter()
            .any(|dependency| !requested_set.contains(dependency))
        {
            return Err(ActivationError::MissingDependency);
        }
    }

    for (left_index, left) in selected.iter().enumerate() {
        for right in selected.iter().skip(left_index + 1) {
            let overlap = left
                .interference_surface()
                .iter()
                .any(|entry| right.interference_surface().binary_search(entry).is_ok());
            if !overlap {
                continue;
            }

            let left_digest = left.structural_digest();
            let right_digest = right.structural_digest();
            let claim = claims.iter().find(|claim| {
                (claim.left_package() == left_digest && claim.right_package() == right_digest)
                    || (claim.left_package() == right_digest
                        && claim.right_package() == left_digest)
            });
            let Some(claim) = claim else {
                return Err(ActivationError::InterferenceUnproven);
            };

            if !matches!(
                claim.class(),
                CompositionClass::DisjointSafe
                    | CompositionClass::CertifiedCombination
                    | CompositionClass::ConservativeExtension
            ) {
                return Err(ActivationError::CompositionNotAdmissible);
            }
        }
    }

    Ok(ActivatedPackageSet { digests: requested })
}
