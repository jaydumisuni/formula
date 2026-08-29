# P1 Core Identity + Authority Store Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the minimum D1/D2 durable artifact identity system, immutable content-addressed storage, atomic Universe-generation publication, and historical replay required by roadmap phase P1.

**Architecture:** Preserve the proved P0 crate boundaries. `formula-core` owns immutable semantic schemas, canonical authority encoding, SHA-256 structural identity, and generation manifests; `formula-store` owns physical blob persistence and the SQLite-backed authority index/transaction layer. SQLite remains an implementation detail: generation and artifact identities are computed solely from canonical semantic bytes. The older `f0/b01-canonical-identity` branch is evidence only; compatible digest/canonicalization behavior is ported through fresh RED/GREEN tests onto the P0 base rather than merged wholesale.

**Tech Stack:** Rust 1.98.0, Cargo, `sha2 = 0.10`, `num-bigint = 0.4`, `num-integer = 0.1`, `num-traits = 0.2`, `rusqlite = 0.40.2` with `bundled`, `tempfile = 3` for tests only.

**Spec:** `docs/roadmap/2026-08-28-implementation-roadmap.md` P1; `docs/design/2026-08-28-d1-mathematical-constitution.md` sections 3–6; `docs/design/2026-08-28-d2-operational-mathematical-machine.md`; `docs/design/2026-08-28-d3-first-light-build-architecture.md` sections 2–5.

## Global Constraints

- Base exactly on proved P0 branch tip `feaa80d964f61a1829fd6b3b6a563a9424316950`; do not rewrite or merge the old B01 branch.
- Keep Rust pinned exactly to `1.98.0`.
- Authority digests are SHA-256.
- Canonical authority encoding v1 is deterministic UTF-8 restricted JSON.
- No IEEE floating-point value may participate in structural identity.
- Integers are arbitrary precision; rationals are reduced with a positive denominator.
- Object fields are encoded in deterministic lexicographic key order.
- Machine-local path, timestamp, process ID, random nonce, and filesystem metadata are excluded from structural identity.
- Structural identity and semantic equivalence remain distinct: certified equivalence is represented by a Judgement/Evidence relation, never by hash aliasing.
- `formula-core` contains no filesystem, SQLite, search, promotion-policy, or mutable authority implementation.
- `formula-store` may depend on `formula-core`; `formula-core` must not depend on `formula-store`.
- Blob writes are immutable and idempotent; every read re-verifies SHA-256.
- Generation manifests are immutable and content-addressed.
- Publishing a generation is atomic at the authority-index boundary. Failure before commit may leave unreachable immutable files but must not change the active generation.
- Historical generation replay must reconstruct canonical manifest bytes from indexed authority rows and verify the requested digest.
- P0 architectural tests remain green. Runtime dependency allowlisting must stay explicit as new pinned dependencies enter the First-Light closure.
- TDD is mandatory: each production behavior is preceded by a failing test observed failing for the intended reason.

---

## File Structure

Create or modify:

```text
Cargo.toml
Cargo.lock
crates/formula-core/Cargo.toml
crates/formula-core/src/lib.rs
crates/formula-core/src/digest.rs
crates/formula-core/src/canonical.rs
crates/formula-core/src/artifacts.rs
crates/formula-core/src/generation.rs
crates/formula-core/tests/canonical_identity.rs
crates/formula-core/tests/artifact_identity.rs
crates/formula-core/tests/generation_identity.rs
crates/formula-store/Cargo.toml
crates/formula-store/src/lib.rs
crates/formula-store/src/blob_store.rs
crates/formula-store/src/authority_store.rs
crates/formula-store/tests/blob_store.rs
crates/formula-store/tests/generation_replay.rs
tests/authority-boundary/runtime-allowlist.txt
.github/workflows/p1-branch-ci.yml
docs/checkpoints/2026-08-29-p1-core-identity-authority-store.md
CURRENT.md
```

Responsibilities:

- `digest.rs` — strict `ArtifactDigest` SHA-256 identity.
- `canonical.rs` — restricted canonical authority-value model and encoder.
- `artifacts.rs` — immutable P1 semantic metadata schemas and their structural projections.
- `generation.rs` — deterministic `UniverseGeneration` manifest construction/identity.
- `blob_store.rs` — physical immutable SHA-256 object store.
- `authority_store.rs` — SQLite authority index, active-generation pointer, atomic publication, replay.
- `generation_replay.rs` — D2-P01/P02/P03/P11 proof-level integration tests.
- `runtime-allowlist.txt` — exact reviewed runtime package closure after P1 dependencies are locked.

---

### Task 1: Port canonical digest and restricted authority encoding onto the P0 base

**Files:**
- Modify: `Cargo.toml`
- Modify: `crates/formula-core/Cargo.toml`
- Modify: `crates/formula-core/src/lib.rs`
- Create: `crates/formula-core/src/digest.rs`
- Create: `crates/formula-core/src/canonical.rs`
- Create: `crates/formula-core/tests/canonical_identity.rs`
- Modify after lock review: `tests/authority-boundary/runtime-allowlist.txt`

**Interfaces:**
- Produces: `ArtifactDigest::of_bytes(&[u8]) -> ArtifactDigest`
- Produces: `ArtifactDigest::parse(&str) -> Result<ArtifactDigest, DigestError>`
- Produces: `ArtifactDigest::as_str() -> String`
- Produces: `CanonicalRational::new(BigInt, BigInt) -> Result<CanonicalRational, CanonicalError>`
- Produces: `CanonicalValue::to_canonical_bytes() -> Vec<u8>`
- Produces: `CanonicalValue::digest() -> ArtifactDigest`

- [ ] **Step 1: Add only dependency declarations and failing external identity tests**

Add workspace dependencies:

```toml
[workspace.dependencies]
sha2 = "0.10"
num-bigint = "0.4"
num-integer = "0.1"
num-traits = "0.2"
rusqlite = { version = "0.40.2", features = ["bundled"] }
tempfile = "3"
```

`formula-core/Cargo.toml` dependencies:

```toml
[dependencies]
sha2.workspace = true
num-bigint.workspace = true
num-integer.workspace = true
num-traits.workspace = true
```

Create `canonical_identity.rs` with the frozen old-branch vectors plus the missing zero-rational normalization vector:

```rust
use formula_core::canonical::{CanonicalRational, CanonicalValue};
use formula_core::digest::ArtifactDigest;
use num_bigint::BigInt;
use std::collections::BTreeMap;

#[test]
fn digest_of_bytes_is_sha256_and_round_trips() {
    let digest = ArtifactDigest::of_bytes(b"abc");
    assert_eq!(digest.as_str(), "sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    assert_eq!(ArtifactDigest::parse(&digest.as_str()).unwrap(), digest);
}

#[test]
fn digest_parser_rejects_noncanonical_forms() {
    assert!(ArtifactDigest::parse("md5:ba7816bf").is_err());
    assert!(ArtifactDigest::parse("sha256:ABCDEF").is_err());
    assert!(ArtifactDigest::parse("sha256:00").is_err());
    assert!(ArtifactDigest::parse("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad").is_err());
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
```

- [ ] **Step 2: Run RED**

```bash
cargo test -p formula-core --test canonical_identity
```

Expected: compile failure because `formula_core::digest` / `formula_core::canonical` do not exist on the P0 base.

- [ ] **Step 3: Port the reviewed digest/canonical implementation**

Port the behavior from old commits `f7f67df.../c3339ab...`, with one correction: `CanonicalRational::new(0, d)` must normalize to `0/1` before returning. `CanonicalValue` is exactly:

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

`ArtifactDigest` remains a private `[u8; 32]` wrapper with strict lowercase `sha256:<64 hex>` parsing. The encoder uses a private JSON-string writer and never `serde_json`.

- [ ] **Step 4: Run GREEN and static gates**

```bash
cargo test -p formula-core --test canonical_identity
cargo fmt --all -- --check
cargo clippy -p formula-core --all-targets -- -D warnings
```

Expected: all pass.

- [ ] **Step 5: Generate/inspect `Cargo.lock` and update the runtime allowlist explicitly**

```bash
cargo generate-lockfile
cargo tree --locked -p formula-first-light --prefix none | awk '{print $1}' | sort -u
cargo tree --locked -p formula-cli --prefix none | awk '{print $1}' | sort -u
```

Add every reviewed runtime package name to `tests/authority-boundary/runtime-allowlist.txt`; do not wildcard or disable the P0 test. Then run:

```bash
cargo test -p formula-archtest --test runtime_network_policy --locked
```

- [ ] **Step 6: Commit Task 1**

```bash
git add Cargo.toml Cargo.lock crates/formula-core tests/authority-boundary/runtime-allowlist.txt
git commit -m "feat(core): port canonical structural identity"
```

---

### Task 2: Implement immutable P1 semantic schemas and structural projections

**Files:**
- Create: `crates/formula-core/src/artifacts.rs`
- Modify: `crates/formula-core/src/lib.rs`
- Create: `crates/formula-core/tests/artifact_identity.rs`

**Interfaces:**
- Produces: `StructuralIdentity` trait with `canonical_value(&self) -> CanonicalValue` and default `structural_digest()`.
- Produces schemas: `Entity`, `Relation`, `World`, `Judgement`, `EvidenceEnvelope`, `RealizationMetadata`, `AuthorityContract`, `Observer`.

- [ ] **Step 1: Write RED tests proving semantic fields enter identity and non-semantic metadata cannot**

```rust
use formula_core::{
    artifacts::{Entity, Judgement, StructuralIdentity},
    canonical::CanonicalValue,
    digest::ArtifactDigest,
};
use std::collections::BTreeMap;

fn d(label: &[u8]) -> ArtifactDigest { ArtifactDigest::of_bytes(label) }

#[test]
fn entity_structural_identity_is_stable_and_reference_sensitive() {
    let structure = CanonicalValue::Object(BTreeMap::from([
        ("kind".into(), CanonicalValue::String("integer-literal".into())),
        ("value".into(), CanonicalValue::Integer(17.into())),
    ]));
    let a = Entity::new(d(b"foundation"), structure.clone(), vec![d(b"parent")]);
    let b = Entity::new(d(b"foundation"), structure, vec![d(b"parent")]);
    assert_eq!(a.structural_digest(), b.structural_digest());

    let changed = Entity::new(d(b"foundation"), CanonicalValue::Integer(17.into()), vec![d(b"different-parent")]);
    assert_ne!(a.structural_digest(), changed.structural_digest());
}

#[test]
fn machine_metadata_is_not_an_entity_identity_input() {
    let entity = Entity::new(d(b"foundation"), CanonicalValue::Integer(17.into()), vec![]);
    let before = entity.structural_digest();
    let local_path = "/tmp/machine-A/17";
    let timestamp = "2026-08-29T04:00:00Z";
    assert!(!entity.canonical_value().to_canonical_bytes().windows(local_path.len()).any(|w| w == local_path.as_bytes()));
    assert!(!entity.canonical_value().to_canonical_bytes().windows(timestamp.len()).any(|w| w == timestamp.as_bytes()));
    assert_eq!(before, entity.structural_digest());
}

#[test]
fn semantic_equivalence_does_not_alias_structural_digest() {
    let x_plus_x = Entity::new(d(b"foundation"), CanonicalValue::String("x+x".into()), vec![]);
    let two_x = Entity::new(d(b"foundation"), CanonicalValue::String("2*x".into()), vec![]);
    assert_ne!(x_plus_x.structural_digest(), two_x.structural_digest());

    let equivalence = Judgement::new(
        d(b"world"),
        CanonicalValue::Object(BTreeMap::from([
            ("kind".into(), CanonicalValue::String("Equivalent".into())),
            ("left".into(), CanonicalValue::Digest(x_plus_x.structural_digest())),
            ("right".into(), CanonicalValue::Digest(two_x.structural_digest())),
        ])),
        vec![x_plus_x.structural_digest(), two_x.structural_digest()],
    );
    assert_ne!(equivalence.structural_digest(), x_plus_x.structural_digest());
}
```

- [ ] **Step 2: Run RED**

```bash
cargo test -p formula-core --test artifact_identity
```

Expected: compile failure because `artifacts` and `StructuralIdentity` do not exist.

- [ ] **Step 3: Implement the schemas with only semantic fields**

Use this common trait:

```rust
pub trait StructuralIdentity {
    fn canonical_value(&self) -> CanonicalValue;
    fn structural_digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}
```

All structs are immutable value types whose constructors sort/deduplicate digest sets where mathematical meaning is set-like. Every canonical object includes `"schema":"formula-authority-v1"` and an exact `"kind"` field. Constructors expose no timestamp/path fields.

Required structural fields:

```text
Entity: foundation, normalized exact structure, referenced entity/artifact digests
Relation: foundation, arity, semantic definition, referenced artifact digests
World: parent worlds, assumptions, local definitions, equalities, disequalities, foundation
Judgement: world, proposition, referenced artifact digests
EvidenceEnvelope: target judgement, world, scope, evidence family/body digest, producer/checker/trust-root/verdict, dependency digests, replay binding
RealizationMetadata: semantic target, realization kind, source/binary digest, input/output semantics, independent validation evidence digest
AuthorityContract: requested authority class, allowed evidence families, exactness requirement
Observer: observer family, projection/return semantics
```

Do not introduce `verified: bool` on `Judgement`.

- [ ] **Step 4: Run GREEN and all core tests**

```bash
cargo test -p formula-core --locked
cargo fmt --all -- --check
cargo clippy -p formula-core --all-targets --locked -- -D warnings
```

- [ ] **Step 5: Commit Task 2**

```bash
git add crates/formula-core/src crates/formula-core/tests/artifact_identity.rs
git commit -m "feat(core): add P1 durable semantic schemas"
```

---

### Task 3: Implement deterministic Universe-generation manifest identity

**Files:**
- Create: `crates/formula-core/src/generation.rs`
- Modify: `crates/formula-core/src/lib.rs`
- Create: `crates/formula-core/tests/generation_identity.rs`

**Interfaces:**
- Produces: `UniverseGeneration::new(number: u64, parent: Option<ArtifactDigest>, admitted: Vec<ArtifactDigest>, authority_bindings: Vec<ArtifactDigest>) -> UniverseGeneration`
- Produces: `UniverseGeneration::canonical_bytes() -> Vec<u8>`
- Produces: `UniverseGeneration::digest() -> ArtifactDigest`

- [ ] **Step 1: Write RED deterministic-manifest tests**

```rust
use formula_core::{digest::ArtifactDigest, generation::UniverseGeneration};
fn d(x: &[u8]) -> ArtifactDigest { ArtifactDigest::of_bytes(x) }

#[test]
fn generation_identity_is_order_independent_for_set_like_members() {
    let a = UniverseGeneration::new(1, Some(d(b"u0")), vec![d(b"b"), d(b"a")], vec![d(b"e2"), d(b"e1")]);
    let b = UniverseGeneration::new(1, Some(d(b"u0")), vec![d(b"a"), d(b"b")], vec![d(b"e1"), d(b"e2")]);
    assert_eq!(a.canonical_bytes(), b.canonical_bytes());
    assert_eq!(a.digest(), b.digest());
}

#[test]
fn parent_or_authority_delta_changes_generation_root() {
    let base = UniverseGeneration::new(1, Some(d(b"u0")), vec![d(b"a")], vec![d(b"e1")]);
    let changed = UniverseGeneration::new(1, Some(d(b"other")), vec![d(b"a")], vec![d(b"e1")]);
    assert_ne!(base.digest(), changed.digest());
}
```

- [ ] **Step 2: Run RED**

```bash
cargo test -p formula-core --test generation_identity
```

Expected: compile failure because `generation` does not exist.

- [ ] **Step 3: Implement deterministic generation construction**

`UniverseGeneration::new` sorts and deduplicates `admitted` and `authority_bindings`. Its canonical value is exactly:

```text
{
  "admitted": [...canonical digest strings...],
  "authority_bindings": [...canonical digest strings...],
  "generation_number": <integer>,
  "kind": "UniverseGeneration",
  "parent": <digest string or null>,
  "schema": "formula-authority-v1"
}
```

No timestamp, path, SQLite row id, or process metadata enters these bytes.

- [ ] **Step 4: Run GREEN**

```bash
cargo test -p formula-core --test generation_identity --locked
cargo test -p formula-core --locked
```

- [ ] **Step 5: Commit Task 3**

```bash
git add crates/formula-core/src/generation.rs crates/formula-core/src/lib.rs crates/formula-core/tests/generation_identity.rs
git commit -m "feat(core): add deterministic Universe generation manifests"
```

---

### Task 4: Implement immutable verified content-addressed blob storage in `formula-store`

**Files:**
- Modify: `crates/formula-store/Cargo.toml`
- Modify: `crates/formula-store/src/lib.rs`
- Create: `crates/formula-store/src/blob_store.rs`
- Create: `crates/formula-store/tests/blob_store.rs`

**Interfaces:**
- Consumes: `ArtifactDigest`
- Produces: `BlobStore::new(root: impl AsRef<Path>) -> BlobStore`
- Produces: `put(&self, bytes: &[u8]) -> Result<ArtifactDigest, BlobStoreError>`
- Produces: `get(&self, digest: ArtifactDigest) -> Result<Vec<u8>, BlobStoreError>`
- Produces: `contains(&self, digest: ArtifactDigest) -> Result<bool, BlobStoreError>`
- Produces: `path_for(&self, digest: ArtifactDigest) -> PathBuf`

- [ ] **Step 1: Add only `tempfile` dev dependency and RED tests**

```rust
use formula_core::digest::ArtifactDigest;
use formula_store::blob_store::{BlobStore, BlobStoreError};
use tempfile::tempdir;
use std::fs;

#[test]
fn blob_path_matches_d3_layout() {
    let dir = tempdir().unwrap();
    let store = BlobStore::new(dir.path());
    let digest = ArtifactDigest::of_bytes(b"abc");
    let hex = digest.hex();
    assert_eq!(store.path_for(digest), dir.path().join("objects/sha256").join(&hex[..2]).join(&hex[2..]));
}

#[test]
fn put_is_idempotent_and_read_is_verified() {
    let dir = tempdir().unwrap();
    let store = BlobStore::new(dir.path());
    let d1 = store.put(b"authority bytes").unwrap();
    let d2 = store.put(b"authority bytes").unwrap();
    assert_eq!(d1, d2);
    assert_eq!(store.get(d1).unwrap(), b"authority bytes");
}

#[test]
fn mutated_blob_is_rejected() {
    let dir = tempdir().unwrap();
    let store = BlobStore::new(dir.path());
    let digest = store.put(b"authority bytes").unwrap();
    fs::write(store.path_for(digest), b"tampered").unwrap();
    assert!(matches!(store.get(digest), Err(BlobStoreError::DigestMismatch { .. })));
}
```

- [ ] **Step 2: Run RED**

```bash
cargo test -p formula-store --test blob_store
```

Expected: compile failure because `blob_store` does not exist.

- [ ] **Step 3: Implement immutable store**

`put` must:

1. derive digest from bytes;
2. create `objects/sha256/<first-two>/<remaining>` parent;
3. if target exists, read and verify its digest; return only when bytes/digest match;
4. otherwise write a same-directory temporary file with `OpenOptions::create_new(true)`;
5. `write_all`, `sync_all`, then rename into place;
6. if a race creates the target first, verify the winner rather than overwrite it;
7. never replace a different existing blob.

`get` reads bytes and recomputes SHA-256; mismatch returns `DigestMismatch`.

- [ ] **Step 4: Run GREEN**

```bash
cargo test -p formula-store --test blob_store --locked
cargo clippy -p formula-store --all-targets --locked -- -D warnings
```

- [ ] **Step 5: Commit Task 4**

```bash
git add crates/formula-store
git commit -m "feat(store): add immutable verified blob store"
```

---

### Task 5: Implement SQLite authority index and atomic generation publication

**Files:**
- Modify: `crates/formula-store/Cargo.toml`
- Create: `crates/formula-store/src/authority_store.rs`
- Modify: `crates/formula-store/src/lib.rs`
- Create: `crates/formula-store/tests/generation_replay.rs`

**Interfaces:**
- Produces: `AuthorityStore::open(root: impl AsRef<Path>) -> Result<AuthorityStore, AuthorityStoreError>`
- Produces: `initialize_genesis(&mut self, generation: &UniverseGeneration) -> Result<ArtifactDigest, AuthorityStoreError>`
- Produces: `publish_generation(&mut self, generation: &UniverseGeneration) -> Result<ArtifactDigest, AuthorityStoreError>`
- Produces: `active_generation(&self) -> Result<Option<ArtifactDigest>, AuthorityStoreError>`
- Produces: `replay_generation(&self, digest: ArtifactDigest) -> Result<UniverseGeneration, AuthorityStoreError>`

- [ ] **Step 1: Add `rusqlite.workspace = true` to `formula-store` and write RED atomic-publication tests**

```rust
use formula_core::{digest::ArtifactDigest, generation::UniverseGeneration};
use formula_store::authority_store::AuthorityStore;
use tempfile::tempdir;

fn d(x: &[u8]) -> ArtifactDigest { ArtifactDigest::of_bytes(x) }

#[test]
fn successful_generation_publish_moves_active_root_once() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![d(b"base")], vec![]);
    let u0d = store.initialize_genesis(&u0).unwrap();
    assert_eq!(store.active_generation().unwrap(), Some(u0d));

    let u1 = UniverseGeneration::new(1, Some(u0d), vec![d(b"base"), d(b"new")], vec![d(b"proof")]);
    let u1d = store.publish_generation(&u1).unwrap();
    assert_eq!(store.active_generation().unwrap(), Some(u1d));
    assert_eq!(store.replay_generation(u0d).unwrap().digest(), u0d);
}

#[test]
fn wrong_parent_cannot_publish() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![], vec![]);
    store.initialize_genesis(&u0).unwrap();
    let bad = UniverseGeneration::new(1, Some(d(b"not-active")), vec![d(b"x")], vec![]);
    assert!(store.publish_generation(&bad).is_err());
    assert_eq!(store.active_generation().unwrap(), Some(u0.digest()));
}
```

- [ ] **Step 2: Run RED**

```bash
cargo test -p formula-store --test generation_replay successful_generation_publish_moves_active_root_once
```

Expected: compile failure because `authority_store` does not exist.

- [ ] **Step 3: Implement schema and normal publication path**

On `open`, create `.formula` root (the caller supplies the root itself in tests), `authority.sqlite`, `generations/`, and these tables inside SQLite:

```sql
CREATE TABLE IF NOT EXISTS meta (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS generations (
  digest TEXT PRIMARY KEY,
  generation_number INTEGER NOT NULL,
  parent_digest TEXT,
  manifest_blob_digest TEXT NOT NULL UNIQUE
);
CREATE TABLE IF NOT EXISTS generation_admitted (
  generation_digest TEXT NOT NULL,
  artifact_digest TEXT NOT NULL,
  PRIMARY KEY (generation_digest, artifact_digest)
);
CREATE TABLE IF NOT EXISTS generation_authority_bindings (
  generation_digest TEXT NOT NULL,
  evidence_digest TEXT NOT NULL,
  PRIMARY KEY (generation_digest, evidence_digest)
);
```

`publish_generation` algorithm:

1. canonicalize and digest manifest;
2. persist canonical bytes in BlobStore and `generations/<digest>.json` immutably;
3. begin `TransactionBehavior::Immediate`;
4. read active root from `meta`;
5. require manifest parent == active root and generation number == parent number + 1;
6. insert generation/admitted/binding rows;
7. set `meta.active_generation = digest` inside the same transaction;
8. commit;
9. return digest.

Genesis is the same transaction shape but requires no active root, `parent=None`, `generation_number=0`.

- [ ] **Step 4: Run GREEN normal-path tests**

```bash
cargo test -p formula-store --test generation_replay successful_generation_publish_moves_active_root_once wrong_parent_cannot_publish --locked
```

- [ ] **Step 5: Add private failure-injection unit tests before commit**

Inside `authority_store.rs`, make production `publish_generation` call a private:

```rust
fn publish_generation_inner(
    &mut self,
    generation: &UniverseGeneration,
    failpoint: PublishFailpoint,
) -> Result<ArtifactDigest, AuthorityStoreError>
```

with private enum:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PublishFailpoint { None, AfterRowsBeforeActive, AfterActiveBeforeCommit }
```

Unit tests call the private function and assert after each injected error:

```text
active_generation remains old root
new generation is not replayable as authoritative DB state
old generation still replays exactly
unreachable immutable manifest/blob files are permitted
```

- [ ] **Step 6: Run RED by first adding the tests with failpoint branches returning injected errors before rollback assertions are satisfied, then implement transaction rollback behavior and run GREEN**

```bash
cargo test -p formula-store authority_store::tests::failure --locked
cargo test -p formula-store --locked
```

- [ ] **Step 7: Commit Task 5**

```bash
git add crates/formula-store Cargo.toml Cargo.lock tests/authority-boundary/runtime-allowlist.txt
git commit -m "feat(store): publish Universe generations atomically"
```

---

### Task 6: Prove deterministic replay and historical authority preservation

**Files:**
- Modify: `crates/formula-store/tests/generation_replay.rs`
- Modify if required: `crates/formula-store/src/authority_store.rs`

**Interfaces:**
- Consumes all prior P1 APIs.
- Proves D2-P01, D2-P02, D2-P03, D2-P11.

- [ ] **Step 1: Add fresh-reopen replay test**

```rust
#[test]
fn historical_roots_replay_after_fresh_store_open() {
    let dir = tempdir().unwrap();
    let (u0d, u1d) = {
        let mut store = AuthorityStore::open(dir.path()).unwrap();
        let u0 = UniverseGeneration::new(0, None, vec![d(b"a")], vec![]);
        let u0d = store.initialize_genesis(&u0).unwrap();
        let u1 = UniverseGeneration::new(1, Some(u0d), vec![d(b"a"), d(b"b")], vec![d(b"proof-b")]);
        let u1d = store.publish_generation(&u1).unwrap();
        (u0d, u1d)
    };

    let reopened = AuthorityStore::open(dir.path()).unwrap();
    assert_eq!(reopened.active_generation().unwrap(), Some(u1d));
    assert_eq!(reopened.replay_generation(u0d).unwrap().digest(), u0d);
    assert_eq!(reopened.replay_generation(u1d).unwrap().digest(), u1d);
}
```

- [ ] **Step 2: Add generation-file tamper rejection test**

After publication, overwrite `generations/<digest>.json` with different bytes and require replay to return a digest/integrity error rather than trusting SQLite rows alone.

- [ ] **Step 3: Run RED for tamper detection, then implement replay verification**

`replay_generation` must:

1. load generation number/parent and sorted admitted/binding rows from SQLite;
2. reconstruct `UniverseGeneration` in memory;
3. canonicalize it and require reconstructed digest == requested digest;
4. read `generations/<digest>.json` and require exact bytes == reconstructed canonical bytes;
5. verify the same bytes through BlobStore using `manifest_blob_digest`;
6. return the generation only after all checks pass.

- [ ] **Step 4: Run complete P1 functional suite**

```bash
cargo test -p formula-core --locked --offline
cargo test -p formula-store --locked --offline
cargo test -p formula-archtest --locked --offline
```

Expected: all pass, including P0 architecture tests.

- [ ] **Step 5: Commit Task 6**

```bash
git add crates/formula-store/tests crates/formula-store/src/authority_store.rs
git commit -m "test(store): prove deterministic historical generation replay"
```

---

### Task 7: Run/freeze the canonical P1 proof gate

**Files:**
- Create: `.github/workflows/p1-branch-ci.yml`
- Create after proof: `docs/checkpoints/2026-08-29-p1-core-identity-authority-store.md`
- Modify after proof: `CURRENT.md`

**Interfaces:**
- Produces the P1 evidence checkpoint and next recovery boundary.

- [ ] **Step 1: Create the branch-only proof workflow**

The workflow must checkout `implementation/p1-core-identity-authority-store`, install exact Rust 1.98.0, and run in this order:

```bash
git status --porcelain
git rev-parse HEAD
rustc -vV
cargo -V
cargo metadata --locked --offline --format-version 1
cargo test -p formula-archtest --locked --offline
cargo test -p formula-core --locked --offline
cargo test -p formula-store --locked --offline
cargo test --workspace --locked --offline
cargo build --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo tree --locked --offline -p formula-check
cargo tree --locked --offline -p formula-engine
cargo tree --locked --offline -p formula-first-light
git status --porcelain
```

- [ ] **Step 2: Require a complete green proof run**

Do not call P1 complete if any step is skipped or fails. Recover exact logs for the four roadmap obligations and the negative tests.

- [ ] **Step 3: Write the P1 checkpoint only after proof**

The checkpoint records:

```text
exact branch/head
Rust/cargo versions
Cargo.lock identity
canonical digest/encoding vectors
blob mutation rejection evidence
P1 D2-P01 deterministic structural replay
P1 D2-P02 atomic publication + injected-failure rollback
P1 D2-P03 structural identity distinct from certified semantic equivalence
P1 D2-P11 historical root replay after fresh reopen
runtime dependency closure evidence
full proof run ID
scope note: hosted-runner proof is not mislabeled as a separate workstation replay
```

- [ ] **Step 4: Update `CURRENT.md` to the proved P1 boundary and canonical P2 next phase**

Only after the checkpoint proof is green. Preserve evidence precedence and note that P0 remains frozen history.

- [ ] **Step 5: Rerun the complete proof gate at the exact post-checkpoint/post-CURRENT branch tip**

P1 is frozen only if the final branch tip remains fully green.

- [ ] **Step 6: Commit/freeze and stop before P2 production code**

The final branch tip becomes the P1 recovery authority. P2 starts on a new branch after P1 is frozen.

---

## Self-Review

- P1 roadmap build scope is covered: all named P1 schema families, canonical encoding v1, SHA-256 identity, immutable blob storage, SQLite authority index/transaction, generation build/load/replay.
- D2-P01 is covered by canonical fixture/order tests plus fresh-reopen replay.
- D2-P02 is covered by atomic transaction tests and two explicit failure injection points.
- D2-P03 is covered by a test requiring structurally distinct but semantically equivalent entities to retain distinct digests connected by a Judgement.
- D2-P11 is covered by replay of both U0 and U1 after closing and reopening the store.
- Negative tests cover field/insertion order, non-semantic metadata exclusion, blob mutation rejection, wrong parent rejection, partial transaction rollback, and generation-file tamper rejection.
- P0 checker/search/sealed/runtime boundary tests remain part of every P1 proof run.
- No P2 checker/certificate behavior, capability closure, discovery, promotion policy, First-Light targets, realization, Ptah, GPU, model, UI, or network behavior is introduced here.
