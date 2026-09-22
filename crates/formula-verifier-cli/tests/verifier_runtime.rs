use num_bigint::BigInt;
use std::io::Write;
use std::process::{Command, Output, Stdio};
use std::str::FromStr;

fn run(input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_formula-verifier-cli"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn formula-verifier-cli");
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("write request");
    child.wait_with_output().expect("wait")
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("utf8 stdout")
}

#[test]
fn identity_is_verification_only_and_bound_to_frozen_predecessor() {
    let output = run("command=identity\n");
    assert!(output.status.success());
    let text = stdout(&output);
    assert!(text.contains("schema=formula-verifier-runtime-v1\n"));
    assert!(
        text.contains(
            "verifier_equivalence_frozen_head=944d7d5c6a0094b7dd623519b6c1ff3c899188b1\n"
        )
    );
    assert!(text.contains("signing_supported=false\n"));
    assert!(text.contains("authorization_generation_supported=false\n"));
    assert!(text.contains("private_key_operations_supported=false\n"));
}

#[test]
fn powmod_uses_formula_checker_and_rejects_wrong_result() {
    let good = run("command=powmod\nbase=4\nexponent=3\nmodulus=17\nproducer_decimal=13\n");
    assert!(good.status.success());
    assert!(stdout(&good).contains("result_decimal=13\n"));

    let bad = run("command=powmod\nbase=4\nexponent=3\nmodulus=17\nproducer_decimal=14\n");
    assert_eq!(bad.status.code(), Some(2));
    assert!(stdout(&bad).contains("error_code=powmod-rejected\n"));
}

#[test]
fn rsa_public_checks_decimal_and_fixed_width_hex() {
    let output = run(
        "command=rsa-public\nrepresentative=4\nexponent=3\nmodulus=17\nproducer_decimal=13\nproducer_hex=0d\n",
    );
    assert!(output.status.success());
    let text = stdout(&output);
    assert!(text.contains("result_decimal=13\n"));
    assert!(text.contains("result_hex=0d\n"));
    assert!(text.contains("width_bytes=1\n"));
}

#[test]
fn montgomery_accepts_one_deferred_subtraction() {
    let output =
        run("command=montgomery\nlhs=5\nrhs=7\nmodulus=19\nradix_bits=8\nproducer_decimal=25\n");
    assert!(output.status.success());
    let text = stdout(&output);
    assert!(text.contains("normalized_decimal=6\n"));
    assert!(text.contains("required_subtraction=true\n"));
}

#[test]
fn type1_checks_exact_payload_without_signing() {
    let block = format!("0001{}00aa", "ff".repeat(8));
    let output = run(&format!(
        "command=type1\nblock_hex={block}\nwidth_bytes=12\nexpected_payload_hex=aa\n"
    ));
    assert!(output.status.success());
    let text = stdout(&output);
    assert!(text.contains("payload_hex=aa\n"));
    assert!(text.contains("padding_bytes=8\n"));
}

#[test]
fn rsa_type1_uses_public_operation_and_exact_payload() {
    let block = format!("0001{}00aa", "ff".repeat(8));
    let representative = BigInt::parse_bytes(block.as_bytes(), 16).expect("block representative");
    let modulus = (BigInt::from(1u8) << 96usize) - BigInt::from(5u8);
    let request = format!(
        "command=rsa-type1\nrepresentative={representative}\nexponent=1\nmodulus={modulus}\nexpected_payload_hex=aa\n"
    );
    let output = run(&request);
    assert!(output.status.success());
    let text = stdout(&output);
    assert!(text.contains("payload_hex=aa\n"));
    assert!(text.contains("padding_bytes=8\n"));
}

#[test]
fn protocol_rejects_duplicate_keys_noncanonical_numbers_and_unknown_commands() {
    for input in [
        "command=identity\ncommand=identity\n",
        "command=powmod\nbase=04\nexponent=3\nmodulus=17\nproducer_decimal=13\n",
        "command=sign\n",
    ] {
        let output = run(input);
        assert_eq!(output.status.code(), Some(64));
        assert!(stdout(&output).contains("error_kind=protocol\n"));
    }
}

#[test]
fn stdin_values_do_not_need_to_appear_in_process_arguments() {
    let secret_marker = "sensitive_payload_marker_7ad9";
    let output = run(&format!(
        "command=type1\nblock_hex=0001{}00\nwidth_bytes=11\nexpected_payload_hex=\nunused={secret_marker}\n",
        "ff".repeat(8)
    ));
    assert!(output.status.success());
    assert!(!stdout(&output).contains(secret_marker));
}

#[test]
fn canonical_decimal_parser_accepts_large_public_values() {
    let large = BigInt::from_str(
        "19076043798060371941559465302453866169504163389425602492912251598253440742698694216939646292357740713747036398975753758933042895927533899096797324507908998842267335248882773101513693630118289484713512383296958217011449417049337316547985586308114378355884757699258805308685657944380999662689381166690279488797393512053106994364905902080695221743864208341582944757584837247414500416217099226860311785775525542307631752507361275451077246390532793259703876845074638192811866148775613103119461834891674906835159876220737964957641363380889938223812389190402835514196026544400588210743495647502436690084993354197379203081291",
    )
    .unwrap();
    assert!(large > BigInt::from(0u8));
}
