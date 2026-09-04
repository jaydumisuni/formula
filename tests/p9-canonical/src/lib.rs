use formula_check::{
    envelope::{CheckRequest, validate_envelope},
    first_light::{
        FirstLightReplayEvidence, checker_identity_v1, verifier_identity_v1,
        verify_first_light_manifest_v1,
    },
    gf2::{BooleanXorRow, BooleanXorSystem, Gf2Row, Gf2System, check_gf2_witness},
    identity::CheckerDescriptor,
    polynomial::{IntegerPolynomial, check_polynomial_identity},
    promotion::{PromotionDecision, authorize_promotion_v1},
    realization::authorize_native_u8_realization_v1,
    u8::{BoolExpr as CheckedBoolExpr, ByteExpr as CheckedByteExpr},
    verdict::{CheckFailure, CheckVerdict},
};
use formula_core::{
    artifacts::{AuthorityContract, Observer, StructuralIdentity},
    canonical::CanonicalValue,
    certification::{
        CertificateEnvelope, FrozenCandidate as CertifiedFrozenCandidate, PromotionManifest,
        RealizationCheckManifest,
    },
    digest::ArtifactDigest,
    first_light::{
        FirstLightNativeEvidence, FirstLightProofManifest, FirstLightReuseEvidence,
        FirstLightTargetEvidence, NegativeControlEvidence, NegativeControlId,
        NegativeControlManifest,
    },
    generation::UniverseGeneration,
    promotion::{PromotionCandidate, PromotionRecord, PromotionState},
    realization::{
        NativeRealizationManifest, NativeToolchainIdentity, RealizationDispatchContext,
        SpecializationIdentity,
    },
    theory::ClosureContext,
};
use formula_engine::{
    affine_polynomial::{AffinePolynomialSpace, Rational128},
    candidate_space::CandidateSpaceContext,
    compiler::{CompilerError, CompilerInputs, CompilerV1},
    observational::{
        BoolExpr as EngineBoolExpr, ByteExpr as EngineByteExpr, ObservationalExprSpace,
        U8BoolGrammar,
    },
    query::{
        ActivatedPackageBinding, KnownBinding, QueryIR, RequestedResultClass, ResourceContract,
        SideEffectPolicy, TargetRequest,
    },
    region::CompilerAuthoritySnapshot,
    reuse::ReuseRequest,
    route_space::{ReductionRouteSpace, RouteCandidate},
};
use formula_first_light::{
    fl_a::{fl_a_oracle, fl_a_target_digest},
    fl_b::{
        fl_b_direct_route_digest, fl_b_gf2_route_digest, fl_b_problem_digest,
        fl_b_public_problem, fl_b_route_contract_digest,
    },
    fl_c::{fl_c_grammar_digest, fl_c_oracle, fl_c_target_digest, fl_c_zero_near_miss},
    reuse::{SecondQueryResult, canonical_second_query_vector},
};
use formula_packages::{
    activation::validate_activation,
    closure::{
        CapabilityClosure, CapabilityClosureDelta, derive_capabilities_with_semantic_activations,
    },
};
use formula_realize::rust_native::generate_u8_bool_rust_source;
use formula_store::authority_store::{AuthorityStore, AuthorityStoreError};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use tempfile::tempdir;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

const EXPECTED_FL_A_COEFFICIENTS: [i64; 7] = [1, 7, 21, 35, 35, 21, 7];

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn evidence_digest(label: &str, detail: impl AsRef<str>) -> ArtifactDigest {
    ArtifactDigest::of_bytes(format!("formula-p9-evidence-v1|{label}|{}", detail.as_ref()).as_bytes())
}

fn checked_byte(expression: &EngineByteExpr) -> CheckedByteExpr {
    match expression {
        EngineByteExpr::X => CheckedByteExpr::X,
        EngineByteExpr::Zero => CheckedByteExpr::Const(0),
        EngineByteExpr::One => CheckedByteExpr::Const(1),
        EngineByteExpr::SubWrap(left, right) => {
            CheckedByteExpr::SubWrap(Box::new(checked_byte(left)), Box::new(checked_byte(right)))
        }
        EngineByteExpr::BitAnd(left, right) => {
            CheckedByteExpr::BitAnd(Box::new(checked_byte(left)), Box::new(checked_byte(right)))
        }
    }
}

fn checked_bool(expression: &EngineBoolExpr) -> CheckedBoolExpr {
    match expression {
        EngineBoolExpr::EqZero(value) => CheckedBoolExpr::EqZero(checked_byte(value)),
        EngineBoolExpr::NeqZero(value) => CheckedBoolExpr::NeqZero(checked_byte(value)),
        EngineBoolExpr::And(left, right) => {
            CheckedBoolExpr::And(Box::new(checked_bool(left)), Box::new(checked_bool(right)))
        }
    }
}

fn canonical_rust_toolchain() -> (NativeToolchainIdentity, String, String) {
    let output = Command::new("rustc")
        .arg("-vV")
        .output()
        .expect("pinned rustc must be executable");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let version = String::from_utf8(output.stdout).expect("rustc -vV must be UTF-8");
    let release = version
        .lines()
        .find_map(|line| line.strip_prefix("release: "))
        .expect("rustc release evidence")
        .to_owned();
    let host = version
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .expect("rustc host evidence")
        .to_owned();
    assert_eq!(release, "1.98.0");
    (
        NativeToolchainIdentity::new(release.clone(), host.clone()),
        release,
        host,
    )
}

fn execute_native(binary: &Path, input: u8) -> bool {
    let output = Command::new(binary)
        .arg(input.to_string())
        .output()
        .expect("admitted native realization must execute");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    match output.stdout.as_slice() {
        b"0\n" => false,
        b"1\n" => true,
        other => panic!("non-canonical admitted realization output: {other:?}"),
    }
}

fn closure_digest(closure: &CapabilityClosure) -> ArtifactDigest {
    let mut rendered = format!(
        "formula-p9-capability-closure-v1|{}",
        closure.context_digest().as_str()
    );
    for capability in closure.capabilities() {
        rendered.push('|');
        rendered.push_str(&capability.as_str());
    }
    ArtifactDigest::of_bytes(rendered.as_bytes())
}

fn closure_delta_digest(delta: &CapabilityClosureDelta) -> ArtifactDigest {
    let mut rendered = format!(
        "formula-p9-capability-closure-delta-v1|{}|{}",
        delta.before_context_digest().as_str(),
        delta.after_context_digest().as_str()
    );
    for capability in delta.added() {
        rendered.push_str("|+");
        rendered.push_str(&capability.as_str());
    }
    for capability in delta.removed() {
        rendered.push_str("|-");
        rendered.push_str(&capability.as_str());
    }
    ArtifactDigest::of_bytes(rendered.as_bytes())
}

fn package_set_digest(generation: ArtifactDigest, packages: &[ArtifactDigest]) -> ArtifactDigest {
    let mut rendered = format!("formula-p9-activated-package-set-v1|{}", generation.as_str());
    for package in packages {
        rendered.push('|');
        rendered.push_str(&package.as_str());
    }
    ArtifactDigest::of_bytes(rendered.as_bytes())
}

fn fl_a_proof(
    u0: ArtifactDigest,
    world: ArtifactDigest,
) -> (FirstLightTargetEvidence, NegativeControlEvidence) {
    let context = CandidateSpaceContext::new(
        u0,
        world,
        d("p9-fl-a-query"),
        d("p9-fl-a-obligation"),
        fl_a_target_digest(),
        d("p9-fl-a-search-policy"),
    );
    let mut space = AffinePolynomialSpace::new(context.clone(), 6);
    for n in 0_i128..=6 {
        space
            .add_exact_sample(n, fl_a_oracle().sample(n).unwrap())
            .unwrap();
    }
    let candidate = space.extract_min_degree_unique().unwrap();
    let expected_rationals: Vec<_> = EXPECTED_FL_A_COEFFICIENTS
        .iter()
        .copied()
        .map(|value| Rational128::integer(value.into()))
        .collect();
    assert_eq!(candidate.coefficients(), expected_rationals.as_slice());

    let expected = IntegerPolynomial::from_i64(&EXPECTED_FL_A_COEFFICIENTS);
    let candidate_polynomial = IntegerPolynomial::from_i64(&EXPECTED_FL_A_COEFFICIENTS);
    assert_eq!(
        check_polynomial_identity(&expected, &candidate_polynomial),
        CheckVerdict::Pass
    );
    let certification = evidence_digest("fl-a-certification", candidate.digest().as_str());

    let near_miss = |n: i128| {
        let target = fl_a_oracle().sample(n).unwrap();
        let vanishing = (0_i128..=6).map(|i| n - i).product::<i128>();
        target + vanishing
    };
    for n in 0_i128..=6 {
        assert_eq!(near_miss(n), fl_a_oracle().sample(n).unwrap());
    }
    let discriminating = 7_i128;
    let expected_at_7 = fl_a_oracle().sample(discriminating).unwrap();
    let near_at_7 = near_miss(discriminating);
    assert_ne!(near_at_7, expected_at_7);
    let nc = NegativeControlEvidence::new(
        NegativeControlId::FlASampleNearMiss,
        evidence_digest(
            NegativeControlId::FlASampleNearMiss.as_str(),
            format!("n=7|expected={expected_at_7}|near={near_at_7}"),
        ),
    );

    (
        FirstLightTargetEvidence::new(
            context.query_digest(),
            space.freeze().digest(),
            candidate.digest(),
            certification,
            vec![fl_a_target_digest(), context.digest()],
        ),
        nc,
    )
}

fn fl_b_proof(
    u0: ArtifactDigest,
    world: ArtifactDigest,
) -> (FirstLightTargetEvidence, NegativeControlEvidence) {
    let context = CandidateSpaceContext::new(
        u0,
        world,
        d("p9-fl-b-query"),
        d("p9-fl-b-obligation"),
        fl_b_route_contract_digest(),
        d("p9-fl-b-search-policy"),
    );
    let direct = RouteCandidate::new(
        fl_b_direct_route_digest(),
        "boolean-xor",
        "boolean-direct",
        vec![RequestedResultClass::Witness],
        vec![fl_b_route_contract_digest()],
        true,
        9,
    );
    let gf2 = RouteCandidate::new(
        fl_b_gf2_route_digest(),
        "boolean-xor",
        "gf2",
        vec![RequestedResultClass::Witness],
        vec![fl_b_route_contract_digest()],
        true,
        2,
    );
    let mut routes = ReductionRouteSpace::new(context.clone(), vec![direct, gf2]);
    routes.restrict_result_class(RequestedResultClass::Witness);
    routes.restrict_capabilities(&[fl_b_route_contract_digest()]);
    let selected = routes.extract_min_cost().expect("canonical FL-B route");
    assert_eq!(selected.route_digest(), fl_b_gf2_route_digest());

    let public = fl_b_public_problem();
    let boolean = BooleanXorSystem::new(
        public.width(),
        public
            .rows()
            .iter()
            .map(|row| BooleanXorRow::new(row.variables().to_vec(), row.rhs()))
            .collect(),
    );
    let translated = Gf2System::new(
        public.width(),
        public
            .rows()
            .iter()
            .map(|row| Gf2Row::new(row.variables().to_vec(), row.rhs()))
            .collect(),
    );
    let witness: Vec<bool> = (0..public.width()).map(|index| index % 2 == 0).collect();
    assert_eq!(
        check_gf2_witness(&boolean, &translated, &witness),
        CheckVerdict::Pass
    );

    let corrupted = Gf2System::new(
        public.width(),
        public
            .rows()
            .iter()
            .enumerate()
            .map(|(index, row)| {
                Gf2Row::new(
                    row.variables().to_vec(),
                    if index == 0 { !row.rhs() } else { row.rhs() },
                )
            })
            .collect(),
    );
    let corrupted_verdict = check_gf2_witness(&boolean, &corrupted, &witness);
    assert_ne!(corrupted_verdict, CheckVerdict::Pass);
    let nc = NegativeControlEvidence::new(
        NegativeControlId::FlBCorruptedTranslation,
        evidence_digest(
            NegativeControlId::FlBCorruptedTranslation.as_str(),
            format!("{corrupted_verdict:?}"),
        ),
    );

    let frozen_candidate = routes.freeze_candidate(&selected);
    (
        FirstLightTargetEvidence::new(
            context.query_digest(),
            routes.freeze().digest(),
            frozen_candidate.digest(),
            evidence_digest("fl-b-certification", fl_b_gf2_route_digest().as_str()),
            vec![
                fl_b_problem_digest(),
                fl_b_direct_route_digest(),
                fl_b_gf2_route_digest(),
                fl_b_route_contract_digest(),
            ],
        ),
        nc,
    )
}

struct EnvelopeFixture {
    authority: AuthorityContract,
    observer: Observer,
    candidate: CertifiedFrozenCandidate,
    dependencies: Vec<ArtifactDigest>,
    target: ArtifactDigest,
    generation: ArtifactDigest,
    world: ArtifactDigest,
    body: Vec<u8>,
    trust_root: ArtifactDigest,
}

fn envelope_fixture() -> EnvelopeFixture {
    let authority = AuthorityContract::new(
        "deterministic-proof".into(),
        vec!["u8-exhaustive".into()],
        "exact".into(),
    );
    let observer = Observer::new("full-value".into(), CanonicalValue::String("exact".into()));
    let target = fl_a_target_digest();
    let generation = d("p9-nc-generation");
    let world = d("p9-nc-world");
    let dependencies = vec![d("p9-nc-dependency")];
    let candidate = CertifiedFrozenCandidate::new(
        "p9-negative-control-candidate".into(),
        vec![d("p9-nc-semantic")],
        world,
        generation,
        dependencies.clone(),
        vec![target],
        authority.structural_digest(),
        observer.structural_digest(),
    );
    EnvelopeFixture {
        authority,
        observer,
        candidate,
        dependencies,
        target,
        generation,
        world,
        body: b"p9-authentic-certificate-body".to_vec(),
        trust_root: d("p9-nc-trust-root"),
    }
}

fn envelope_for(
    fixture: &EnvelopeFixture,
    candidate: &CertifiedFrozenCandidate,
    target: ArtifactDigest,
    mode: &str,
    body: &[u8],
) -> CertificateEnvelope {
    CertificateEnvelope::new(
        candidate.structural_digest(),
        target,
        fixture.generation,
        fixture.world,
        CanonicalValue::String("universal".into()),
        "PROVED".into(),
        mode.into(),
        "u8-exhaustive".into(),
        "1".into(),
        ArtifactDigest::of_bytes(body),
        d("p9-nc-producer"),
        CheckerDescriptor::current().identity(),
        fixture.trust_root,
        fixture.dependencies.clone(),
        fixture.authority.structural_digest(),
        fixture.observer.structural_digest(),
        CanonicalValue::String("replay-v1".into()),
    )
}

fn validate_fixture(
    fixture: &EnvelopeFixture,
    envelope: &CertificateEnvelope,
    candidate: &CertifiedFrozenCandidate,
    body: &[u8],
) -> Result<formula_check::verdict::AuthorityMatch, CheckFailure> {
    validate_envelope(&CheckRequest::new(
        envelope,
        candidate,
        fixture.target,
        fixture.generation,
        fixture.world,
        &fixture.dependencies,
        &fixture.authority,
        &fixture.observer,
        body,
        fixture.trust_root,
    ))
}

fn envelope_negative_controls() -> Vec<NegativeControlEvidence> {
    let fixture = envelope_fixture();

    let modified_target = d("p9-modified-sealed-target");
    let target_envelope = envelope_for(
        &fixture,
        &fixture.candidate,
        modified_target,
        "EXHAUSTIVE",
        &fixture.body,
    );
    let target_error = validate_fixture(
        &fixture,
        &target_envelope,
        &fixture.candidate,
        &fixture.body,
    )
    .unwrap_err();
    assert_eq!(target_error, CheckFailure::TargetMismatch);

    let authentic = envelope_for(
        &fixture,
        &fixture.candidate,
        fixture.target,
        "EXHAUSTIVE",
        &fixture.body,
    );
    let forged_error = validate_fixture(
        &fixture,
        &authentic,
        &fixture.candidate,
        b"p9-forged-certificate-body",
    )
    .unwrap_err();
    assert_eq!(forged_error, CheckFailure::CertificateBodyDigestMismatch);

    let changed_candidate = CertifiedFrozenCandidate::new(
        "p9-negative-control-candidate".into(),
        vec![d("p9-nc-mutated-semantic")],
        fixture.world,
        fixture.generation,
        fixture.dependencies.clone(),
        vec![fixture.target],
        fixture.authority.structural_digest(),
        fixture.observer.structural_digest(),
    );
    let changed_error = validate_fixture(&fixture, &authentic, &changed_candidate, &fixture.body)
        .unwrap_err();
    assert_eq!(changed_error, CheckFailure::FrozenCandidateMismatch);

    let weak = envelope_for(
        &fixture,
        &fixture.candidate,
        fixture.target,
        "PROBABILISTIC",
        &fixture.body,
    );
    let weak_error = validate_fixture(&fixture, &weak, &fixture.candidate, &fixture.body)
        .unwrap_err();
    assert_eq!(weak_error, CheckFailure::AuthorityInsufficient);

    vec![
        NegativeControlEvidence::new(
            NegativeControlId::ModifiedSealedTarget,
            evidence_digest(
                NegativeControlId::ModifiedSealedTarget.as_str(),
                format!("{target_error:?}"),
            ),
        ),
        NegativeControlEvidence::new(
            NegativeControlId::ForgedEvidence,
            evidence_digest(
                NegativeControlId::ForgedEvidence.as_str(),
                format!("{forged_error:?}"),
            ),
        ),
        NegativeControlEvidence::new(
            NegativeControlId::CandidateMutationAfterCertificate,
            evidence_digest(
                NegativeControlId::CandidateMutationAfterCertificate.as_str(),
                format!("{changed_error:?}"),
            ),
        ),
        NegativeControlEvidence::new(
            NegativeControlId::StricterAuthorityWithoutEvidence,
            evidence_digest(
                NegativeControlId::StricterAuthorityWithoutEvidence.as_str(),
                format!("{weak_error:?}"),
            ),
        ),
    ]
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn visit_rs_files(path: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<_> = fs::read_dir(path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
        .map(|entry| entry.expect("directory entry must be readable").path())
        .collect();
    entries.sort();
    for entry in entries {
        if entry.is_dir() {
            visit_rs_files(&entry, out);
        } else if entry.extension().and_then(|value| value.to_str()) == Some("rs") {
            out.push(entry);
        }
    }
}

fn sealed_import_control() -> NegativeControlEvidence {
    let root = repository_root();
    for crate_name in ["formula-engine", "formula-packages"] {
        let crate_root = root.join("crates").join(crate_name);
        let manifest = fs::read_to_string(crate_root.join("Cargo.toml")).unwrap();
        assert!(!manifest.contains("formula-first-light"));
        let mut files = Vec::new();
        visit_rs_files(&crate_root.join("src"), &mut files);
        for file in files {
            let text = fs::read_to_string(&file).unwrap();
            assert!(!text.contains("formula_first_light"));
            assert!(!text.contains("tests/first-light/sealed"));
            assert!(!text.contains("formula-p6-fl-a-sealed-target-v1"));
            assert!(!text.contains("formula-p6-fl-c-sealed-u8-target-v1"));
        }
    }
    NegativeControlEvidence::new(
        NegativeControlId::SealedImportAttempt,
        evidence_digest(
            NegativeControlId::SealedImportAttempt.as_str(),
            "engine-and-packages-source-scan-rejected",
        ),
    )
}

fn search_authority_write_control() -> NegativeControlEvidence {
    let root = repository_root();
    let manifest = fs::read_to_string(root.join("crates/formula-engine/Cargo.toml")).unwrap();
    for forbidden in ["formula-store", "formula-check", "formula-first-light"] {
        assert!(!manifest.contains(forbidden));
    }
    let mut files = Vec::new();
    visit_rs_files(&root.join("crates/formula-engine/src"), &mut files);
    for file in files {
        let text = fs::read_to_string(&file).unwrap();
        for forbidden in [
            "formula_store",
            "formula_check",
            "PromotionAuthorization",
            "AuthorityStore",
            "publish_generation",
            ".promote(",
        ] {
            assert!(!text.contains(forbidden));
        }
    }
    assert!(!SideEffectPolicy::deny_all().allows_authority_write());
    NegativeControlEvidence::new(
        NegativeControlId::SearchAuthorityWrite,
        evidence_digest(
            NegativeControlId::SearchAuthorityWrite.as_str(),
            "engine-authority-boundary-and-side-effect-policy-rejected",
        ),
    )
}

pub struct CanonicalProofReport {
    source_commit: String,
    u0_digest: ArtifactDigest,
    u1_digest: ArtifactDigest,
    toolchain_release: String,
    toolchain_host: String,
    manifest_digest: ArtifactDigest,
    negative_controls_digest: ArtifactDigest,
    negative_control_count: usize,
    reuse_candidate_spaces: u64,
    reuse_discovery_work_cells: u64,
    matching_count: u64,
    markers: &'static [&'static str; 15],
}

impl CanonicalProofReport {
    pub fn source_commit(&self) -> &str {
        &self.source_commit
    }
    pub fn u0_digest(&self) -> ArtifactDigest {
        self.u0_digest
    }
    pub fn u1_digest(&self) -> ArtifactDigest {
        self.u1_digest
    }
    pub fn toolchain_release(&self) -> &str {
        &self.toolchain_release
    }
    pub fn toolchain_host(&self) -> &str {
        &self.toolchain_host
    }
    pub fn manifest_digest(&self) -> ArtifactDigest {
        self.manifest_digest
    }
    pub fn negative_controls_digest(&self) -> ArtifactDigest {
        self.negative_controls_digest
    }
    pub fn negative_control_count(&self) -> usize {
        self.negative_control_count
    }
    pub fn reuse_candidate_spaces(&self) -> u64 {
        self.reuse_candidate_spaces
    }
    pub fn reuse_discovery_work_cells(&self) -> u64 {
        self.reuse_discovery_work_cells
    }
    pub fn matching_count(&self) -> u64 {
        self.matching_count
    }
    pub fn markers(&self) -> &'static [&'static str; 15] {
        self.markers
    }
}

pub fn run_canonical_first_light_proof(source_commit: &str) -> CanonicalProofReport {
    assert!(!source_commit.is_empty());

    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![], vec![]);
    let u0_digest = store.initialize_genesis(&u0).unwrap();
    let world = d("p7-fl-c-world");
    let authority_contract = d("p7-fl-c-authority-contract");
    let observer = d("p7-fl-c-observer");

    let (fl_a, nc03) = fl_a_proof(u0_digest, world);
    let (fl_b, nc04) = fl_b_proof(u0_digest, world);

    let fl_c_context = CandidateSpaceContext::new(
        u0_digest,
        world,
        d("p7-fl-c-query"),
        d("p7-fl-c-obligation"),
        fl_c_grammar_digest(),
        d("p7-fl-c-search-policy"),
    );
    let mut fl_c_space =
        ObservationalExprSpace::new(fl_c_context.clone(), U8BoolGrammar::minimal(), 9);
    let final_candidate = (0..=u8::MAX)
        .find_map(|_| {
            let candidate = fl_c_space.extract_min_cost().expect("bounded FL-C candidate");
            match fl_c_oracle().first_counterexample(&candidate) {
                Some((input, expected)) => {
                    fl_c_space.refine_counterexample(input, expected);
                    None
                }
                None => Some(candidate),
            }
        })
        .expect("bounded FL-C discovery converges");
    assert_eq!(fl_c_oracle().first_counterexample(&final_candidate), None);
    let engine_expression = final_candidate.expression().clone();
    let primitive = engine_expression.digest();

    let zero_near_miss = fl_c_zero_near_miss();
    assert_ne!(zero_near_miss.eval(0), final_candidate.eval(0));
    let nc05 = NegativeControlEvidence::new(
        NegativeControlId::FlCZeroNearMiss,
        evidence_digest(
            NegativeControlId::FlCZeroNearMiss.as_str(),
            format!(
                "near={}|accepted={}",
                zero_near_miss.digest().as_str(),
                final_candidate.digest().as_str()
            ),
        ),
    );

    let promotion_evidence = d("p7-fl-c-exhaustive-equivalence-evidence");
    let frozen = CertifiedFrozenCandidate::new(
        "first-light-fl-c-semantic-primitive".into(),
        vec![primitive],
        world,
        u0_digest,
        vec![],
        vec![fl_c_target_digest()],
        authority_contract,
        observer,
    );
    let promotion_manifest = PromotionManifest::new(
        u0_digest,
        frozen.structural_digest(),
        vec![promotion_evidence],
        vec![primitive],
        vec![promotion_evidence],
    );
    let promotion = PromotionCandidate::new(
        frozen.structural_digest(),
        promotion_manifest.structural_digest(),
        u0_digest,
        u0_digest,
        vec![],
        vec![],
    );
    let PromotionDecision::Authorized(promotion_authorization) = authorize_promotion_v1(
        &promotion_manifest,
        &frozen,
        &promotion,
        &[promotion_evidence],
        &u0,
        &[],
    )
    .unwrap()
    else {
        panic!("valid FL-C primitive must authorize")
    };
    let stale_authorization = promotion_authorization.clone();
    let promotion_outcome = store.promote(&promotion_authorization).unwrap();
    let u1_digest = promotion_outcome.new_generation();
    let u1 = store.replay_generation(u1_digest).unwrap();
    assert_eq!(u1.parent(), Some(u0_digest));
    assert!(u1.admitted().contains(&primitive));

    let stale_error = store.promote(&stale_authorization).unwrap_err();
    assert!(matches!(
        stale_error,
        AuthorityStoreError::ParentMismatch {
            expected: Some(expected),
            actual: Some(actual),
        } if expected == u1_digest && actual == u0_digest
    ));
    let nc12 = NegativeControlEvidence::new(
        NegativeControlId::PromotionParentRace,
        evidence_digest(
            NegativeControlId::PromotionParentRace.as_str(),
            format!("{stale_error}"),
        ),
    );

    let packages = validate_activation(&u1, &[], &[], &[]).unwrap();
    let closure_context = ClosureContext::new(
        u1_digest,
        world,
        packages.digests().to_vec(),
        d("p9-second-query-closure-rules"),
        authority_contract,
    );
    let closure_before = derive_capabilities_with_semantic_activations(
        &closure_context,
        &packages,
        &[],
        &[],
        &u1,
        &[],
    )
    .unwrap();
    assert!(!closure_before.contains(primitive));

    let activated = PromotionRecord::new(
        promotion.structural_digest(),
        PromotionState::Activated,
        u1_digest,
        promotion_authorization.policy_digest(),
        vec![promotion_evidence],
        vec![primitive],
    );
    store
        .admit_semantic_activation(&activated, primitive)
        .unwrap();
    let persisted_activation = store
        .resolve_semantic_activation(u1_digest, primitive)
        .unwrap()
        .expect("semantic activation persisted");
    let closure_after = derive_capabilities_with_semantic_activations(
        &closure_context,
        &packages,
        &[],
        &[],
        &u1,
        &[persisted_activation],
    )
    .unwrap();
    assert!(closure_after.contains(primitive));
    let closure_delta = CapabilityClosureDelta::between(&closure_before, &closure_after);
    assert_eq!(closure_delta.added().collect::<Vec<_>>(), vec![primitive]);
    assert!(closure_delta.removed().next().is_none());

    let specialization =
        SpecializationIdentity::new(primitive, u1_digest, world, authority_contract, observer);
    let generated = generate_u8_bool_rust_source(&engine_expression, &specialization).unwrap();
    let (toolchain, toolchain_release, toolchain_host) = canonical_rust_toolchain();
    let build_dir = tempdir().unwrap();
    let source_path = build_dir.path().join("p9_canonical_native.rs");
    let binary_path = build_dir.path().join("p9_canonical_native");
    fs::write(&source_path, generated.source().as_bytes()).unwrap();
    let compile_status = Command::new(toolchain.compiler())
        .arg(&source_path)
        .arg(toolchain.optimization())
        .arg("-o")
        .arg(&binary_path)
        .status()
        .unwrap();
    assert!(compile_status.success());
    let binary_bytes = fs::read(&binary_path).unwrap();
    let binary_digest = ArtifactDigest::of_bytes(&binary_bytes);
    let native_manifest = NativeRealizationManifest::new(
        primitive,
        u1_digest,
        world,
        authority_contract,
        observer,
        specialization.structural_digest(),
        generated.source_digest(),
        toolchain.structural_digest(),
        binary_digest,
    );
    let outputs: Vec<bool> = (0u16..=255)
        .map(|raw| execute_native(&binary_path, raw as u8))
        .collect();
    let semantic = checked_bool(&engine_expression);
    let check_manifest = RealizationCheckManifest::new(
        primitive,
        native_manifest.structural_digest(),
        u1_digest,
        world,
        authority_contract,
        observer,
        binary_digest,
    );
    let realization_authorization = authorize_native_u8_realization_v1(
        &native_manifest,
        &specialization,
        &toolchain,
        &check_manifest,
        generated.source().as_bytes(),
        &binary_bytes,
        &semantic,
        &outputs,
    )
    .unwrap();

    let mut mutated_binary = binary_bytes.clone();
    mutated_binary.push(0);
    let mutated_error = authorize_native_u8_realization_v1(
        &native_manifest,
        &specialization,
        &toolchain,
        &check_manifest,
        generated.source().as_bytes(),
        &mutated_binary,
        &semantic,
        &outputs,
    )
    .unwrap_err();
    assert_eq!(
        mutated_error,
        CheckFailure::RealizationArtifactDigestMismatch
    );
    let nc09 = NegativeControlEvidence::new(
        NegativeControlId::MutatedRealizationBinary,
        evidence_digest(
            NegativeControlId::MutatedRealizationBinary.as_str(),
            format!("{mutated_error:?}"),
        ),
    );

    let admitted = store
        .admit_realization(&realization_authorization, &binary_bytes)
        .unwrap();
    let dispatch =
        RealizationDispatchContext::new(primitive, u1_digest, world, authority_contract, observer);
    let resolved = store
        .resolve_realization(&dispatch)
        .unwrap()
        .expect("admitted P8 realization resolves");
    assert_eq!(resolved.manifest_digest(), admitted.manifest_digest());
    assert_eq!(resolved.binary_digest(), admitted.binary_digest());

    let mut controls = envelope_negative_controls();
    controls.push(sealed_import_control());
    controls.push(search_authority_write_control());
    controls.extend([nc03, nc04, nc05, nc09, nc12]);

    let vector = canonical_second_query_vector();
    let package_binding = ActivatedPackageBinding::new(
        u1_digest,
        packages.digests().to_vec(),
        packages.composition_claims().to_vec(),
    );
    let query = QueryIR::new(
        u1_digest,
        world,
        vec![KnownBinding::new("values", vector.digest())],
        vec![],
        vec![TargetRequest::new(
            d("p9-count-power-of-two-u8"),
            RequestedResultClass::Count,
        )],
        observer,
        authority_contract,
        ResourceContract::new(100, 1024, 50),
        SideEffectPolicy::deny_all(),
        package_binding.clone(),
    );
    let compiler_inputs = CompilerInputs::new(
        observer,
        authority_contract,
        d("p9-second-query-evidence-requirement"),
        d("p9-second-query-random-key"),
    );
    let missing_snapshot = CompilerAuthoritySnapshot::new(
        u1_digest,
        world,
        package_binding.clone(),
        vec![vector.digest()],
        closure_before.capabilities().collect(),
        vec![],
    );
    let request = ReuseRequest::new(&query, primitive);
    let activation_removed_error = CompilerV1::compile_reuse(
        &query,
        &missing_snapshot,
        compiler_inputs.clone(),
        &request,
    )
    .unwrap_err();
    assert_eq!(
        activation_removed_error,
        CompilerError::RequiredCapabilityUnavailable
    );
    controls.push(NegativeControlEvidence::new(
        NegativeControlId::ActivationRemoved,
        evidence_digest(
            NegativeControlId::ActivationRemoved.as_str(),
            format!("{activation_removed_error:?}"),
        ),
    ));

    let snapshot = CompilerAuthoritySnapshot::new(
        u1_digest,
        world,
        package_binding,
        vec![vector.digest(), primitive],
        closure_after.capabilities().collect(),
        vec![],
    );
    let compiled = CompilerV1::compile_reuse(&query, &snapshot, compiler_inputs, &request).unwrap();
    assert!(compiled.work_cells().is_empty());
    assert_eq!(compiled.metrics().primitive_discovery_candidate_spaces(), 0);
    assert_eq!(compiled.metrics().primitive_discovery_work_cells(), 0);
    assert_eq!(compiled.metrics().resolved_capability_count(), 1);
    assert_eq!(compiled.metrics().execution_work_items(), 1);

    let execution_dir = tempdir().unwrap();
    let admitted_binary_path = execution_dir.path().join("p9_admitted_native");
    fs::write(&admitted_binary_path, resolved.binary_bytes()).unwrap();
    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(&admitted_binary_path).unwrap().permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&admitted_binary_path, permissions).unwrap();
    }
    let native_count = vector
        .values()
        .iter()
        .copied()
        .filter(|value| execute_native(&admitted_binary_path, *value))
        .count() as u64;
    let semantic_count = vector
        .values()
        .iter()
        .copied()
        .filter(|value| engine_expression.eval(*value))
        .count() as u64;
    assert_eq!(native_count, semantic_count);
    assert_eq!(native_count, 9);
    let second_result = SecondQueryResult::new(
        &vector,
        primitive,
        resolved.manifest_digest(),
        native_count,
    );

    let negative_controls = NegativeControlManifest::complete(controls).unwrap();
    assert!(negative_controls.is_complete());
    assert_eq!(negative_controls.controls().len(), 12);

    let fl_c = FirstLightTargetEvidence::new(
        fl_c_context.query_digest(),
        fl_c_space.freeze().digest(),
        final_candidate.digest(),
        promotion_evidence,
        vec![fl_c_target_digest(), fl_c_grammar_digest()],
    );
    let native = FirstLightNativeEvidence::new(
        generated.source_digest(),
        toolchain.structural_digest(),
        binary_digest,
        resolved.manifest_digest(),
    );
    let reuse = FirstLightReuseEvidence::new(
        query.digest(),
        compiled.campaign().digest(),
        compiled.resolved_capability().digest(),
        compiled.execution_plans()[0].digest(),
        second_result.digest(),
        compiled.metrics().digest(),
        resolved.manifest_digest(),
    );
    let closure_before_digest = closure_digest(&closure_before);
    let closure_after_digest = closure_digest(&closure_after);
    let closure_delta_digest = closure_delta_digest(&closure_delta);
    let activated_package_set = package_set_digest(u1_digest, packages.digests());
    let promotion_digest = promotion_outcome.admitted_record().structural_digest();

    let manifest = FirstLightProofManifest::new(
        source_commit.to_owned(),
        u0_digest,
        u1_digest,
        world,
        activated_package_set,
        fl_a.clone(),
        fl_b.clone(),
        fl_c.clone(),
        promotion_digest,
        closure_before_digest,
        closure_after_digest,
        closure_delta_digest,
        native.clone(),
        reuse.clone(),
        negative_controls.clone(),
        verifier_identity_v1(),
        checker_identity_v1(),
    );
    let replay = FirstLightReplayEvidence {
        source_commit: source_commit.to_owned(),
        u0_digest,
        u1_digest,
        u1_parent: u0_digest,
        world,
        activated_package_set,
        fl_a,
        fl_b,
        fl_c,
        promotion_digest,
        closure_before: closure_before_digest,
        closure_after: closure_after_digest,
        closure_delta: closure_delta_digest,
        native,
        reuse,
        reuse_candidate_spaces: compiled.metrics().primitive_discovery_candidate_spaces(),
        reuse_discovery_work_cells: compiled.metrics().primitive_discovery_work_cells(),
        reuse_result_exact: native_count == semantic_count,
        negative_controls: negative_controls.clone(),
        verifier_identity: verifier_identity_v1(),
        checker_identity: checker_identity_v1(),
    };
    let verification = verify_first_light_manifest_v1(&manifest, &replay).unwrap();

    CanonicalProofReport {
        source_commit: source_commit.to_owned(),
        u0_digest,
        u1_digest,
        toolchain_release,
        toolchain_host,
        manifest_digest: manifest.structural_digest(),
        negative_controls_digest: negative_controls.structural_digest(),
        negative_control_count: negative_controls.controls().len(),
        reuse_candidate_spaces: compiled.metrics().primitive_discovery_candidate_spaces(),
        reuse_discovery_work_cells: compiled.metrics().primitive_discovery_work_cells(),
        matching_count: native_count,
        markers: verification.markers(),
    }
}
