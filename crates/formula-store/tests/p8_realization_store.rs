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
};
use formula_store::authority_store::{AuthorityStore, AuthorityStoreError};
use std::fs;
use tempfile::tempdir;

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn promoted_u1(store: &mut AuthorityStore) -> (ArtifactDigest, ArtifactDigest) {
    let primitive = d("fl-c-primitive");
    let evidence = d("checked-certificate");
    let parent = UniverseGeneration::new(0, None, vec![], vec![]);
    let u0 = store.initialize_genesis(&parent).unwrap();
    let frozen = FrozenCandidate::new(
        "fl-c-semantic-primitive".into(),
        vec![primitive],
        d("world"),
        u0,
        vec![],
        vec![],
        d("authority-contract"),
        d("observer"),
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
        panic!("valid P8 fixture promotion was quarantined")
    };
    let u1 = store.promote(&authorization).unwrap().new_generation();
    (primitive, u1)
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

fn realization_authorization(
    semantic_target: ArtifactDigest,
    generation: ArtifactDigest,
    binary: &[u8],
) -> formula_check::realization::RealizationAuthorization {
    let world = d("world");
    let authority = d("authority-contract");
    let observer = d("observer");
    let source = b"fn main() { /* candidate native source */ }\n";
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
    let outputs: Vec<bool> = (0u16..=255)
        .map(|raw| semantic.evaluate(raw as u8))
        .collect();

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

fn context(
    semantic_target: ArtifactDigest,
    generation: ArtifactDigest,
) -> RealizationDispatchContext {
    RealizationDispatchContext::new(
        semantic_target,
        generation,
        d("world"),
        d("authority-contract"),
        d("observer"),
    )
}

#[test]
fn checker_authorization_is_required_for_admission_and_exact_dispatch() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let (primitive, u1) = promoted_u1(&mut store);
    let binary = b"checked-native-binary";
    let authorization = realization_authorization(primitive, u1, binary);

    let admitted = store.admit_realization(&authorization, binary).unwrap();
    assert_eq!(
        admitted.manifest_digest(),
        authorization.realization_manifest()
    );
    assert_eq!(admitted.binary_digest(), authorization.binary_digest());

    let resolved = store.resolve_realization(&context(primitive, u1)).unwrap().unwrap();
    assert_eq!(resolved.manifest_digest(), authorization.realization_manifest());
    assert_eq!(resolved.binary_bytes(), binary);

    let wrong_context = context(primitive, d("different-generation"));
    assert!(store.resolve_realization(&wrong_context).unwrap().is_none());
}

#[test]
fn stale_generation_or_changed_binary_cannot_be_admitted() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let (primitive, u1) = promoted_u1(&mut store);
    let binary = b"checked-native-binary";
    let authorization = realization_authorization(primitive, u1, binary);

    let changed = b"changed-native-binary";
    let binary_error = store.admit_realization(&authorization, changed).unwrap_err();
    assert!(matches!(
        binary_error,
        AuthorityStoreError::RealizationBinaryDigestMismatch { .. }
    ));

    let stale_authorization = realization_authorization(primitive, d("stale-u0"), binary);
    let generation_error = store
        .admit_realization(&stale_authorization, binary)
        .unwrap_err();
    assert!(matches!(
        generation_error,
        AuthorityStoreError::RealizationGenerationMismatch { .. }
    ));
}

#[test]
fn missing_or_tampered_admitted_binary_fails_closed_at_dispatch() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let (primitive, u1) = promoted_u1(&mut store);
    let binary = b"checked-native-binary";
    let authorization = realization_authorization(primitive, u1, binary);
    store.admit_realization(&authorization, binary).unwrap();

    let digest = authorization.binary_digest();
    let hex = digest.hex();
    let blob_path = dir
        .path()
        .join("objects")
        .join("sha256")
        .join(&hex[..2])
        .join(&hex[2..]);
    fs::write(&blob_path, b"tampered-native-binary").unwrap();

    let error = store.resolve_realization(&context(primitive, u1)).unwrap_err();
    assert!(matches!(error, AuthorityStoreError::Blob(_)));
}
