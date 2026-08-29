#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CheckFailure {
    CertificateBodyDigestMismatch,
    FrozenCandidateMismatch,
    TargetMismatch,
    GenerationMismatch,
    WorldMismatch,
    DependencyMismatch,
    AuthorityContractMismatch,
    ObserverMismatch,
    CheckerIdentityMismatch,
    CheckerTrustRootMismatch,
    UnsupportedCertificateFamily,
    UnsupportedCertificateFamilyVersion,
    AuthorityInsufficient,
    SemanticMismatch,
    InvalidConstraint,
    TranslationMismatch,
    WitnessWidthMismatch,
    WitnessMismatch,
    U8Counterexample(u8),
    PromotionParentMismatch,
    PromotionEvidenceMismatch,
    PromotionAdmissionMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CheckVerdict {
    Pass,
    Fail(CheckFailure),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityMatch {
    evidence_family: String,
    exactness: String,
    verification_mode: String,
}

impl AuthorityMatch {
    pub(crate) fn new(
        evidence_family: String,
        exactness: String,
        verification_mode: String,
    ) -> Self {
        Self {
            evidence_family,
            exactness,
            verification_mode,
        }
    }

    pub fn evidence_family(&self) -> &str {
        &self.evidence_family
    }

    pub fn exactness(&self) -> &str {
        &self.exactness
    }

    pub fn verification_mode(&self) -> &str {
        &self.verification_mode
    }
}
