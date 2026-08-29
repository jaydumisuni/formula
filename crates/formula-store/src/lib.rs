pub mod authority_store;
pub mod blob_store;

pub const CRATE_ROLE: &str = "local authority persistence boundary";

#[cfg(test)]
mod atomicity_tests {
    use crate::authority_store::{AuthorityStore, PublishFailpoint};
    use formula_core::{digest::ArtifactDigest, generation::UniverseGeneration};
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
        assert_eq!(store.replay_generation(u0_digest).unwrap().digest(), u0_digest);
    }

    #[test]
    fn failure_after_rows_before_active_rolls_back_authority() {
        assert_failed_publish_rolls_back(PublishFailpoint::AfterRowsBeforeActive);
    }

    #[test]
    fn failure_after_active_before_commit_rolls_back_authority() {
        assert_failed_publish_rolls_back(PublishFailpoint::AfterActiveBeforeCommit);
    }
}
