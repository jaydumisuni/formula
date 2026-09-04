use crate::{
    artifacts::StructuralIdentity, canonical::CanonicalValue, digest::ArtifactDigest,
};
use std::collections::{BTreeMap, BTreeSet};

const FIRST_LIGHT_SCHEMA_V1: &str = "formula-first-light-proof-v1";

fn canonical_object(kind: &str) -> BTreeMap<String, CanonicalValue> {
    BTreeMap::from([
        ("kind".into(), CanonicalValue::String(kind.into())),
        (
            "schema".into(),
            CanonicalValue::String(FIRST_LIGHT_SCHEMA_V1.into()),
        ),
    ])
}

fn sorted_digests(mut values: Vec<ArtifactDigest>) -> Vec<ArtifactDigest> {
    values.sort_unstable();
    values.dedup();
    values
}

fn digest_array(values: &[ArtifactDigest]) -> CanonicalValue {
    CanonicalValue::Array(values.iter().copied().map(CanonicalValue::Digest).collect())
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NegativeControlId {
    ModifiedSealedTarget,
    SealedImportAttempt,
    FlASampleNearMiss,
    FlBCorruptedTranslation,
    FlCZeroNearMiss,
    ForgedEvidence,
    CandidateMutationAfterCertificate,
    SearchAuthorityWrite,
    MutatedRealizationBinary,
    ActivationRemoved,
    StricterAuthorityWithoutEvidence,
    PromotionParentRace,
}

impl NegativeControlId {
    pub const ALL: [Self; 12] = [
        Self::ModifiedSealedTarget,
        Self::SealedImportAttempt,
        Self::FlASampleNearMiss,
        Self::FlBCorruptedTranslation,
        Self::FlCZeroNearMiss,
        Self::ForgedEvidence,
        Self::CandidateMutationAfterCertificate,
        Self::SearchAuthorityWrite,
        Self::MutatedRealizationBinary,
        Self::ActivationRemoved,
        Self::StricterAuthorityWithoutEvidence,
        Self::PromotionParentRace,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ModifiedSealedTarget => "NC-01_MODIFIED_SEALED_TARGET",
            Self::SealedImportAttempt => "NC-02_SEALED_IMPORT_ATTEMPT",
            Self::FlASampleNearMiss => "NC-03_FL_A_SAMPLE_NEAR_MISS",
            Self::FlBCorruptedTranslation => "NC-04_FL_B_CORRUPTED_TRANSLATION",
            Self::FlCZeroNearMiss => "NC-05_FL_C_ZERO_NEAR_MISS",
            Self::ForgedEvidence => "NC-06_FORGED_EVIDENCE",
            Self::CandidateMutationAfterCertificate => {
                "NC-07_CANDIDATE_MUTATION_AFTER_CERTIFICATE"
            }
            Self::SearchAuthorityWrite => "NC-08_SEARCH_AUTHORITY_WRITE",
            Self::MutatedRealizationBinary => "NC-09_MUTATED_REALIZATION_BINARY",
            Self::ActivationRemoved => "NC-10_ACTIVATION_REMOVED",
            Self::StricterAuthorityWithoutEvidence => {
                "NC-11_STRICTER_AUTHORITY_WITHOUT_EVIDENCE"
            }
            Self::PromotionParentRace => "NC-12_PROMOTION_PARENT_RACE",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NegativeControlEvidence {
    id: NegativeControlId,
    evidence_digest: ArtifactDigest,
}

impl NegativeControlEvidence {
    pub fn new(id: NegativeControlId, evidence_digest: ArtifactDigest) -> Self {
        Self {
            id,
            evidence_digest,
        }
    }

    pub fn id(&self) -> NegativeControlId {
        self.id
    }

    pub fn evidence_digest(&self) -> ArtifactDigest {
        self.evidence_digest
    }
}

impl StructuralIdentity for NegativeControlEvidence {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("NegativeControlEvidence");
        object.insert(
            "evidence_digest".into(),
            CanonicalValue::Digest(self.evidence_digest),
        );
        object.insert(
            "id".into(),
            CanonicalValue::String(self.id.as_str().into()),
        );
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FirstLightManifestError {
    MissingNegativeControl(NegativeControlId),
    DuplicateNegativeControl(NegativeControlId),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NegativeControlManifest {
    controls: Vec<NegativeControlEvidence>,
}

impl NegativeControlManifest {
    pub fn complete(
        mut controls: Vec<NegativeControlEvidence>,
    ) -> Result<Self, FirstLightManifestError> {
        controls.sort_by_key(NegativeControlEvidence::id);
        let mut seen = BTreeSet::new();
        for control in &controls {
            if !seen.insert(control.id()) {
                return Err(FirstLightManifestError::DuplicateNegativeControl(control.id()));
            }
        }
        for required in NegativeControlId::ALL {
            if !seen.contains(&required) {
                return Err(FirstLightManifestError::MissingNegativeControl(required));
            }
        }
        Ok(Self { controls })
    }

    pub fn controls(&self) -> &[NegativeControlEvidence] {
        &self.controls
    }

    pub fn is_complete(&self) -> bool {
        self.controls.len() == NegativeControlId::ALL.len()
            && NegativeControlId::ALL
                .iter()
                .all(|id| self.controls.iter().any(|control| control.id() == *id))
    }
}

impl StructuralIdentity for NegativeControlManifest {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("NegativeControlManifest");
        object.insert(
            "controls".into(),
            CanonicalValue::Array(
                self.controls
                    .iter()
                    .map(StructuralIdentity::canonical_value)
                    .collect(),
            ),
        );
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FirstLightTargetEvidence {
    query: ArtifactDigest,
    campaign: ArtifactDigest,
    candidate: ArtifactDigest,
    certification: ArtifactDigest,
    auxiliary: Vec<ArtifactDigest>,
}

impl FirstLightTargetEvidence {
    pub fn new(
        query: ArtifactDigest,
        campaign: ArtifactDigest,
        candidate: ArtifactDigest,
        certification: ArtifactDigest,
        auxiliary: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            query,
            campaign,
            candidate,
            certification,
            auxiliary: sorted_digests(auxiliary),
        }
    }

    pub fn query(&self) -> ArtifactDigest {
        self.query
    }
    pub fn campaign(&self) -> ArtifactDigest {
        self.campaign
    }
    pub fn candidate(&self) -> ArtifactDigest {
        self.candidate
    }
    pub fn certification(&self) -> ArtifactDigest {
        self.certification
    }
    pub fn auxiliary(&self) -> &[ArtifactDigest] {
        &self.auxiliary
    }
}

impl StructuralIdentity for FirstLightTargetEvidence {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("FirstLightTargetEvidence");
        object.insert("query".into(), CanonicalValue::Digest(self.query));
        object.insert("campaign".into(), CanonicalValue::Digest(self.campaign));
        object.insert("candidate".into(), CanonicalValue::Digest(self.candidate));
        object.insert(
            "certification".into(),
            CanonicalValue::Digest(self.certification),
        );
        object.insert("auxiliary".into(), digest_array(&self.auxiliary));
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FirstLightNativeEvidence {
    source: ArtifactDigest,
    toolchain: ArtifactDigest,
    binary: ArtifactDigest,
    realization: ArtifactDigest,
}

impl FirstLightNativeEvidence {
    pub fn new(
        source: ArtifactDigest,
        toolchain: ArtifactDigest,
        binary: ArtifactDigest,
        realization: ArtifactDigest,
    ) -> Self {
        Self {
            source,
            toolchain,
            binary,
            realization,
        }
    }

    pub fn source(&self) -> ArtifactDigest {
        self.source
    }
    pub fn toolchain(&self) -> ArtifactDigest {
        self.toolchain
    }
    pub fn binary(&self) -> ArtifactDigest {
        self.binary
    }
    pub fn realization(&self) -> ArtifactDigest {
        self.realization
    }
}

impl StructuralIdentity for FirstLightNativeEvidence {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("FirstLightNativeEvidence");
        object.insert("source".into(), CanonicalValue::Digest(self.source));
        object.insert("toolchain".into(), CanonicalValue::Digest(self.toolchain));
        object.insert("binary".into(), CanonicalValue::Digest(self.binary));
        object.insert(
            "realization".into(),
            CanonicalValue::Digest(self.realization),
        );
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FirstLightReuseEvidence {
    query: ArtifactDigest,
    campaign: ArtifactDigest,
    resolved_capability: ArtifactDigest,
    execution_plan: ArtifactDigest,
    result: ArtifactDigest,
    reuse_metrics: ArtifactDigest,
    realization: ArtifactDigest,
}

impl FirstLightReuseEvidence {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        query: ArtifactDigest,
        campaign: ArtifactDigest,
        resolved_capability: ArtifactDigest,
        execution_plan: ArtifactDigest,
        result: ArtifactDigest,
        reuse_metrics: ArtifactDigest,
        realization: ArtifactDigest,
    ) -> Self {
        Self {
            query,
            campaign,
            resolved_capability,
            execution_plan,
            result,
            reuse_metrics,
            realization,
        }
    }

    pub fn query(&self) -> ArtifactDigest {
        self.query
    }
    pub fn campaign(&self) -> ArtifactDigest {
        self.campaign
    }
    pub fn resolved_capability(&self) -> ArtifactDigest {
        self.resolved_capability
    }
    pub fn execution_plan(&self) -> ArtifactDigest {
        self.execution_plan
    }
    pub fn result(&self) -> ArtifactDigest {
        self.result
    }
    pub fn reuse_metrics(&self) -> ArtifactDigest {
        self.reuse_metrics
    }
    pub fn realization(&self) -> ArtifactDigest {
        self.realization
    }
}

impl StructuralIdentity for FirstLightReuseEvidence {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("FirstLightReuseEvidence");
        object.insert("query".into(), CanonicalValue::Digest(self.query));
        object.insert("campaign".into(), CanonicalValue::Digest(self.campaign));
        object.insert(
            "resolved_capability".into(),
            CanonicalValue::Digest(self.resolved_capability),
        );
        object.insert(
            "execution_plan".into(),
            CanonicalValue::Digest(self.execution_plan),
        );
        object.insert("result".into(), CanonicalValue::Digest(self.result));
        object.insert(
            "reuse_metrics".into(),
            CanonicalValue::Digest(self.reuse_metrics),
        );
        object.insert(
            "realization".into(),
            CanonicalValue::Digest(self.realization),
        );
        CanonicalValue::Object(object)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FirstLightProofManifest {
    source_commit: String,
    u0_digest: ArtifactDigest,
    u1_digest: ArtifactDigest,
    world: ArtifactDigest,
    activated_package_set: ArtifactDigest,
    fl_a: FirstLightTargetEvidence,
    fl_b: FirstLightTargetEvidence,
    fl_c: FirstLightTargetEvidence,
    promotion_digest: ArtifactDigest,
    closure_before: ArtifactDigest,
    closure_after: ArtifactDigest,
    closure_delta: ArtifactDigest,
    native: FirstLightNativeEvidence,
    reuse: FirstLightReuseEvidence,
    negative_controls: ArtifactDigest,
    verifier_identity: ArtifactDigest,
    checker_identity: ArtifactDigest,
}

impl FirstLightProofManifest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_commit: String,
        u0_digest: ArtifactDigest,
        u1_digest: ArtifactDigest,
        world: ArtifactDigest,
        activated_package_set: ArtifactDigest,
        fl_a: FirstLightTargetEvidence,
        fl_b: FirstLightTargetEvidence,
        fl_c: FirstLightTargetEvidence,
        promotion_digest: ArtifactDigest,
        closure_before: ArtifactDigest,
        closure_after: ArtifactDigest,
        closure_delta: ArtifactDigest,
        native: FirstLightNativeEvidence,
        reuse: FirstLightReuseEvidence,
        negative_controls: NegativeControlManifest,
        verifier_identity: ArtifactDigest,
        checker_identity: ArtifactDigest,
    ) -> Self {
        Self {
            source_commit,
            u0_digest,
            u1_digest,
            world,
            activated_package_set,
            fl_a,
            fl_b,
            fl_c,
            promotion_digest,
            closure_before,
            closure_after,
            closure_delta,
            native,
            reuse,
            negative_controls: negative_controls.structural_digest(),
            verifier_identity,
            checker_identity,
        }
    }

    pub fn source_commit(&self) -> &str {
        &self.source_commit
    }
    pub fn u0_digest(&self) -> ArtifactDigest {
        self.u0_digest
    }
    pub fn u1_digest(&self) -> ArtifactDigest {
        self.u1_digest
    }
    pub fn world(&self) -> ArtifactDigest {
        self.world
    }
    pub fn activated_package_set(&self) -> ArtifactDigest {
        self.activated_package_set
    }
    pub fn fl_a(&self) -> &FirstLightTargetEvidence {
        &self.fl_a
    }
    pub fn fl_b(&self) -> &FirstLightTargetEvidence {
        &self.fl_b
    }
    pub fn fl_c(&self) -> &FirstLightTargetEvidence {
        &self.fl_c
    }
    pub fn promotion_digest(&self) -> ArtifactDigest {
        self.promotion_digest
    }
    pub fn closure_before(&self) -> ArtifactDigest {
        self.closure_before
    }
    pub fn closure_after(&self) -> ArtifactDigest {
        self.closure_after
    }
    pub fn closure_delta(&self) -> ArtifactDigest {
        self.closure_delta
    }
    pub fn native(&self) -> &FirstLightNativeEvidence {
        &self.native
    }
    pub fn reuse(&self) -> &FirstLightReuseEvidence {
        &self.reuse
    }
    pub fn negative_controls(&self) -> ArtifactDigest {
        self.negative_controls
    }
    pub fn verifier_identity(&self) -> ArtifactDigest {
        self.verifier_identity
    }
    pub fn checker_identity(&self) -> ArtifactDigest {
        self.checker_identity
    }
}

impl StructuralIdentity for FirstLightProofManifest {
    fn canonical_value(&self) -> CanonicalValue {
        let mut object = canonical_object("FirstLightProofManifest");
        object.insert(
            "source_commit".into(),
            CanonicalValue::String(self.source_commit.clone()),
        );
        object.insert("u0_digest".into(), CanonicalValue::Digest(self.u0_digest));
        object.insert("u1_digest".into(), CanonicalValue::Digest(self.u1_digest));
        object.insert("world".into(), CanonicalValue::Digest(self.world));
        object.insert(
            "activated_package_set".into(),
            CanonicalValue::Digest(self.activated_package_set),
        );
        object.insert("fl_a".into(), self.fl_a.canonical_value());
        object.insert("fl_b".into(), self.fl_b.canonical_value());
        object.insert("fl_c".into(), self.fl_c.canonical_value());
        object.insert(
            "promotion_digest".into(),
            CanonicalValue::Digest(self.promotion_digest),
        );
        object.insert(
            "closure_before".into(),
            CanonicalValue::Digest(self.closure_before),
        );
        object.insert(
            "closure_after".into(),
            CanonicalValue::Digest(self.closure_after),
        );
        object.insert(
            "closure_delta".into(),
            CanonicalValue::Digest(self.closure_delta),
        );
        object.insert("native".into(), self.native.canonical_value());
        object.insert("reuse".into(), self.reuse.canonical_value());
        object.insert(
            "negative_controls".into(),
            CanonicalValue::Digest(self.negative_controls),
        );
        object.insert(
            "verifier_identity".into(),
            CanonicalValue::Digest(self.verifier_identity),
        );
        object.insert(
            "checker_identity".into(),
            CanonicalValue::Digest(self.checker_identity),
        );
        CanonicalValue::Object(object)
    }
}
