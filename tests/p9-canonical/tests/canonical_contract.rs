use formula_p9_canonical::run_canonical_first_light_proof;

#[test]
fn canonical_p9_runner_closes_first_light_from_one_clean_state() {
    let source =
        std::env::var("FORMULA_P9_SOURCE_COMMIT").unwrap_or_else(|_| "development-source".into());
    let report = run_canonical_first_light_proof(&source);

    assert_eq!(report.source_commit(), source);
    assert_ne!(report.u0_digest(), report.u1_digest());
    assert_eq!(report.negative_control_count(), 12);
    assert_eq!(report.reuse_candidate_spaces(), 0);
    assert_eq!(report.reuse_discovery_work_cells(), 0);
    assert_eq!(report.matching_count(), 9);
    assert_eq!(report.toolchain_release(), "1.98.0");
    assert_eq!(report.markers().len(), 15);
    assert_eq!(
        report.markers(),
        &[
            "PASS D1_AUTHORITY_SEPARATION",
            "PASS D2_IDENTITY_GENERATION_REPLAY",
            "PASS D2_CERTIFICATE_ROUTING",
            "PASS D2_SEARCH_STATE_SEPARATION",
            "PASS D3_BLIND_SEMANTIC_ELABORATION",
            "PASS D3_REPRESENTATION_REDUCTION",
            "PASS D3_SYMBOLIC_CANDIDATE_SPACE",
            "PASS D3_FALSE_NEARMISS_REJECTION",
            "PASS D4_NATIVE_REALIZATION_EQUIVALENCE",
            "PASS D4_CPU_LOCAL_OFFLINE",
            "PASS D5_ATOMIC_PROMOTION",
            "PASS D5_CAPABILITY_CLOSURE_EXPANDED",
            "PASS D5_SECOND_QUERY_REUSE",
            "PASS NEGATIVE_CONTROLS",
            "PASS FIRST_LIGHT_COMPLETE",
        ]
    );

    println!("P9_SOURCE_SHA={}", report.source_commit());
    println!("P9_TOOLCHAIN_RELEASE={}", report.toolchain_release());
    println!("P9_TOOLCHAIN_HOST={}", report.toolchain_host());
    println!("P9_MANIFEST={}", report.manifest_digest().as_str());
    println!("P9_U0={}", report.u0_digest().as_str());
    println!("P9_U1={}", report.u1_digest().as_str());
    println!(
        "P9_NEGATIVE_CONTROLS={}",
        report.negative_controls_digest().as_str()
    );
    println!("P9_COUNT={}", report.matching_count());
    for marker in report.markers() {
        println!("{marker}");
    }
}
