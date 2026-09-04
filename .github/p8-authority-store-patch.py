from pathlib import Path

path = Path("crates/formula-store/src/authority_store.rs")
text = path.read_text()

old = "use crate::blob_store::{BlobStore, BlobStoreError};\n"
new = "mod realization_store;\n\npub use realization_store::AdmittedRealization;\n\nuse crate::blob_store::{BlobStore, BlobStoreError};\n"
assert text.count(old) == 1
text = text.replace(old, new, 1)

old = "    ManifestBlobBytesMismatch(ArtifactDigest),\n    InjectedPublishFailure(&'static str),\n"
new = "    ManifestBlobBytesMismatch(ArtifactDigest),\n    RealizationGenerationMismatch {\n        expected: ArtifactDigest,\n        actual: ArtifactDigest,\n    },\n    RealizationBinaryDigestMismatch {\n        expected: ArtifactDigest,\n        actual: ArtifactDigest,\n    },\n    InjectedPublishFailure(&'static str),\n"
assert text.count(old) == 1
text = text.replace(old, new, 1)

old = '''            Self::ManifestBlobBytesMismatch(digest) => write!(
                f,
                "generation manifest blob bytes do not match canonical replay for {}",
                digest.as_str()
            ),
            Self::InjectedPublishFailure(point) => {
'''
new = '''            Self::ManifestBlobBytesMismatch(digest) => write!(
                f,
                "generation manifest blob bytes do not match canonical replay for {}",
                digest.as_str()
            ),
            Self::RealizationGenerationMismatch { expected, actual } => write!(
                f,
                "realization generation mismatch: expected {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
            Self::RealizationBinaryDigestMismatch { expected, actual } => write!(
                f,
                "realization binary digest mismatch: expected {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
            Self::InjectedPublishFailure(point) => {
'''
assert text.count(old) == 1
text = text.replace(old, new, 1)

path.write_text(text)
