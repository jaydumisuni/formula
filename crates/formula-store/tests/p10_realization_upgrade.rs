use formula_check::{
    promotion::{PromotionDecision, authorize_promotion_v1},
    realization::authorize_native_u8_realization_v1,
    u8::{BoolExpr, ByteExpr},
};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::{FrozenCandidate, PromotionManifest, RealizationCheckManifest},
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    promotion::PromotionCandidate,
    realization::{
        NativeRealizationManifest, NativeToolchainIdentity, RealizationDispatchContext,
        SpecializationIdentity,
    },
    self_expansion::{RealizationUpgrade, SemanticChangeClass, SupersessionKind},
};
use formula_store::authority_store::{AuthorityStore, AuthorityStoreError};
use tempfile::tempdir;

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn promoted_u1(store: &mut AuthorityStore) -> (ArtifactDigest, ArtifactDigest, ArtifactDigest) {
    let primitive = d("p10:upgrade:primitive");
    let evidence = d("p10:upgrade:evidence");
    let parent = UniverseGeneration::new(0, None, vec![], vec![]);
    let u0 = store.initialize_genesis(&parent).unwrap();
    let frozen = FrozenCandidate::new(
        "p10-upgrade-semantic-primitive".into(),
        vec![primitive],
        d("p10:world"),
        u0,
        vec![],
        vec![],
        d("p10:authority"),
        d("p10:observer"),
    );
    let manifest = PromotionManifest::new(
        u0,
        frozen.structural_digest(),
        vec![evidence],
        vec![primitive],
        vec![evidence],
    );
    let candidate = PromotionCandidate::new(
        frozen.structural_digest(),
        manifest.structural_digest(),
        u0,
        u0,
        vec![],
        vec![],
    );
    let decision =
        authorize_promotion_v1(&manifest, &frozen, &candidate, &[evidence], &parent, &[]).unwrap();
    let PromotionDecision::Authorized(authorization) = decision else {
        panic!("valid P10 realization-upgrade fixture was quarantined")
    };
    let u1 = store.promote(&authorization).unwrap().new_generation();
    (primitive, u1, evidence)
}

fn semantic() -> BoolExpr {
    BoolExpr::And(
        Box::new(BoolExpr::NeqZero(ByteExpr::X)),
        Box::new(BoolExpr::EqZero(ByteExpr::BitAnd(
            Box::new(ByteExpr::X),
            Box::new(ByteExpr::SubWrap(
                Box::new(ByteExpr::X),
                Box::new(ByteExpr::Const(1)),
            )),
        ))),
    )
}

fn authorization(
    semantic_target: ArtifactDigest,
    generation: ArtifactDigest,
    binary: &[u8],
) -> formula_check::realization::RealizationAuthorization {
    let world = d("p10:world");
    let authority = d("p10:authority");
    let observer = d("p10:observer");
    let source = b"fn main() { /* p10 realization variant */ }\n";
    let specialization =
        SpecializationIdentity::new(semantic_target, generation, world, authority, observer);
    let toolchain =
        NativeToolchainIdentity::new("1.98.0".into(), "x86_64-unknown-linux-gnu".into());
    let native_manifest = NativeRealizationManifest::new(
        semantic_target,
        generation,
        world,
        authority,
        observer,
        specialization.structural_digest(),
        ArtifactDigest::of_bytes(source),
        toolchain.structural_digest(),
        ArtifactDigest::of_bytes(binary),
    );
    let check_manifest = RealizationCheckManifest::new(
        semantic_target,
        native_manifest.structural_digest(),
        generation,
        world,
        authority,
        observer,
        ArtifactDigest::of_bytes(binary),
    );
    let semantic = semantic();
    let outputs = (0u16..=255)
        .map(|raw| semantic.evaluate(raw as u8))
        .collect::<Vec<_>>();
    authorize_native_u8_realization_v1(
        &native_manifest,
        &specialization,
        &toolchain,
        &check_manifest,
        source,
        binary,
        &semantic,
        &outputs,
    )
    .unwrap()
}

fn context(target: ArtifactDigest, generation: ArtifactDigest) -> RealizationDispatchContext {
    RealizationDispatchContext::new(
        target,
        generation,
        d("p10:world"),
        d("p10:authority"),
        d("p10:observer"),
    )
}

#[test]
fn faster_admitted_realization_can_replace_selection_without_new_universe_generation() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let (primitive, u1, evidence) = promoted_u1(&mut store);
    let r1 = authorization(primitive, u1, b"p10-native-r1");
    let r2 = authorization(primitive, u1, b"p10-native-r2-faster");
    let admitted_r1 = store.admit_realization(&r1, b"p10-native-r1").unwrap();
    let admitted_r2 = store
        .admit_realization(&r2, b"p10-native-r2-faster")
        .unwrap();
    let before = store.active_generation().unwrap().unwrap();

    let upgrade = RealizationUpgrade::new(
        primitive,
        u1,
        admitted_r1.manifest_digest(),
        admitted_r2.manifest_digest(),
        SemanticChangeClass::RealizationOnly,
        vec![evidence],
        d("p10:selection-policy"),
    );
    store.record_realization_upgrade(&upgrade).unwrap();

    let preferred = store
        .preferred_realization(&context(primitive, u1))
        .unwrap()
        .unwrap();
    assert_eq!(preferred.manifest_digest(), admitted_r2.manifest_digest());
    assert_eq!(store.active_generation().unwrap(), Some(before));
    assert!(store.realization_by_manifest(admitted_r1.manifest_digest()).unwrap().is_some());
    assert_eq!(
        store
            .supersessions_for(admitted_r1.manifest_digest())
            .unwrap()[0]
            .kind(),
        SupersessionKind::ReplacedRealizationBy
    );
}

#[test]
fn realization_upgrade_cannot_smuggle_semantic_admission() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let (primitive, u1, evidence) = promoted_u1(&mut store);
    let r1 = authorization(primitive, u1, b"p10-native-r1");
    let r2 = authorization(primitive, u1, b"p10-native-r2");
    let admitted_r1 = store.admit_realization(&r1, b"p10-native-r1").unwrap();
    let admitted_r2 = store.admit_realization(&r2, b"p10-native-r2").unwrap();

    let upgrade = RealizationUpgrade::new(
        d("p10:not-admitted-semantic"),
        u1,
        admitted_r1.manifest_digest(),
        admitted_r2.manifest_digest(),
        SemanticChangeClass::RealizationOnly,
        vec![evidence],
        d("p10:selection-policy"),
    );
    assert!(matches!(
        store.record_realization_upgrade(&upgrade),
        Err(AuthorityStoreError::RealizationUpgradeSemanticNotAdmitted(_))
    ));
}

#[test]
fn realization_selection_can_roll_back_without_deleting_new_variant_or_changing_u() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let (primitive, u1, evidence) = promoted_u1(&mut store);
    let r1 = authorization(primitive, u1, b"p10-native-r1");
    let r2 = authorization(primitive, u1, b"p10-native-r2");
    let admitted_r1 = store.admit_realization(&r1, b"p10-native-r1").unwrap();
    let admitted_r2 = store.admit_realization(&r2, b"p10-native-r2").unwrap();
    let before = store.active_generation().unwrap().unwrap();
    let ctx = context(primitive, u1);

    let upgrade = RealizationUpgrade::new(
        primitive,
        u1,
        admitted_r1.manifest_digest(),
        admitted_r2.manifest_digest(),
        SemanticChangeClass::RealizationOnly,
        vec![evidence],
        d("p10:selection-policy"),
    );
    store.record_realization_upgrade(&upgrade).unwrap();
    store
        .select_realization(&ctx, admitted_r1.manifest_digest())
        .unwrap();

    assert_eq!(
        store
            .preferred_realization(&ctx)
            .unwrap()
            .unwrap()
            .manifest_digest(),
        admitted_r1.manifest_digest()
    );
    assert!(store.realization_by_manifest(admitted_r2.manifest_digest()).unwrap().is_some());
    assert_eq!(store.active_generation().unwrap(), Some(before));
}
