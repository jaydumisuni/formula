use formula_core::canonical::{CanonicalRational, CanonicalValue};
use formula_core::digest::ArtifactDigest;
use num_bigint::BigInt;
use std::collections::BTreeMap;

#[test]
fn digest_of_bytes_is_sha256_and_round_trips() {
    let digest = ArtifactDigest::of_bytes(b"abc");
    assert_eq!(
        digest.as_str(),
        "sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(ArtifactDigest::parse(&digest.as_str()).unwrap(), digest);
}

#[test]
fn digest_parser_rejects_noncanonical_forms() {
    assert!(ArtifactDigest::parse("md5:ba7816bf").is_err());
    assert!(ArtifactDigest::parse("sha256:ABCDEF").is_err());
    assert!(ArtifactDigest::parse("sha256:00").is_err());
    assert!(
        ArtifactDigest::parse("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
            .is_err()
    );
}

#[test]
fn canonical_object_identity_is_independent_of_insertion_order() {
    let a = CanonicalValue::Object(BTreeMap::from([
        ("z".into(), CanonicalValue::Integer(2.into())),
        ("a".into(), CanonicalValue::String("x".into())),
    ]));
    let b = CanonicalValue::Object(BTreeMap::from([
        ("a".into(), CanonicalValue::String("x".into())),
        ("z".into(), CanonicalValue::Integer(2.into())),
    ]));
    assert_eq!(a.to_canonical_bytes(), br#"{"a":"x","z":2}"#);
    assert_eq!(a.to_canonical_bytes(), b.to_canonical_bytes());
    assert_eq!(a.digest(), b.digest());
}

#[test]
fn canonical_rational_normalizes_sign_gcd_and_zero() {
    let q = CanonicalRational::new(BigInt::from(-6), BigInt::from(-8)).unwrap();
    assert_eq!(q.numerator(), &BigInt::from(3));
    assert_eq!(q.denominator(), &BigInt::from(4));

    let zero = CanonicalRational::new(BigInt::from(0), BigInt::from(-99)).unwrap();
    assert_eq!(zero.numerator(), &BigInt::from(0));
    assert_eq!(zero.denominator(), &BigInt::from(1));
}

#[test]
fn canonical_rational_rejects_zero_denominator() {
    assert!(CanonicalRational::new(BigInt::from(1), BigInt::from(0)).is_err());
}

#[test]
fn canonical_string_escaping_is_deterministic_utf8_json() {
    let value = CanonicalValue::String("a\n\"β".to_owned());
    assert_eq!(value.to_canonical_bytes(), "\"a\\n\\\"β\"".as_bytes());
}
