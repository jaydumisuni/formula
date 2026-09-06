pub mod authority_store;
pub mod blob_store;
pub mod bootstrap_store;
pub mod promotion_store;

pub const CRATE_ROLE: &str = "local authority persistence boundary";

#[cfg(test)]
mod atomicity_tests {
    use crate::authority_store::{AuthorityStore, PublishFailpoint};
    use formula_check::promotion::{PromotionDecision, authorize_promotion_v1};
    use formula_core::{
        artifacts::StructuralIdentity,
        certification::{FrozenCandidate, PromotionManifest},
        digest::ArtifactDigest,
        generation::UniverseGeneration,
        promotion::PromotionCandidate,
    };
    use tempfile::tempdir;

    fn d(value: &[u8]) -> ArtifactDigest {
        ArtifactDigest::of_bytes(value)
    }

    fn assert_failed_publish_rolls_back(failpoint: PublishFailpoint) {
        let dir = tempdir().unwrap();
        let mut store = AuthorityStore::open(dir.path()).unwrap();
        let u0 = UniverseGeneration::new(0, None, vec![d(b"base")], vec![]);
        let u0_digest = store.initialize_genesis(&u0).unwrap();
        let u1 = UniverseGeneration::new(
            1,
            Some(u0_digest),
            vec![d(b"base"), d(b"new")],
            vec![d(b"proof")],
        );
        let u1_digest = u1.digest();

        assert!(store.publish_generation_inner(&u1, failpoint).is_err());
        assert_eq!(store.active_generation().unwrap(), Some(u0_digest));
        assert!(store.replay_generation(u1_digest).is_err());
        assert_eq!(
            store.replay_generation(u0_digest).unwrap().digest(),
            u0_digest
        );
    }

    fn valid_authorization(
        parent: &UniverseGeneration,
    ) -> formula_check::promotion::PromotionAuthorization {
        let primitive = d(b"promotion-failpoint-primitive");
        let evidence = d(b"promotion-failpoint-evidence");
        let parent_digest = parent.digest();
        let frozen = FrozenCandidate::new(
            "promotion-failpoint-primitive".into(),
            vec![primitive],
            d(b"world"),
            parent_digest,
            vec![],
            vec![],
            d(b"authority-contract"),
            d(b"observer"),
        );
        let manifest = PromotionManifest::new(
            parent_digest,
            frozen.structural_digest(),
            vec![evidence],
            vec![primitive],
            vec![evidence],
        );
        let candidate = PromotionCandidate::new(
            frozen.structural_digest(),
            manifest.structural_digest(),
            parent_digest,
            parent_digest,
            vec![],
            vec![],
        );
        let decision =
            authorize_promotion_v1(&manifest, &frozen, &candidate, &[evidence], parent, &[])
                .unwrap();
        let PromotionDecision::Authorized(authorization) = decision else {
            panic!("valid promotion failpoint fixture was quarantined")
        };
        authorization
    }

    fn assert_failed_promotion_rolls_back(failpoint: PublishFailpoint) {
        let dir = tempdir().unwrap();
        let mut store = AuthorityStore::open(dir.path()).unwrap();
        let u0 = UniverseGeneration::new(0, None, vec![d(b"base")], vec![]);
        let u0_digest = store.initialize_genesis(&u0).unwrap();
        let u0_bytes = u0.canonical_bytes();
        let authorization = valid_authorization(&u0);
        let u1 = UniverseGeneration::new(
            1,
            Some(u0_digest),
            vec![d(b"base"), d(b"promotion-failpoint-primitive")],
            vec![d(b"promotion-failpoint-evidence")],
        );
        let u1_digest = u1.digest();

        assert!(store.promote_inner(&authorization, failpoint).is_err());
        assert_eq!(store.active_generation().unwrap(), Some(u0_digest));
        assert!(store.replay_generation(u1_digest).is_err());
        let replayed_u0 = store.replay_generation(u0_digest).unwrap();
        assert_eq!(replayed_u0.digest(), u0_digest);
        assert_eq!(replayed_u0.canonical_bytes(), u0_bytes);
    }

    #[test]
    fn failure_after_rows_before_active_rolls_back_authority() {
        assert_failed_publish_rolls_back(PublishFailpoint::AfterRowsBeforeActive);
    }

    #[test]
    fn failure_after_active_before_commit_rolls_back_authority() {
        assert_failed_publish_rolls_back(PublishFailpoint::AfterActiveBeforeCommit);
    }

    #[test]
    fn promotion_failure_after_rows_before_active_preserves_u0_history() {
        assert_failed_promotion_rolls_back(PublishFailpoint::AfterRowsBeforeActive);
    }

    #[test]
    fn promotion_failure_after_active_before_commit_preserves_u0_history() {
        assert_failed_promotion_rolls_back(PublishFailpoint::AfterActiveBeforeCommit);
    }
}
