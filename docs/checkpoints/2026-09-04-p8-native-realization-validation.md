# P8 Checkpoint — Native Realization + Validation

**Recorded:** 2026-09-04  
**Status:** PROVED SOURCE — D4 native CPU realization/validation; documentation-bearing head still requires exact-head canonical proof  
**Branch:** `implementation/p8-native-realization-validation`  
**P7 exact frozen predecessor:** `e82f7b0535694285baeeb4baae37edc27b6864b8`  
**P7 final canonical proof run:** `33862079731`  
**P7 final canonical proof job:** `100988579155`  
**Source-under-test commit:** `3ca395e13de7dbbc611347f37b8cbaf3875d4236`  
**Canonical source proof run:** `33898667278`  
**Canonical source proof job:** `101107293202`  
**Roadmap authority:** `docs/roadmap/2026-08-28-implementation-roadmap.md`, phase P8  
**Design authority:** `docs/superpowers/specs/2026-09-04-p8-native-realization-validation-design.md`  
**Implementation plan:** `docs/superpowers/plans/2026-09-04-p8-native-realization-validation.md`

---

## 1. Scope

P8 implements the bounded D4 realization path from the P7-admitted FL-C semantic primitive to a native CPU artifact whose realization correctness is independently checked before the artifact becomes dispatchable.

P8 preserves the constitutional split:

```text
mathematical authority
  P7 admitted semantic primitive in U1

realization production
  admitted semantic primitive -> deterministic Rust source -> rustc 1.98.0 -O -> native binary

realization authority
  independent checker -> opaque RealizationAuthorization -> AuthorityStore::admit_realization

execution
  exact RealizationDispatchContext -> admitted immutable binary
```

The compiler/generator is an untrusted producer. It cannot certify itself, cannot admit itself, and cannot mutate mathematical authority.

P8 source surfaces include:

- canonical `SpecializationIdentity`;
- canonical `NativeToolchainIdentity`;
- canonical `NativeRealizationManifest`;
- canonical `RealizationDispatchContext`;
- deterministic standalone Rust lowering for the bounded U8/Boolean AST;
- checker-owned opaque `RealizationAuthorization`;
- exact source/toolchain/binary/specialization/check-manifest binding;
- inherited exhaustive 256-input semantic equivalence as the decision point;
- durable realization registry in the existing authority store;
- active-generation and binary-content-address admission checks;
- exact dispatch-context resolution;
- tampered/missing admitted binary rejection through the existing verified blob store;
- architecture firewall preventing realization generation from reaching checker/store authority;
- real optimized FL-C native compilation and CPU execution over all 256 U8 inputs;
- replay proof that realization admission leaves the already-admitted U0/U1 mathematical generations unchanged.

P8 does **not** implement second-query reuse, synthesis skipping, the P9 completion manifest, `FIRST_LIGHT_COMPLETE`, GPU/SIMD/JIT backends, Ptah/distributed execution, or broader native backend families.

---

## 2. Exact predecessor and review boundary

P8 was cut from the exact finally frozen P7 documentation-bearing head:

```text
e82f7b0535694285baeeb4baae37edc27b6864b8
workflow: P7 canonical proof
run:      33862079731
job:      100988579155
result:   success
```

The proved P8 source boundary is:

```text
3ca395e13de7dbbc611347f37b8cbaf3875d4236
```

Exact P7 -> P8 compare evidence:

```text
base:    e82f7b0535694285baeeb4baae37edc27b6864b8
head:    3ca395e13de7dbbc611347f37b8cbaf3875d4236
status:  ahead
ahead:   37 commits
behind:  0 commits
files:   20
```

The reviewed source delta is exactly:

```text
.github/workflows/p8-canonical-proof.yml
Cargo.lock
crates/formula-check/src/realization.rs
crates/formula-check/src/verdict.rs
crates/formula-check/tests/p8_realization_authorization.rs
crates/formula-core/src/lib.rs
crates/formula-core/src/realization.rs
crates/formula-core/tests/p8_realization_identity.rs
crates/formula-first-light/Cargo.toml
crates/formula-first-light/tests/p8_fl_c_native_realization.rs
crates/formula-realize/Cargo.toml
crates/formula-realize/src/lib.rs
crates/formula-realize/src/rust_native.rs
crates/formula-realize/tests/p8_rust_generation.rs
crates/formula-store/src/authority_store.rs
crates/formula-store/src/authority_store/realization_store.rs
crates/formula-store/tests/p8_realization_store.rs
docs/superpowers/plans/2026-09-04-p8-native-realization-validation.md
docs/superpowers/specs/2026-09-04-p8-native-realization-validation-design.md
tests/authority-boundary/tests/p8_realization_authority.rs
```

Temporary P8 development, lockfile-repair, store-patch, and dependency-patch helpers were removed before canonical source proof and are absent from this reviewed delta.

---

## 3. Canonical source proof

Exact source proof:

```text
workflow: P8 canonical proof
run:      33898667278
job:      101107293202
head:     3ca395e13de7dbbc611347f37b8cbaf3875d4236
result:   success
runner:   ubuntu-24.04
rustc:    1.98.0 (88d9e12ae 2026-08-18)
host:     x86_64-unknown-linux-gnu
cargo:    1.98.0
```

The canonical workflow is read-only (`permissions: contents: read`). It primes dependencies once, then uses locked/offline proof execution where applicable.

The exact source head passed:

```text
identity/toolchain
locked offline metadata
P8 realization identity tests
deterministic Rust generator tests
independent realization checker tests
legacy realization-equivalence regression tests
authorized realization store/dispatch tests
P8 architecture firewall + all predecessor architecture tests
real FL-C native CPU exhaustive proof
formula-core all-targets
formula-check all-targets
formula-store all-targets
formula-realize all-targets
formula-packages all-targets
formula-engine all-targets
formula-first-light all-targets
workspace all-targets tests
workspace all-targets build
cargo fmt --all -- --check
workspace clippy --all-targets -D warnings
normal dependency trees
authority dependency firewall
clean worktree
```

---

## 4. Identity and specialization law

`SpecializationIdentity` binds the exact admitted semantic target to the exact universe generation, world, authority contract, and observer under the frozen `EXACT_EQUIVALENCE` lowering class.

`NativeToolchainIdentity` binds the canonical compiler family/release/host target/optimization contract used by P8.

`NativeRealizationManifest` separately binds:

```text
semantic target
universe generation
world
authority contract
observer
specialization identity
source digest
toolchain identity
binary digest
EXACT_EQUIVALENCE lowering class
```

Changing those bindings changes the realization identity and cannot silently alias an already-authorized artifact.

---

## 5. Generator is production, not authority

`formula-realize` deterministically lowers only the bounded engine U8/Boolean AST into standalone Rust source.

The generated source has no First-Light fixture dependency, environment-variable answer channel, network dependency, checker dependency, or store dependency.

Production dependency firewall proof establishes:

```text
formula-realize -> formula-core + formula-engine
formula-realize -/-> formula-check
formula-realize -/-> formula-store
formula-realize -/-> formula-first-light

formula-engine -/-> formula-check
formula-engine -/-> formula-store
formula-engine -/-> formula-first-light
```

The compiler can produce bytes; compiler success is not authority.

---

## 6. Independent checker authorization law

The checker owns the only constructor path for `RealizationAuthorization`. Its authority-bearing fields are private and there is no public constructor.

Authorization validates the exact native/check manifests and requires:

```text
semantic target binding
specialization binding
generation/world/authority/observer binding
source digest binding
toolchain identity binding
binary digest binding
exact 256-output coverage
exhaustive equality with the independently represented semantic expression
```

The existing exhaustive checker remains the semantic decision point. The generator and compiler do not certify their own output.

---

## 7. Realization store and dispatch law

The durable P8 realization registry is a private child of the existing `AuthorityStore`; it does not create a second authority database or publication root.

Public admission is:

```text
checker-issued RealizationAuthorization
 -> active-generation equality check
 -> binary digest recomputation
 -> existing immutable content-addressed BlobStore
 -> realization registry row
```

The store consumes checker authorization. It does not re-decide mathematical correctness from a manifest or compiler claim.

Dispatch is exact over:

```text
semantic target
generation
world
authority contract
observer
```

A context mismatch resolves to no realization. A missing or tampered binary fails closed because blob reads recompute the expected content digest.

---

## 8. Real FL-C native CPU proof

The P8 First-Light integration test reuses the P7 authority path rather than bypassing it:

```text
bounded public FL-C discovery
 -> sealed exact counterexample/equivalence loop
 -> final semantic expression
 -> P7 checker authorization
 -> P7 atomic promotion U0 -> U1
 -> specialization of the U1-admitted primitive
 -> deterministic Rust source
 -> rustc 1.98.0 -O
 -> native binary
 -> execute binary for every U8 input 0..=255
 -> independent checker authorization
 -> realization admission
 -> exact dispatch
```

Before checker authorization/admission, exact dispatch returns no artifact, proving that compilation itself cannot make the binary authoritative.

The compiled program was executed as a real local CPU process for all 256 U8 inputs. Only canonical `0\n`/`1\n` outputs were accepted, stderr had to remain empty, and the independent checker compared all 256 realized outputs against an independently translated checker semantic AST.

The targeted canonical FL-C native test passed in the source proof. The whole First-Light and workspace sweeps repeated that path successfully.

`formula-realize` is linked into `formula-first-light` only as a **dev-dependency** for this proof. `formula-first-light` production dependencies remain `formula-core` + `formula-engine`.

---

## 9. Negative controls

P8 fails closed for the required bounded adversarial cases:

```text
changed semantic target -> generation fails before valid source authority
changed specialization -> checker rejection
changed toolchain identity -> checker rejection
changed checker/native manifest binding -> checker rejection
changed generated source bytes -> RealizationSourceDigestMismatch
changed compiled binary bytes -> RealizationArtifactDigestMismatch
short realized output set -> RealizationOutputCoverageMismatch
one wrong realized output -> exact RealizationCounterexample(input)
compiler output without checker authorization -> no admission/dispatch
changed active generation at admission -> RealizationGenerationMismatch
changed binary at admission -> RealizationBinaryDigestMismatch
wrong dispatch context -> no realization
on-disk admitted binary tampering -> verified blob read failure
```

No negative control invalidates the already-admitted P7 mathematics.

---

## 10. Mathematical authority remains unchanged

P8 does not create `U2` and does not rewrite `U1` to store realization state.

The FL-C proof records U0 and U1 canonical bytes/digests before realization admission and replays them afterward:

```text
U0 digest unchanged
U0 canonical bytes unchanged
U1 digest unchanged
U1 canonical bytes unchanged
```

Therefore a false or corrupted realization is a realization-authority failure only; it cannot retroactively revoke or mutate admitted mathematical truth.

---

## 11. P8 canonical proof markers

Canonical source run `33898667278` emitted all required markers only after the corresponding gates succeeded:

```text
PASS P8_SEMANTIC_BINDING
PASS P8_SPECIALIZATION_IDENTITY
PASS P8_EXACT_LOWERING_CLASS
PASS P8_SOURCE_TOOLCHAIN_BINARY_BOUND
PASS P8_COMPILER_CANNOT_SELF_ADMIT
PASS P8_ALL_256_INPUTS_EQUIVALENT
PASS P8_MUTATED_SOURCE_REJECTED
PASS P8_MUTATED_BINARY_REJECTED
PASS P8_DISPATCH_IDENTITY_ENFORCED
PASS P8_CPU_LOCAL_OFFLINE
PASS P8_P7_MATH_AUTHORITY_PRESERVED
```

---

## 12. P0–P7 remain authority

P8 extends rather than replaces predecessor proofs:

```text
P0 reproducible repository/build + architecture firewall
P1 deterministic identity + immutable atomic authority store
P2 independent checker/certificate authority
P3 theory packages + generation/world-scoped capability closure
P4 deterministic query compiler + campaign planning
P5 bounded exact CandidateSpace/discovery
P6 sealed First-Light target harness + blindness gates
P7 checker-authorized atomic promotion into immutable U1
```

The final source workflow reruns predecessor crate, architecture, generation, promotion, First-Light, workspace, formatting, lint, and dependency-firewall gates.

---

## 13. Not proved by P8

Do not claim from P8:

```text
second-query semantic reuse under U1
proof that synthesis/search is skipped on reuse
P9 reusable dispatch policy
complete First-Light proof manifest
FIRST_LIGHT_COMPLETE
GPU realization
SIMD realization
JIT realization
Ptah/distributed execution
external backend federation
P9 completion
P10 completion
```

P8 proves one bounded native CPU realization path and its authority separation. It does not prove the next-query reuse behavior.

---

## 14. Next implementation boundary

The roadmap phase after a finally frozen P8 is **P9 — reuse / complete First-Light proof**.

Do **not** start P9 until this checkpoint plus `CURRENT.md` form the documentation-bearing P8 branch candidate and the **unchanged** P8 canonical workflow succeeds on that exact documentation-bearing head.

---

## 15. Freeze state

P8 source is proved and scope/review-clean on:

```text
3ca395e13de7dbbc611347f37b8cbaf3875d4236
workflow: P8 canonical proof
run:      33898667278
job:      101107293202
result:   success
```

This checkpoint plus the accompanying `CURRENT.md` update form the documentation-bearing branch candidate. **P8 is not finally frozen until the unchanged P8 canonical workflow succeeds on that exact documentation-bearing head.**

The P8 branch has not been merged to `main`. P9 has not started.
