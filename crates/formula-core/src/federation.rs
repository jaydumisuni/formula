use crate::{
    artifacts::StructuralIdentity,
    canonical::CanonicalValue,
    digest::ArtifactDigest,
    theory::{FactPolarity, SharedFact},
};
use std::collections::BTreeMap;

const FEDERATION_SCHEMA_V1: &str = "formula-federation-v1";

fn canonical_object(kind: &str) -> BTreeMap<String, CanonicalValue> {
    BTreeMap::from([
        ("kind".into(), CanonicalValue::String(kind.into())),
        (
            "schema".into(),
            CanonicalValue::String(FEDERATION_SCHEMA_V1.into()),
        ),
    ])
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedFederationFact {
    fact: SharedFact,
    package: ArtifactDigest,
    adapter: ArtifactDigest,
    translation: ArtifactDigest,
    checker_route: ArtifactDigest,
    semantic_input: ArtifactDigest,
    evidence: ArtifactDigest,
}

impl CertifiedFederationFact {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fact: SharedFact,
        package: ArtifactDigest,
        adapter: ArtifactDigest,
        translation: ArtifactDigest,
        checker_route: ArtifactDigest,
        semantic_input: ArtifactDigest,
        evidence: ArtifactDigest,
    ) -> Self {
        Self {
            fact,
            package,
            adapter,
            translation,
            checker_route,
            semantic_input,
            evidence,
        }
    }

    pub fn fact(&self) -> &SharedFact {
        &self.fact
    }

    pub fn package(&self) -> ArtifactDigest {
        self.package
    }

    pub fn adapter(&self) -> ArtifactDigest {
        self.adapter
    }

    pub fn translation(&self) -> ArtifactDigest {
        self.translation
    }

    pub fn checker_route(&self) -> ArtifactDigest {
        self.checker_route
    }

    pub fn semantic_input(&self) -> ArtifactDigest {
        self.semantic_input
    }

    pub fn evidence(&self) -> ArtifactDigest {
        self.evidence
    }
}

impl StructuralIdentity for CertifiedFederationFact {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("CertifiedFederationFact");
        object.insert("fact".into(), self.fact.canonical_value());
        object.insert("package".into(), CanonicalValue::Digest(self.package));
        object.insert("adapter".into(), CanonicalValue::Digest(self.adapter));
        object.insert(
            "translation".into(),
            CanonicalValue::Digest(self.translation),
        );
        object.insert(
            "checker_route".into(),
            CanonicalValue::Digest(self.checker_route),
        );
        object.insert(
            "semantic_input".into(),
            CanonicalValue::Digest(self.semantic_input),
        );
        object.insert("evidence".into(), CanonicalValue::Digest(self.evidence));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeContract {
    source_package: ArtifactDigest,
    target_package: ArtifactDigest,
    source_subject: ArtifactDigest,
    target_subject: ArtifactDigest,
    source_polarity: FactPolarity,
    target_polarity: FactPolarity,
    translation: ArtifactDigest,
    evidence: ArtifactDigest,
}

impl BridgeContract {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_package: ArtifactDigest,
        target_package: ArtifactDigest,
        source_subject: ArtifactDigest,
        target_subject: ArtifactDigest,
        source_polarity: FactPolarity,
        target_polarity: FactPolarity,
        translation: ArtifactDigest,
        evidence: ArtifactDigest,
    ) -> Self {
        Self {
            source_package,
            target_package,
            source_subject,
            target_subject,
            source_polarity,
            target_polarity,
            translation,
            evidence,
        }
    }

    pub fn source_package(&self) -> ArtifactDigest {
        self.source_package
    }

    pub fn target_package(&self) -> ArtifactDigest {
        self.target_package
    }

    pub fn source_subject(&self) -> ArtifactDigest {
        self.source_subject
    }

    pub fn target_subject(&self) -> ArtifactDigest {
        self.target_subject
    }

    pub fn source_polarity(&self) -> FactPolarity {
        self.source_polarity
    }

    pub fn target_polarity(&self) -> FactPolarity {
        self.target_polarity
    }

    pub fn translation(&self) -> ArtifactDigest {
        self.translation
    }

    pub fn evidence(&self) -> ArtifactDigest {
        self.evidence
    }
}

impl StructuralIdentity for BridgeContract {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("BridgeContract");
        object.insert(
            "source_package".into(),
            CanonicalValue::Digest(self.source_package),
        );
        object.insert(
            "target_package".into(),
            CanonicalValue::Digest(self.target_package),
        );
        object.insert(
            "source_subject".into(),
            CanonicalValue::Digest(self.source_subject),
        );
        object.insert(
            "target_subject".into(),
            CanonicalValue::Digest(self.target_subject),
        );
        object.insert(
            "source_polarity".into(),
            CanonicalValue::String(self.source_polarity.as_str().into()),
        );
        object.insert(
            "target_polarity".into(),
            CanonicalValue::String(self.target_polarity.as_str().into()),
        );
        object.insert(
            "translation".into(),
            CanonicalValue::Digest(self.translation),
        );
        object.insert("evidence".into(), CanonicalValue::Digest(self.evidence));
        CanonicalValue::Object(object)
    }
}
