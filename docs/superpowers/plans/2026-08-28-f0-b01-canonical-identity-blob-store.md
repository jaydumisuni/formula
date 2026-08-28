# F0 B01 Canonical Identity + Blob Store Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement D3 stage B01: deterministic canonical authority bytes, SHA-256 artifact identity, and an immutable content-addressed blob store suitable for later D1 artifact manifests.

**Architecture:** Create only the minimum Rust workspace and `formula-core` crate needed for B01. Canonicalization is an explicit typed encoder rather than generic `serde_json::Value`, so floats and unordered maps cannot enter authority identity accidentally. The blob store accepts exact bytes, derives the SHA-256 digest, stores immutable blobs at the D3 path layout, verifies integrity on every read, and never overwrites a different blob at an existing digest path.

**Tech Stack:** Rust 1.98.0, Cargo, `sha2`, `num-bigint`, `num-integer`, `num-traits`, `tempfile` (dev only).

**Spec:** `docs/design/2026-08-28-d3-first-light-build-architecture.md` sections 2.1, 2.4, 4, 14.1, 15 (B01), plus D1 structural-identity rules in `docs/design/2026-08-28-d1-mathematical-constitution.md`.

## Global Constraints

- Pin Rust exactly to `1.98.0` in `rust-toolchain.toml`.
- Authority digests are SHA-256.
- Canonical authority encoding is UTF-8 deterministic JSON under a restricted schema.
- No IEEE floating-point value may participate in B01 authority identity.
- Integers are arbitrary precision and encoded canonically.
- Rationals are reduced numerator / positive denominator pairs.
- Object fields are serialized in deterministic lexicographic key order regardless of construction order.
- No timestamps, random values, process IDs, machine-local absolute paths, or filesystem metadata participate in structural identity.
- Blob layout is `.formula/objects/sha256/<first-byte-hex>/<remaining-62-hex>`.
- Blob writes are immutable and idempotent; reads verify the stored bytes against the requested digest.
- No D1 schema, discovery, campaign, promotion, or solver behavior is implemented in B01.
- TDD is mandatory: each production behavior is preceded by a test observed failing for the intended reason.

---

## File Structure

Create:

```text
Cargo.toml
rust-toolchain.toml
crates/formula-core/Cargo.toml
crates/formula-core/src/lib.rs
crates/formula-core/src/digest.rs
crates/formula-core/src/canonical.rs
crates/formula-core/src/blob_store.rs
crates/formula-core/tests/canonical_identity.rs
crates/formula-core/tests/blob_store.rs
```

Responsibilities:

- `Cargo.toml` — First-Light workspace root; only `formula-core` is a member at B01.
- `rust-toolchain.toml` — exact Rust 1.98.0 pin.
- `digest.rs` — `ArtifactDigest`, SHA-256 derivation, strict `sha256:<hex>` parse/display.
- `canonical.rs` — restricted canonical authority-value model and deterministic UTF-8 JSON encoder.
- `blob_store.rs` — immutable SHA-256 content-addressed store with verified reads.
- `lib.rs` — public B01 surface only.
- integration tests — authority behavior from the external crate API.

---

### Task 1: Bootstrap the pinned Rust workspace and digest identity

**Files:**
- Create: `Cargo.toml`
- Create: `rust-toolchain.toml`
- Create: `crates/formula-core/Cargo.toml`
- Create: `crates/formula-core/src/lib.rs`
- Create: `crates/formula-core/src/digest.rs`
- Test: `crates/formula-core/tests/canonical_identity.rs`

**Interfaces:**
- Produces: `ArtifactDigest::of_bytes(bytes: &[u8]) -> ArtifactDigest`
- Produces: `ArtifactDigest::parse(s: &str) -> Result<ArtifactDigest, DigestError>`
- Produces: `ArtifactDigest::as_str() -> String`
- Produces: `ArtifactDigest::hex() -> String`

- [ ] **Step 1: Create only workspace/toolchain manifests, then write the failing digest tests**

`Cargo.toml`:

```toml
[workspace]
members = ["crates/formula-core"]
resolver = "2"

[workspace.package]
edition = "2024"
license = "MIT"

[workspace.dependencies]
sha2 = "0.10"
num-bigint = "0.4"
num-integer = "0.1"
num-traits = "0.2"
tempfile = "3"
```

`rust-toolchain.toml`:

```toml
[toolchain]
channel = "1.98.0"
profile = "minimal"
components = ["rustfmt", "clippy"]
```

`crates/formula-core/Cargo.toml`:

```toml
[package]
name = "formula-core"
version = "0.0.1"
edition.workspace = true
license.workspace = true
publish = false

[dependencies]
sha2.workspace = true
num-bigint.workspace = true
num-integer.workspace = true
num-traits.workspace = true

[dev-dependencies]
tempfile.workspace = true
```

`crates/formula-core/src/lib.rs` initially contains only:

```rust
pub mod digest;
```

Create `crates/formula-core/tests/canonical_identity.rs`:

```rust
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
    assert!(ArtifactDigest::parse("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad").is_err());
}
```

- [ ] **Step 2: Run the digest test and verify RED**

Run:

```bash
cargo test -p formula-core --test canonical_identity digest_
```

Expected: compilation failure because `formula_core::digest::ArtifactDigest` does not yet exist.

- [ ] **Step 3: Implement the minimal digest type**

`crates/formula-core/src/digest.rs`:

```rust
use sha2::{Digest, Sha256};
use std::{error::Error, fmt};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ArtifactDigest([u8; 32]);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DigestError {
    InvalidPrefix,
    InvalidLength,
    InvalidHex,
    NonCanonicalHex,
}

impl fmt::Display for DigestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for DigestError {}

impl ArtifactDigest {
    pub fn of_bytes(bytes: &[u8]) -> Self {
        let hash = Sha256::digest(bytes);
        let mut out = [0u8; 32];
        out.copy_from_slice(&hash);
        Self(out)
    }

    pub fn parse(value: &str) -> Result<Self, DigestError> {
        let hex = value
            .strip_prefix("sha256:")
            .ok_or(DigestError::InvalidPrefix)?;
        if hex.len() != 64 {
            return Err(DigestError::InvalidLength);
        }
        if hex.bytes().any(|b| matches!(b, b'A'..=b'F')) {
            return Err(DigestError::NonCanonicalHex);
        }
        let mut bytes = [0u8; 32];
        for (i, chunk) in hex.as_bytes().chunks_exact(2).enumerate() {
            let high = decode_nibble(chunk[0])?;
            let low = decode_nibble(chunk[1])?;
            bytes[i] = (high << 4) | low;
        }
        Ok(Self(bytes))
    }

    pub fn hex(&self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut out = String::with_capacity(64);
        for byte in self.0 {
            out.push(HEX[(byte >> 4) as usize] as char);
            out.push(HEX[(byte & 0x0f) as usize] as char);
        }
        out
    }

    pub fn as_str(&self) -> String {
        format!("sha256:{}", self.hex())
    }
}

fn decode_nibble(byte: u8) -> Result<u8, DigestError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(DigestError::InvalidHex),
    }
}
```

- [ ] **Step 4: Run digest tests and verify GREEN**

```bash
cargo test -p formula-core --test canonical_identity digest_
```

Expected: 2 passed, 0 failed.

- [ ] **Step 5: Run formatter/clippy for the new surface**

```bash
cargo fmt --check
cargo clippy -p formula-core --all-targets -- -D warnings
```

Expected: both succeed with no warnings.

- [ ] **Step 6: Commit Task 1**

```bash
git add Cargo.toml rust-toolchain.toml crates/formula-core
git commit -m "feat(core): add canonical artifact digest"
```

---

### Task 2: Canonical restricted JSON authority encoding

**Files:**
- Modify: `crates/formula-core/src/lib.rs`
- Create: `crates/formula-core/src/canonical.rs`
- Modify: `crates/formula-core/tests/canonical_identity.rs`

**Interfaces:**
- Consumes: `ArtifactDigest`
- Produces: `CanonicalValue`
- Produces: `CanonicalRational::new(BigInt, BigInt) -> Result<CanonicalRational, CanonicalError>`
- Produces: `CanonicalValue::to_canonical_bytes() -> Vec<u8>`
- Produces: `CanonicalValue::digest() -> ArtifactDigest`

- [ ] **Step 1: Write failing canonical-order and rational-normalization tests**

Append to `canonical_identity.rs`:

```rust
use formula_core::canonical::{CanonicalRational, CanonicalValue};
use num_bigint::BigInt;
use std::collections::BTreeMap;

#[test]
fn canonical_object_identity_is_independent_of_insertion_order() {
    let mut a = BTreeMap::new();
    a.insert("z".to_owned(), CanonicalValue::Integer(2.into()));
    a.insert("a".to_owned(), CanonicalValue::String("x".into()));

    let mut b = BTreeMap::new();
    b.insert("a".to_owned(), CanonicalValue::String("x".into()));
    b.insert("z".to_owned(), CanonicalValue::Integer(2.into()));

    let a = CanonicalValue::Object(a);
    let b = CanonicalValue::Object(b);

    assert_eq!(a.to_canonical_bytes(), br#"{"a":"x","z":2}"#);
    assert_eq!(a.to_canonical_bytes(), b.to_canonical_bytes());
    assert_eq!(a.digest(), b.digest());
}

#[test]
fn canonical_rational_reduces_sign_and_gcd() {
    let rational = CanonicalRational::new(BigInt::from(-6), BigInt::from(-8)).unwrap();
    assert_eq!(rational.numerator(), &BigInt::from(3));
    assert_eq!(rational.denominator(), &BigInt::from(4));
    assert_eq!(
        CanonicalValue::Rational(rational).to_canonical_bytes(),
        br#"{"denominator":4,"numerator":3}"#
    );
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
```

- [ ] **Step 2: Run tests and verify RED**

```bash
cargo test -p formula-core --test canonical_identity canonical_
```

Expected: compilation failure because `formula_core::canonical` does not exist.

- [ ] **Step 3: Implement the restricted canonical value model and encoder**

Create `canonical.rs` with exactly these value families:

```rust
pub enum CanonicalValue {
    Null,
    Bool(bool),
    Integer(BigInt),
    Rational(CanonicalRational),
    String(String),
    Array(Vec<CanonicalValue>),
    Object(BTreeMap<String, CanonicalValue>),
    Digest(ArtifactDigest),
}
```

`CanonicalRational::new` must:

```text
reject denominator == 0
move sign to numerator
reduce by gcd
canonicalize 0/x to 0/1
```

The encoder must:

```text
Null -> null
Bool -> true/false
Integer -> base-10 with no leading zeros
Rational -> {"denominator":D,"numerator":N}
Digest -> JSON string containing canonical sha256:<hex>
Array -> preserve element order
Object -> BTreeMap lexical key order
String -> JSON escaping for quote, backslash, U+0000..U+001F; emit other Unicode as UTF-8
```

Use a private `write_json_string` function; do not add `serde_json` to authority encoding.

Add to `lib.rs`:

```rust
pub mod canonical;
pub mod digest;
```

- [ ] **Step 4: Run canonical tests and verify GREEN**

```bash
cargo test -p formula-core --test canonical_identity canonical_
```

Expected: 4 canonical tests pass.

- [ ] **Step 5: Add a failing dependency-digest identity test**

```rust
#[test]
fn changing_a_referenced_digest_changes_structural_identity() {
    let d1 = ArtifactDigest::of_bytes(b"dependency-one");
    let d2 = ArtifactDigest::of_bytes(b"dependency-two");

    let make = |digest| {
        CanonicalValue::Object(BTreeMap::from([
            ("dependency".into(), CanonicalValue::Digest(digest)),
            ("kind".into(), CanonicalValue::String("example".into())),
        ]))
    };

    assert_ne!(make(d1).digest(), make(d2).digest());
}
```

- [ ] **Step 6: Run it and verify RED for the intended missing/incorrect `Digest` support, then make it GREEN**

```bash
cargo test -p formula-core --test canonical_identity changing_a_referenced_digest
```

Expected before final support: fail because `CanonicalValue::Digest` is absent or not encoded as specified. After implementation: pass.

- [ ] **Step 7: Run all identity tests + lint**

```bash
cargo test -p formula-core --test canonical_identity
cargo fmt --check
cargo clippy -p formula-core --all-targets -- -D warnings
```

Expected: all green.

- [ ] **Step 8: Commit Task 2**

```bash
git add crates/formula-core/src crates/formula-core/tests/canonical_identity.rs
git commit -m "feat(core): add restricted canonical authority encoding"
```

---

### Task 3: Immutable verified content-addressed blob store

**Files:**
- Modify: `crates/formula-core/src/lib.rs`
- Create: `crates/formula-core/src/blob_store.rs`
- Create: `crates/formula-core/tests/blob_store.rs`

**Interfaces:**
- Consumes: `ArtifactDigest`
- Produces: `BlobStore::new(root: impl AsRef<Path>) -> BlobStore`
- Produces: `BlobStore::put(bytes: &[u8]) -> Result<ArtifactDigest, BlobStoreError>`
- Produces: `BlobStore::get(digest: ArtifactDigest) -> Result<Vec<u8>, BlobStoreError>`
- Produces: `BlobStore::contains(digest: ArtifactDigest) -> Result<bool, BlobStoreError>`
- Produces: `BlobStore::path_for(digest: ArtifactDigest) -> PathBuf`

- [ ] **Step 1: Write failing path-layout and idempotent put/get tests**

Create `blob_store.rs` integration tests:

```rust
use formula_core::{blob_store::BlobStore, digest::ArtifactDigest};
use tempfile::tempdir;

#[test]
fn blob_path_matches_d3_sha256_layout() {
    let temp = tempdir().unwrap();
    let store = BlobStore::new(temp.path());
    let digest = ArtifactDigest::of_bytes(b"abc");
    let hex = digest.hex();

    assert_eq!(
        store.path_for(digest),
        temp.path()
            .join("objects")
            .join("sha256")
            .join(&hex[..2])
            .join(&hex[2..])
    );
}

#[test]
fn put_is_idempotent_and_get_verifies_content() {
    let temp = tempdir().unwrap();
    let store = BlobStore::new(temp.path());

    let first = store.put(b"authority bytes").unwrap();
    let second = store.put(b"authority bytes").unwrap();

    assert_eq!(first, second);
    assert!(store.contains(first).unwrap());
    assert_eq!(store.get(first).unwrap(), b"authority bytes");
}
```

- [ ] **Step 2: Run and verify RED**

```bash
cargo test -p formula-core --test blob_store blob_
cargo test -p formula-core --test blob_store put_
```

Expected: compilation failure because `BlobStore` is absent.

- [ ] **Step 3: Implement minimal immutable store**

Implementation requirements:

```text
BlobStore::new(root)
    stores the `.formula` root supplied by caller; tests use temp root

path_for(digest)
    root/objects/sha256/first-two-hex/remaining-hex

put(bytes)
    derive digest
    mkdir parent
    if target exists: read+verify existing bytes; return digest if equal
    otherwise write same-directory temporary file using create_new
    flush + sync_all temporary file
    atomically rename to final path
    if race creates final path first, verify final content and remove temp
    return digest

get(digest)
    read bytes
    recompute SHA-256
    return DigestMismatch if recomputed digest differs

contains(digest)
    false only for NotFound
    if file exists, verify via get before returning true
```

No mutable overwrite API exists.

- [ ] **Step 4: Run path/put tests and verify GREEN**

```bash
cargo test -p formula-core --test blob_store
```

Expected: current tests pass.

- [ ] **Step 5: Write failing corruption test**

Append:

```rust
#[test]
fn corrupted_blob_is_rejected_instead_of_returned() {
    let temp = tempdir().unwrap();
    let store = BlobStore::new(temp.path());
    let digest = store.put(b"original").unwrap();

    std::fs::write(store.path_for(digest), b"corrupt").unwrap();

    let err = store.get(digest).unwrap_err();
    assert!(err.to_string().contains("digest mismatch"));
    assert!(store.contains(digest).is_err());
}
```

- [ ] **Step 6: Run corruption test and verify RED if integrity checking is absent/incomplete; make it GREEN**

```bash
cargo test -p formula-core --test blob_store corrupted_blob
```

Expected final result: pass; corrupted bytes are never returned as authority data.

- [ ] **Step 7: Write and prove an immutability/collision guard test**

Add a test-only helper or filesystem setup that creates the target digest path with bytes not matching the requested digest before `put`. `put` must return `DigestMismatch`/`ExistingBlobConflict`; it must not overwrite the file.

- [ ] **Step 8: Run full crate test/lint suite**

```bash
cargo test -p formula-core
cargo fmt --check
cargo clippy -p formula-core --all-targets -- -D warnings
```

Expected: all green, no warnings.

- [ ] **Step 9: Commit Task 3**

```bash
git add crates/formula-core/src crates/formula-core/tests/blob_store.rs
git commit -m "feat(core): add verified immutable blob store"
```

---

### Task 4: B01 authority-vector freeze and stage proof

**Files:**
- Modify: `crates/formula-core/tests/canonical_identity.rs`
- Modify: `crates/formula-core/tests/blob_store.rs` only if test helper cleanup is required
- Create: `docs/proof/f0-b01.md`

**Interfaces:**
- Produces: frozen B01 canonical vectors documenting exact bytes and digests used to detect accidental encoding changes in later stages.

- [ ] **Step 1: Add exact frozen canonical vectors**

Add tests for at least these authority values:

```text
null
true
0
-1
2^128
3/4
"β\n"
[1,"x",sha256(...)]
{"a":1,"b":2}
```

For each vector assert exact canonical bytes and exact SHA-256 digest literal.

Generate the literals once using an independent one-off script/tool, then copy the constants into the tests; the production encoder must not generate its own expected values at test runtime.

- [ ] **Step 2: Run vector tests and verify RED for newly added unfrozen expectations, then GREEN after copying independently derived constants**

```bash
cargo test -p formula-core --test canonical_identity
```

- [ ] **Step 3: Prove deterministic repeated store identity**

Run the exact integration suite twice from clean temporary directories:

```bash
cargo test -p formula-core --test canonical_identity --test blob_store
cargo test -p formula-core --test canonical_identity --test blob_store
```

Expected: identical PASS set both times; no runtime/time/path value changes canonical digests.

- [ ] **Step 4: Run the complete B01 verification commands**

```bash
cargo test -p formula-core
cargo fmt --check
cargo clippy -p formula-core --all-targets -- -D warnings
cargo metadata --locked --format-version 1 > /tmp/formula-b01-metadata.json
```

Expected: all tests pass, formatting clean, clippy has zero warnings, locked dependency graph resolves.

- [ ] **Step 5: Write `docs/proof/f0-b01.md` with exact evidence**

The proof note must record:

```text
stage: F0/B01
spec: D3 B01
toolchain: rustc 1.98.0
source commit: <exact B01 candidate SHA>
Cargo.lock SHA-256: <digest>
canonical_identity test result: <count> passed, 0 failed
blob_store test result: <count> passed, 0 failed
fmt: PASS
clippy -D warnings: PASS
frozen authority vectors: PASS
blob corruption rejection: PASS
immutable/idempotent put: PASS
```

No B02 completion claim is permitted.

- [ ] **Step 6: Review B01 against D3 before freeze**

Explicitly confirm:

```text
SHA-256 identity only
restricted JSON only
no floats in authority model
canonical integer/rational rules
no local path/time metadata in identity
D3 path layout
immutable blob semantics
verified reads
no D1 schemas accidentally implemented early
```

- [ ] **Step 7: Commit the B01 proof note**

```bash
git add Cargo.lock docs/proof/f0-b01.md crates/formula-core/tests
git commit -m "proof: freeze F0 B01 canonical identity"
```

---

## Self-Review

### Spec coverage

- D3 stable Rust pin: Task 1.
- SHA-256 authority identity: Tasks 1–4.
- deterministic restricted JSON: Task 2 + frozen vectors in Task 4.
- arbitrary precision integer/rational canonicalization: Task 2.
- no float authority values: type model in Task 2.
- immutable SHA-256 object layout: Task 3.
- content verification/corruption rejection: Task 3.
- deterministic identity tests: Task 4.
- no B02 scope creep: global constraints + Task 4 review.

### Placeholder scan

No implementation step uses TODO/TBD/placeholder behavior. The only values intentionally produced during execution are exact commit and Cargo.lock digests, which cannot exist before the implementation commit and are explicitly required evidence outputs rather than unspecified design.

### Type consistency

`ArtifactDigest` is introduced in Task 1 and consumed unchanged by Tasks 2–4. `CanonicalValue` is introduced in Task 2. `BlobStore` consumes `ArtifactDigest` in Task 3. No later task renames these interfaces.

## Execution boundary

Implement this B01 plan only. Do not begin B02 until B01 has a reviewed exact candidate commit and `docs/proof/f0-b01.md` records passing evidence.
