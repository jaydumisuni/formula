use formula_check::verifier_equivalence::{
    MontgomeryProductOperation, RsaType1VerifierOperation, Type1PayloadOperation,
    VerifierEquivalenceError, check_montgomery_product_result, check_rsa_type1_payload,
    check_type1_payload,
};
use num_bigint::BigInt;
use std::str::FromStr;

fn n(value: &str) -> BigInt {
    BigInt::from_str(value).expect("valid bigint")
}

fn type1_block(width: usize, payload: &[u8]) -> String {
    assert!(payload.len() <= width - 11);
    let padding = width - payload.len() - 3;
    let mut bytes = Vec::with_capacity(width);
    bytes.extend_from_slice(&[0, 1]);
    bytes.extend(std::iter::repeat_n(0xff, padding));
    bytes.push(0);
    bytes.extend_from_slice(payload);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn montgomery_product_checks_exact_relation() {
    let operation = MontgomeryProductOperation::new(n("5"), n("7"), n("19"), 8);
    let receipt = check_montgomery_product_result(&operation, "6").expect("montgomery");
    assert_eq!(receipt.normalized_decimal(), "6");
    assert!(!receipt.required_subtraction());
}

#[test]
fn montgomery_product_accepts_one_deferred_subtraction() {
    let operation = MontgomeryProductOperation::new(n("5"), n("7"), n("19"), 8);
    let receipt = check_montgomery_product_result(&operation, "25").expect("single subtract");
    assert_eq!(receipt.normalized_decimal(), "6");
    assert!(receipt.required_subtraction());
}

#[test]
fn montgomery_domain_and_result_bounds_fail_closed() {
    for modulus in ["2", "20"] {
        let operation = MontgomeryProductOperation::new(n("1"), n("1"), n(modulus), 8);
        assert_eq!(
            check_montgomery_product_result(&operation, "1"),
            Err(VerifierEquivalenceError::InvalidModulus)
        );
    }

    let small_radix = MontgomeryProductOperation::new(n("1"), n("1"), n("19"), 4);
    assert_eq!(
        check_montgomery_product_result(&small_radix, "1"),
        Err(VerifierEquivalenceError::InvalidRadix)
    );

    let out_of_range = MontgomeryProductOperation::new(n("19"), n("1"), n("19"), 8);
    assert_eq!(
        check_montgomery_product_result(&out_of_range, "0"),
        Err(VerifierEquivalenceError::OperandOutOfRange)
    );

    let operation = MontgomeryProductOperation::new(n("5"), n("7"), n("19"), 8);
    assert_eq!(
        check_montgomery_product_result(&operation, "38"),
        Err(VerifierEquivalenceError::ProducerOutOfRange)
    );
}

#[test]
fn type1_accepts_empty_and_nonempty_payloads() {
    for payload in [Vec::new(), vec![0xaa], b"token".to_vec()] {
        let width = 11 + payload.len();
        let operation = Type1PayloadOperation::new(
            type1_block(width, &payload),
            width,
            payload
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>(),
        );
        let receipt = check_type1_payload(&operation).expect("type1");
        assert_eq!(receipt.padding_bytes(), 8);
    }
}

#[test]
fn type1_width_256_accepts_48_and_245_byte_payloads() {
    for payload_len in [48usize, 245usize] {
        let payload = (0..payload_len)
            .map(|i| ((i % 251) + 1) as u8)
            .collect::<Vec<_>>();
        let payload_hex = payload
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        let operation = Type1PayloadOperation::new(type1_block(256, &payload), 256, payload_hex);
        let receipt = check_type1_payload(&operation).expect("donor boundary");
        assert_eq!(receipt.padding_bytes(), 256 - payload_len - 3);
    }
}

#[test]
fn type1_rejects_246_byte_payload_at_width_256() {
    let payload = vec![0x42; 246];
    let operation = Type1PayloadOperation::new(
        "00".repeat(256),
        256,
        payload
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>(),
    );
    assert_eq!(
        check_type1_payload(&operation),
        Err(VerifierEquivalenceError::InvalidType1Block)
    );
}

#[test]
fn type1_rejects_prefix_padding_separator_payload_and_hex_mutations() {
    let payload = b"abc";
    let good = type1_block(16, payload);
    let payload_hex = "616263";

    let mut cases = Vec::new();

    let mut bad_prefix = good.clone().into_bytes();
    bad_prefix[2] = b'0';
    bad_prefix[3] = b'2';
    cases.push(String::from_utf8(bad_prefix).unwrap());

    let mut bad_padding = good.clone().into_bytes();
    bad_padding[4] = b'f';
    bad_padding[5] = b'e';
    cases.push(String::from_utf8(bad_padding).unwrap());

    let separator_byte = 16 - payload.len() - 1;
    let mut bad_separator = good.clone().into_bytes();
    bad_separator[separator_byte * 2] = b'0';
    bad_separator[separator_byte * 2 + 1] = b'1';
    cases.push(String::from_utf8(bad_separator).unwrap());

    for block in cases {
        let operation = Type1PayloadOperation::new(block, 16, payload_hex);
        assert_eq!(
            check_type1_payload(&operation),
            Err(VerifierEquivalenceError::InvalidType1Block)
        );
    }

    let wrong_payload = Type1PayloadOperation::new(good.clone(), 16, "616264");
    assert_eq!(
        check_type1_payload(&wrong_payload),
        Err(VerifierEquivalenceError::PayloadMismatch)
    );

    let uppercase = Type1PayloadOperation::new(good.to_uppercase(), 16, payload_hex);
    assert_eq!(
        check_type1_payload(&uppercase),
        Err(VerifierEquivalenceError::MalformedHex)
    );
}

#[test]
fn integrated_rsa_type1_verifier_checks_exact_payload() {
    let payload = [0xaa];
    let block_hex = type1_block(12, &payload);
    let representative = BigInt::parse_bytes(block_hex.as_bytes(), 16).unwrap();
    let modulus = (BigInt::from(1u8) << 96usize) - BigInt::from(5u8);

    let operation = RsaType1VerifierOperation::new(representative, n("1"), modulus, "aa");
    let receipt = check_rsa_type1_payload(&operation).expect("integrated verifier");
    assert_eq!(receipt.payload_hex(), "aa");
    assert_eq!(receipt.padding_bytes(), 8);
}

#[test]
fn integrated_rsa_type1_verifier_rejects_wrong_payload() {
    let payload = [0xaa];
    let block_hex = type1_block(12, &payload);
    let representative = BigInt::parse_bytes(block_hex.as_bytes(), 16).unwrap();
    let modulus = (BigInt::from(1u8) << 96usize) - BigInt::from(5u8);

    let operation = RsaType1VerifierOperation::new(representative, n("1"), modulus, "ab");
    assert_eq!(
        check_rsa_type1_payload(&operation),
        Err(VerifierEquivalenceError::PayloadMismatch)
    );
}
