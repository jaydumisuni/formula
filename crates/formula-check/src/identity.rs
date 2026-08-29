use formula_core::digest::ArtifactDigest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckerDescriptor {
    identity: ArtifactDigest,
}

impl CheckerDescriptor {
    pub fn current() -> Self {
        let identity_material = format!(
            "formula-check:{}:formula-certification-v1",
            env!("CARGO_PKG_VERSION")
        );
        Self {
            identity: ArtifactDigest::of_bytes(identity_material.as_bytes()),
        }
    }

    pub fn identity(&self) -> ArtifactDigest {
        self.identity
    }

    pub fn supports_family(&self, family: &str, version: &str) -> bool {
        version == "1"
            && matches!(
                family,
                "polynomial-identity"
                    | "gf2-witness"
                    | "u8-exhaustive"
                    | "promotion-manifest"
                    | "realization-equivalence"
            )
    }

    pub fn supports_family_name(&self, family: &str) -> bool {
        matches!(
            family,
            "polynomial-identity"
                | "gf2-witness"
                | "u8-exhaustive"
                | "promotion-manifest"
                | "realization-equivalence"
        )
    }
}
