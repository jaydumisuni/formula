use formula_core::{artifacts::AuthorityContract, digest::ArtifactDigest};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteCandidate {
    evidence_family: String,
    verification_mode: String,
    authority_class: String,
    exactness: String,
    checker: ArtifactDigest,
    checker_trust_root: ArtifactDigest,
    cost: u64,
}

impl RouteCandidate {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        evidence_family: String,
        verification_mode: String,
        authority_class: String,
        exactness: String,
        checker: ArtifactDigest,
        checker_trust_root: ArtifactDigest,
        cost: u64,
    ) -> Self {
        Self {
            evidence_family,
            verification_mode,
            authority_class,
            exactness,
            checker,
            checker_trust_root,
            cost,
        }
    }

    pub fn evidence_family(&self) -> &str {
        &self.evidence_family
    }

    pub fn verification_mode(&self) -> &str {
        &self.verification_mode
    }

    pub fn authority_class(&self) -> &str {
        &self.authority_class
    }

    pub fn exactness(&self) -> &str {
        &self.exactness
    }

    pub fn checker(&self) -> ArtifactDigest {
        self.checker
    }

    pub fn checker_trust_root(&self) -> ArtifactDigest {
        self.checker_trust_root
    }

    pub fn cost(&self) -> u64 {
        self.cost
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertificateRoute {
    candidate: RouteCandidate,
}

impl CertificateRoute {
    pub fn candidate(&self) -> &RouteCandidate {
        &self.candidate
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteError {
    NoAdmissibleRoute,
}

pub fn select_certificate_route(
    authority_contract: &AuthorityContract,
    expected_checker: ArtifactDigest,
    expected_trust_root: ArtifactDigest,
    candidates: &[RouteCandidate],
) -> Result<CertificateRoute, RouteError> {
    let exact_contract = authority_contract.requested_authority_class() == "deterministic-proof"
        && authority_contract.exactness_requirement() == "exact";

    let mut admissible: Vec<_> = candidates
        .iter()
        .filter(|candidate| {
            authority_contract
                .allowed_evidence_families()
                .binary_search(&candidate.evidence_family)
                .is_ok()
                && candidate.authority_class == authority_contract.requested_authority_class()
                && candidate.exactness == authority_contract.exactness_requirement()
                && candidate.checker == expected_checker
                && candidate.checker_trust_root == expected_trust_root
                && (!exact_contract
                    || matches!(
                        candidate.verification_mode.as_str(),
                        "FOUNDATIONAL_PROOF"
                            | "INDEPENDENT_CERTIFICATE"
                            | "EXACT_RECOMPUTATION"
                            | "EXHAUSTIVE"
                    ))
        })
        .cloned()
        .collect();

    admissible.sort_by(|left, right| {
        (
            left.cost,
            left.evidence_family.as_str(),
            left.verification_mode.as_str(),
        )
            .cmp(&(
                right.cost,
                right.evidence_family.as_str(),
                right.verification_mode.as_str(),
            ))
    });

    admissible
        .into_iter()
        .next()
        .map(|candidate| CertificateRoute { candidate })
        .ok_or(RouteError::NoAdmissibleRoute)
}
