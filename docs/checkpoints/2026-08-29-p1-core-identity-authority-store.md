# P1 Checkpoint — Core Identity + Authority Store

**Date:** 2026-08-29  
**Status:** PROVED — deterministic structural identity, immutable storage, atomic authority publication, and historical replay  
**Branch:** `implementation/p1-core-identity-authority-store`  
**Source-under-test commit:** `5218ce5cd35636c080ad569391d48aa62f5d3cc0`  
**Canonical proof run:** `33236548151`  
**Canonical proof job:** `99058414531`  
**Roadmap authority:** `docs/roadmap/2026-08-28-implementation-roadmap.md`, phase P1  
**Implementation plan:** `docs/superpowers/plans/2026-08-29-p1-core-identity-authority-store.md`

---

## 1. Scope

This checkpoint proves canonical roadmap phase **P1 — Core structural identity and authority store**.

P1 establishes:

- deterministic canonical serialization for structural identity;
- SHA-256 `ArtifactDigest` identities;
- immutable D1/P1 semantic schema families with structural projections;
- an explicit separation between structural identity and semantic equivalence;
- deterministic `UniverseGeneration` roots;
- immutable content-addressed blob storage with verified reads;
- SQLite-backed authority indexing;
- atomic generation publication with rollback-safe failure semantics;
- historical generation reconstruction and replay;
- replay verification across authority rows, canonical generation bytes, generation manifest files, and content-addressed blob backing;
- an explicit locked runtime dependency closure.

This checkpoint does **not** claim parser correctness, evaluator correctness, dimensional/unit semantics, affine/delta behavior, discovery, certification semantics, promotion beyond the P1 generation transaction, First Light, or native realization. Those remain later roadmap work.

---

## 2. Canonical proof environment

The final P1 proof ran from the exact source-under-test commit on a GitHub-hosted Ubuntu 24.04 runner using the repository-pinned Rust 1.98.0 toolchain.

```text
source commit: 5218ce5cd35636c080ad569391d48aa62f5d3cc0
workflow run: 33236548151
job: 99058414531
runner: ubuntu-24.04
```

Pinned compiler/tool metadata:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
commit-hash: 88d9e12ae178fab0fb5cc050a94da85685d449ea
host: x86_64-unknown-linux-gnu
release: 1.98.0
LLVM version: 22.1.8

cargo 1.98.0 (797e8a9bc 2026-08-05)
```

The hosted workflow used network access only to checkout/provision the pinned toolchain and prime Cargo's locked cache. The canonical metadata/test/build/clippy/tree proof sequence itself ran with `--locked --offline` where Cargo dependency resolution/execution was involved.

---

## 3. Canonical structural identity vectors

The proved identity vectors include:

```text
SHA-256(b"abc")
sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad

canonical object inserted as z=1, a=2
{"a":2,"z":1}

rational 2/-4
-1/2

rational 0/-9
0/1
```

Additional proved identity rules:

- canonical object identity is independent of insertion order;
- string escaping is deterministic UTF-8 JSON encoding;
- uppercase SHA-256 hex is rejected as noncanonical;
- zero denominators are rejected;
- non-semantic machine-local metadata has no structural-identity surface;
- semantically equivalent objects do not alias distinct structural digests;
- changing referenced structural content changes the structural digest.

---

## 4. Immutable P1 schema families

The P1 core schema layer makes the required semantic artifact families structurally addressable, including the roadmap's entity/assumption/source-formula/dimension/unit/constant/affine-delta/equivalence/generation authority surfaces.

The constitutional split is explicit:

```text
structural digest identity != semantic equivalence
```

Equivalence may relate artifacts; it does not rewrite or alias their content-addressed structural identity.

---

## 5. UniverseGeneration identity

`UniverseGeneration` is deterministically encoded and digested.

Proved properties:

- set-like member ordering does not affect the root;
- duplicate set-like members normalize away;
- changing a parent generation changes the root;
- changing authority bindings changes the root;
- generation identity exposes no machine-local metadata surface.

The resulting generation root is therefore a structural commitment to the generation's parent and admitted authority references, not to incidental execution state.

---

## 6. Immutable blob-store proof

The P1 blob store is content addressed by `ArtifactDigest` and uses the frozen D3 path layout.

Proved behavior:

- publishing identical bytes under the same digest is idempotent;
- immutable publication uses a no-clobber atomic finalization path;
- reads re-hash bytes before returning them;
- an on-disk mutation is rejected;
- an existing digest path is never silently overwritten with different content.

This proves immutable byte backing for P1 authority artifacts.

---

## 7. Atomic authority publication proof

The SQLite authority store publishes a generation only after validating the active parent and generation sequence.

Publication uses one authority transaction around the authoritative rows and active-root movement. Immutable bytes/manifests are staged before authority publication.

Normal path proved:

```text
U0 active
 -> validate U1(parent = U0)
 -> insert generation/authority rows
 -> update active root to U1
 -> COMMIT
 -> U1 active
```

Wrong-parent publication is rejected and leaves the previous root active.

Injected failure boundaries were then proved explicitly:

```text
failure after authority rows, before active-root update
    -> transaction rollback
    -> U0 remains active/replayable
    -> U1 is not authoritative

failure after active-root update, before COMMIT
    -> transaction rollback
    -> U0 remains active/replayable
    -> U1 is not authoritative
```

Therefore a failed P1 publication cannot expose partial authority.

---

## 8. Historical replay proof

Historical generations remain reconstructable by exact root after a fresh store reopen.

Replay does not trust one persistence layer in isolation. A replayed generation must agree across:

```text
SQLite authority rows
 -> canonical reconstructed generation bytes
 -> generation manifest file
 -> content-addressed blob
 -> expected generation digest
```

Proved results:

- U0 and U1 replay after a fresh process/store open;
- tampering with the generation manifest file is rejected;
- tampering with the content-addressed manifest blob is rejected;
- replay returns authority only when the reconstructed bytes and immutable backing agree with the requested root.

This satisfies the P1 historical-generation replay obligation.

---

## 9. Frozen dependency identity

The exact runner-generated `Cargo.lock` was frozen on the branch before the final proof.

```text
Cargo.lock Git blob SHA:
ccf6e1cb9e64e5ff0cf80ce6bdcc92e9a594ad4d

Cargo.lock byte SHA-256:
b9e8452c3d354de5c98e36492c9117fb88aa8b2d234ac3286cf8899d5edd56db
```

The canonical normal runtime closure is explicitly frozen to these 26 package names:

```text
bitflags
block-buffer
cfg-if
cpufeatures
crypto-common
digest
fallible-iterator
fallible-streaming-iterator
foldhash
formula-cli
formula-core
formula-engine
formula-first-light
formula-packages
formula-store
generic-array
hashbrown
hashlink
libsqlite3-sys
num-bigint
num-integer
num-traits
rusqlite
sha2
smallvec
typenum
```

Any later runtime dependency addition is therefore an explicit authority-boundary change rather than an invisible transitive expansion.

---

## 10. TDD and correction evidence

P1 was advanced through explicit RED -> GREEN boundaries rather than inferred from a final test pass.

Observed boundaries included:

- missing canonical/digest modules before Task 1 implementation;
- missing artifact schema module before Task 2 implementation;
- missing generation module before Task 3 implementation;
- missing blob store before Task 4 implementation;
- missing authority store before normal publication implementation;
- missing publication failpoints before rollback implementation;
- replay corruption tests failing before backing verification was added.

Two final proof defects were also resolved without changing authority semantics:

1. pinned `rustfmt` rejected source layout after all behavioral/build gates had passed; exact Rust 1.98.0 formatting was applied;
2. pinned Rust 1.98 Clippy rejected `chunks_exact(2)` in digest parsing; it was replaced with the equivalent fixed-size `as_chunks::<2>()` iteration.

After those corrections the complete canonical proof gate passed.

---

## 11. Canonical final proof sequence

The final proof workflow executed the roadmap/plan-required transcript at source-under-test commit `5218ce5cd35636c080ad569391d48aa62f5d3cc0`:

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
test -z "$(git status --porcelain)"
```

Result: **PASS**. The canonical job `99058414531` concluded successfully.

---

## 12. P1 proof markers

```text
P1-01 deterministic structural identity               PASS
P1-02 structural identity separate from equivalence   PASS
P1-03 immutable content-addressed backing             PASS
P1-04 atomic generation publication                   PASS
P1-05 injected publication failures preserve U0       PASS
P1-06 historical roots replay after fresh reopen      PASS
P1-07 replay rejects manifest/blob corruption         PASS
P1-08 explicit locked runtime dependency closure      PASS
P1-09 P0 architecture/build firewall remains green    PASS
```

These satisfy the roadmap's P1 completion boundary: identity semantics and authority-store transactions are proved in isolation.

---

## 13. Milestone boundary

**P1 is proved on the isolated implementation branch for the source-under-test commit recorded above.**

The authoritative claims stop at structural identity, immutable storage, generation authority transactions, and historical replay.

No parser/evaluator correctness, unit algebra, affine/delta semantics, search correctness, independent certificate checking, First-Light promotion loop, or native realization is implied by this checkpoint.

A post-checkpoint branch proof must remain green with this checkpoint and `CURRENT.md` present before this documentation commit is treated as the final P1 branch candidate.

---

## 14. Next phase

After the post-checkpoint proof remains green, the next roadmap boundary is:

```text
P2 — Dimensions, Units, Affine/Delta Semantics
```

P2 must build on the proved P1 identity/authority substrate rather than weakening or bypassing it.
