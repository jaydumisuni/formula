use formula_core::digest::ArtifactDigest;

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
    assert!(ArtifactDigest::parse(
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    )
    .is_err());
}
