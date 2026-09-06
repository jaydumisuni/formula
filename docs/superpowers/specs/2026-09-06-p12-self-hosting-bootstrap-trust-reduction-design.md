# P12 Self-Hosting Bootstrap / Trust Reduction Design

**Status:** FROZEN FOR IMPLEMENTATION

**Predecessor:** exact finally frozen P11 proof head `6f8ce7bb6702ea1baf119aab9950aa5ba0f87283`

**Mandatory research:** `docs/research/2026-09-06-p12-rs-bootstrap-seed.md`

## Goal

Reduce the bootstrap trust surface by rebuilding an authority-critical Formula checker through a Formula-owned deterministic Bootstrap Core and admitting its successor only after a distinct checker-owned validator independently recompiles and behavior-validates it.

P12 does **not** self-host Rust. Rust 1.98.0 remains an explicit B0 seed whose executable hashes and provenance are first-class inputs.

## Constitutional laws

1. `successor generator != successor validator`.
2. A successor cannot authorize itself.
3. Successful generation, execution, reproducibility or self-rebuild does not create authority.
4. Bootstrap/trust generation `T_g` is distinct from mathematical universe generation `U_g`.
5. A bootstrap transition must not mutate or promote mathematical authority.
6. Every seed and rebuild artifact is content-addressed and provenance-bound.
7. Failed equivalence or failed provenance is terminal for that candidate successor.
8. The prior admitted bootstrap generation remains replayable and selectable after a failed or superseded candidate.
9. Canonical P12 authority is earned only by the independent validator and bootstrap store admission path.
10. P13 cluster/remoting is out of scope.

## Architecture

### 1. Core bootstrap identities — `formula-core::bootstrap`

Add immutable canonical types:

```text
BootstrapGenerationId
BootstrapRole
BootstrapSeedManifest
BootstrapInstruction
BootstrapProgramSource
BootstrapBytecode
BootstrapGeneratorImage
BootstrapEquivalenceLevel
BootstrapValidationState
BootstrapRebuildManifest
BootstrapNegativeControl / evidence / manifest
BootstrapProofManifest
```

`BootstrapGenerationId` is a trust-chain identity and is never interchangeable with `UniverseGeneration`.

### 2. B0 explicit seed

The canonical P12 workflow computes SHA-256 of the actual `rustc` and `cargo` executables and constructs `BootstrapSeedManifest` binding:

```text
role = EXTERNAL_TOOLCHAIN_SEED
rust version = 1.98.0
rustc commit = 88d9e12ae178fab0fb5cc050a94da85685d449ea
cargo version = 1.98.0 (797e8a9bc 2026-08-05)
host = x86_64-unknown-linux-gnu
rustc executable digest
cargo executable digest
rust-toolchain.toml digest
reproducible setup descriptor
license/provenance descriptor
```

The seed manifest is evidence about B0. It cannot issue bootstrap validation or mathematical authority.

### 3. Tiny Formula Bootstrap Core

P12 freezes a deliberately tiny source language sufficient for the first authority-critical checker:

```text
LOAD_ACTUAL_DIGEST
LOAD_EXPECTED_DIGEST
DIGEST_EQ
RETURN_DECISION
```

Canonical identity-checker source is exactly the four-instruction sequence above.

Canonical bytecode v1:

```text
header: 46 42 43 31        # "FBC1"
01 LOAD_ACTUAL_DIGEST
02 LOAD_EXPECTED_DIGEST
03 DIGEST_EQ
04 RETURN_DECISION
```

No extension opcode is accepted in P12. Unknown/truncated/noncanonical bytecode fails closed.

### 4. Formula-owned generator — `formula-realize::bootstrap`

The generator is an execution component, not authority.

It exposes:

```rust
compile_bootstrap_source(source: &BootstrapProgramSource) -> BootstrapBytecode
rebuild_with_generator_image(
    image: &BootstrapGeneratorImage,
    source: &BootstrapProgramSource,
) -> Result<BootstrapBytecode, BootstrapGenerationError>
```

`BootstrapGeneratorImage` contains the canonical opcode mapping and encoding schema. `rebuild_with_generator_image` derives output from the supplied image rather than calling the direct compiler mapping.

Stage0 uses the B0-built Rust implementation to create the first generator image. After independent validation, Stage1 uses the admitted generator image to rebuild the identity checker. Stage2 repeats the rebuild through the admitted Stage1 image and must converge byte-for-byte.

### 5. Independent validator — `formula-check::bootstrap`

The validator must not import or call `formula-realize`.

It implements separately:

```rust
reference_compile(source: &BootstrapProgramSource) -> BootstrapBytecode
reference_execute(
    source: &BootstrapProgramSource,
    actual: ArtifactDigest,
    expected: ArtifactDigest,
) -> BootstrapDecision
validate_bootstrap_candidate(
    rebuild: &BootstrapRebuildManifest,
    source: &BootstrapProgramSource,
    candidate: &BootstrapBytecode,
    seed: &BootstrapSeedManifest,
) -> Result<BootstrapValidationAuthorization, BootstrapValidationFailure>
```

Validation requires:

- generator identity differs from validator identity;
- exact seed provenance binding;
- exact source/build-recipe binding;
- candidate bytecode equals independent reference compilation;
- candidate interpreter behavior equals source semantics;
- complete bounded identity-checker truth table:
  - actual == expected -> VALID;
  - actual != expected -> REJECT;
- declared equivalence level is satisfied;
- complete negative-control evidence is present.

The opaque `BootstrapValidationAuthorization` is the only token the bootstrap store may consume to advance `T_g`.

### 6. Candidate bytecode execution

The generator/runtime path may execute `BootstrapBytecode`, but execution remains authority-inert.

The interpreter accepts exactly two digest inputs and returns:

```text
VALID
REJECT
```

Execution errors, malformed bytecode or unsupported opcodes never become VALID.

### 7. Separate bootstrap authority store — `formula-store::bootstrap_store`

Add a trust-generation ledger separate from Universe-generation storage.

Required operations:

```rust
create_bootstrap_root(seed_manifest) -> T0
admit_bootstrap_successor(authorization, candidate) -> T_(g+1)
active_bootstrap_generation() -> T_g
replay_bootstrap_generation(T_g)
select_bootstrap_generation(T_g)
```

Admission is append-only. Selecting/rolling back a bootstrap generation changes only the active `T_g` pointer.

No bootstrap-store method may call mathematical promotion or alter the active `UniverseGeneration`.

### 8. Stage progression

#### T0 / Stage0

- explicit Rust seed manifest;
- Rust-built Bootstrap Core generator image;
- independent validator identity already distinct.

#### T1 / Stage1

- generator image compiles canonical identity-checker Bootstrap Core source;
- independent validator recompiles it separately;
- byte-for-byte and semantic equivalence pass;
- authorization admits T1.

#### T2 / Stage2

- admitted T1 generator image rebuilds the same checker;
- independent validator validates T2 again;
- T1/T2 normalized bytecode and behavior converge;
- T2 is admitted only after validation.

P12 gate requires T2 success.

### 9. Bootstrap equivalence

Supported P12 equivalence levels:

```text
SOURCE_SEMANTIC
NORMALIZED_ARTIFACT
BYTE_FOR_BYTE
```

Canonical P12 requires both:

```text
BYTE_FOR_BYTE
AND exhaustive bounded semantic equivalence
```

Byte equality never substitutes for semantic validation.

### 10. Negative controls

Implement the D1A trust-reduction controls as executable failures:

```text
NC-BS-01 validator identity equals generator identity
NC-BS-02 single generator path attempts admission without independent validation
NC-BS-03 unexpected candidate/reference byte difference
NC-BS-04 seed provenance mismatch
NC-BS-05 source digest mismatch
NC-BS-06 build-recipe digest mismatch
NC-BS-07 normalization/equivalence claim masks semantic difference
NC-BS-08 malformed or unsupported bytecode
NC-BS-09 failed equivalence attempts T-generation promotion
NC-BS-10 bootstrap transition attempts to mutate mathematical U-generation
```

The canonical negative-control manifest is complete only when every concrete rejection path has executed.

### 11. Canonical proof manifest

`BootstrapProofManifest` binds:

```text
source commit
frozen P11 predecessor identity
B0 seed manifest
T0/T1/T2 identities
identity-checker source digest
Stage1 generator identity
Stage1 validator identity
Stage1 rebuild manifest
Stage1 candidate artifact
Stage2 generator identity
Stage2 validator identity
Stage2 rebuild manifest
Stage2 candidate artifact
equivalence/semantic evidence
negative-control manifest
active mathematical U-generation before/after bootstrap proof
checker/verifier identities
```

Independent final replay in `formula-check` emits P12 PASS markers only after exact replay succeeds.

### 12. Canonical workflow

Permanent `.github/workflows/p12-canonical-proof.yml` must:

- use `contents: read`;
- checkout exact SHA;
- pin Rust 1.98.0;
- hash actual `rustc`, `cargo`, and `rust-toolchain.toml`;
- export those digests to the canonical P12 harness;
- prime locked dependencies, then run proof gates offline;
- run P12 canonical bootstrap proof and independent verifier;
- rerun frozen P11 predecessor proof;
- run every crate/workspace regression;
- run rustfmt, Clippy `-D warnings`, dependency/source firewalls and clean-tree;
- contain no write-back step.

### 13. Freeze discipline

P12 follows the same source-proof -> recovery-docs -> unchanged docs-head-proof discipline as P9-P11.

The exact docs-bearing successful SHA/run/job becomes the frozen P12 boundary. Later recovery-only commits may describe it without recursively redefining it.

## Acceptance gate

P12 passes only when all of these are true:

1. B0 seed/toolchain is explicitly pinned and executable-hashed.
2. A Formula-owned Bootstrap Core path rebuilds the selected authority-critical identity checker.
3. A distinct checker-owned path independently recompiles and evaluates the successor.
4. Generator identity differs from validator identity.
5. Stage1 is admitted only after independent validation.
6. Stage2 is rebuilt through the admitted Stage1 path and independently validated again.
7. Stage1/Stage2 converge byte-for-byte and semantically.
8. Failed equivalence cannot advance `T_g`.
9. `U_g` is unchanged throughout the bootstrap transition proof.
10. All NC-BS-01...NC-BS-10 execute and fail closed.
11. The canonical source proof and unchanged documentation-head proof both succeed.

Only then may P12 claim `BOOTSTRAP_TRUST_REDUCED`.

## Explicit exclusions

P12 does not claim:

- a seed-free system;
- a verified Rust compiler;
- removal of CPU/firmware/hardware trust;
- full Formula self-hosting;
- authority from reproducibility alone;
- mathematical authority from bootstrap admission;
- P13 remoting/cluster execution.
