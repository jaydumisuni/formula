use crate::authority_store::{AuthorityStore, AuthorityStoreError, PublishFailpoint};
use formula_check::promotion::PromotionAuthorization;
use formula_core::{
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    promotion::{PromotionRecord, PromotionState},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromotionOutcome {
    parent_generation: ArtifactDigest,
    new_generation: ArtifactDigest,
    admitted_record: PromotionRecord,
}

impl PromotionOutcome {
    pub fn parent_generation(&self) -> ArtifactDigest {
        self.parent_generation
    }

    pub fn new_generation(&self) -> ArtifactDigest {
        self.new_generation
    }

    pub fn admitted_record(&self) -> &PromotionRecord {
        &self.admitted_record
    }
}

impl AuthorityStore {
    pub fn promote(
        &mut self,
        authorization: &PromotionAuthorization,
    ) -> Result<PromotionOutcome, AuthorityStoreError> {
        self.promote_inner(authorization, PublishFailpoint::None)
    }

    pub(crate) fn promote_inner(
        &mut self,
        authorization: &PromotionAuthorization,
        failpoint: PublishFailpoint,
    ) -> Result<PromotionOutcome, AuthorityStoreError> {
        let active = self
            .active_generation()?
            .ok_or(AuthorityStoreError::NoActiveGeneration)?;
        if active != authorization.parent_generation() {
            return Err(AuthorityStoreError::ParentMismatch {
                expected: Some(active),
                actual: Some(authorization.parent_generation()),
            });
        }

        let parent = self.replay_generation(active)?;
        let next_number = parent
            .generation_number()
            .checked_add(1)
            .ok_or(AuthorityStoreError::GenerationNumberOverflow)?;

        let mut admitted = parent.admitted().to_vec();
        admitted.extend_from_slice(authorization.proposed_admissions());
        let mut authority_bindings = parent.authority_bindings().to_vec();
        authority_bindings.extend_from_slice(authorization.authority_bindings());

        let next = UniverseGeneration::new(next_number, Some(active), admitted, authority_bindings);
        let new_generation = self.publish_generation_inner(&next, failpoint)?;
        let admitted_record = PromotionRecord::new(
            authorization.promotion_candidate(),
            PromotionState::Admitted,
            new_generation,
            authorization.policy_digest(),
            authorization.authority_bindings().to_vec(),
            authorization.proposed_admissions().to_vec(),
        );

        Ok(PromotionOutcome {
            parent_generation: active,
            new_generation,
            admitted_record,
        })
    }
}
