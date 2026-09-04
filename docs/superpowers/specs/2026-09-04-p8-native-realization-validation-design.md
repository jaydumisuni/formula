# P8 Native Realization and Validation Design

**Date:** 2026-09-04  
**Status:** implementation design derived from frozen D4 + roadmap P8  
**Branch:** `implementation/p8-native-realization-validation`  
**Exact frozen P7 predecessor:** `e82f7b0535694285baeeb4baae37edc27b6864b8`  
**Authority:** `docs/design/2026-08-28-d4-native-execution-architecture.md`, `docs/roadmap/2026-08-28-implementation-roadmap.md`, frozen P7 checkpoint/current recovery authority.

## 1. Goal

P8 implements the bounded D4 native CPU realization path for the P7-admitted FL-C semantic primitive and independently validates that executable realization without weakening the mathematical-authority/realization-authority separation.

The required bounded flow is:

```text
admitted FL-C semantic construction in U1
 -> deterministic specialization identity
 -> deterministic standalone Rust source
 -> pinned rustc -O native executable
 -> binary/source/toolchain-bound realization manifest
 -> execute all 256 U8 inputs
 -> independent formula-check exhaustive equivalence
 -> opaque checker-issued realization authorization
 -> realization admission/dispatch metadata bound to U1
```

P8 does not implement P9 second-query reuse or `FIRST_LIGHT_COMPLETE`.

## 2. Constitutional boundaries

P8 preserves these laws exactly:

1. Semantic mathematics and executable realization are distinct identities.
2. A compiler/generator may produce candidate code but cannot self-admit it.
3. Realization validation is independent of code generation.
4. The admitted realization binds exact semantic target, universe generation, world, authority contract, observer, source identity, binary identity, specialization identity, and toolchain/backend identity.
5. Mutation of source, executable bytes, or any authority-bearing binding invalidates admission/dispatch.
6. P7 mathematical authority remains valid if a realization is false, missing, stale, or rejected.
7. Canonical P8 execution is local, CPU-only, model-free, network-free during proof, Ptah-free, and GPU-free.
8. P9 reuse/synthesis-skipping behavior is explicitly out of scope.

## 3. Existing evidence to reuse

P8 must reuse rather than replace the existing `formula-check` realization-equivalence path. `check_u8_realization_equivalence` already fails closed on semantic/realization/generation/world/authority/observer binding mismatch, verifies the artifact digest, requires exactly 256 outputs, and checks every U8 value against the semantic `BoolExpr`.

The existing `RealizationCheckManifest` remains the checker request binding for semantic target, realization identity, generation, world, authority contract, observer, and executable artifact digest. P8 may extend realization identity artifacts where D4 requires additional source/toolchain/specialization binding, but must not weaken this checker contract.

The existing `formula-realize` crate is the realization-generation boundary and currently has no production realization implementation beyond its crate-role marker.

## 4. P8 artifacts

### 4.1 `SpecializationIdentity`

A canonical structural identity for the bounded FL-C specialization. It binds:

```text
semantic_target
universe_generation
world
authority_contract
observer
query_direction = "u8_to_bool_forward"
input_domain = "u8:0..=255"
output_domain = "bool"
lowering_class = "EXACT_EQUIVALENCE"
```

No path, timestamp, hostname, benchmark, or temporary build directory enters structural identity.

### 4.2 `NativeSourceArtifact`

A deterministic source artifact generated only from the admitted FL-C semantic construction plus `SpecializationIdentity`.

For P8 the backend is standalone Rust source. The generated program accepts one decimal U8 argument and prints exactly `0` or `1` followed by newline. Invalid arity or invalid U8 input exits non-zero and cannot be interpreted as a semantic result.

The source digest is `ArtifactDigest::of_bytes(source.as_bytes())`.

### 4.3 `NativeToolchainIdentity`

A canonical identity binding the exact compiler/backend contract used for canonical P8:

```text
compiler = "rustc"
rust_release = "1.98.0"
optimization = "-O"
target = canonical runner host target
backend_family = "standalone-rust-native"
```

The canonical workflow proves the actual `rustc --version --verbose` evidence and host target against the manifest inputs before validation/admission.

### 4.4 `NativeRealizationManifest`

A canonical D4 realization manifest binding at minimum:

```text
semantic_target
universe_generation
world
authority_contract
observer
specialization_digest
source_digest
toolchain_digest
binary_digest
lowering_class = "EXACT_EQUIVALENCE"
input_representation = "u8"
output_representation = "bool"
fallback_semantics = semantic execution
```

Measured compile/runtime performance is metadata only and is excluded from structural identity.

## 5. Generator boundary

`formula-realize` owns deterministic source generation and compiler invocation description. It does not own realization admission.

The bounded FL-C generator consumes the admitted semantic `BoolExpr` and a `SpecializationIdentity`, verifies that the specialization semantic target is the expression identity supplied by the caller, and emits deterministic standalone Rust.

The generated source must not read sealed First-Light target fixtures, network resources, environment secrets, authority-store state, or hidden expected outputs.

The canonical build helper compiles with the pinned P8 toolchain and exact optimization policy. Build-directory location is operational state only; only source/toolchain/binary digests become realization authority inputs.

## 6. Independent checker authorization

`formula-check` remains the authority-deciding component for realization correctness.

P8 introduces an opaque `RealizationAuthorization` issued only after:

1. the proposed `NativeRealizationManifest` bindings match the checker request;
2. executable bytes hash to the bound binary digest;
3. actual execution yields exactly 256 outputs for inputs `0..=255`;
4. inherited `check_u8_realization_equivalence` returns `Pass` against the admitted FL-C semantic expression;
5. source/toolchain/specialization bindings match the proposed realization manifest.

`RealizationAuthorization` has private authority-bearing fields and no public constructor. Generator/compiler/store callers cannot manufacture one.

A failed check returns a typed rejection and no authorization.

## 7. Admission and dispatch boundary

P8 adds the minimum realization registry/admission surface required by D4. Admission consumes `RealizationAuthorization`, not raw generator output.

The admitted entry is generation-scoped and binds the exact realization manifest digest. Runtime selection/dispatch may return only a realization whose semantic target, generation, world, authority contract, observer, architecture/toolchain compatibility, and artifact digest match the requested context.

If the binary is absent or its bytes no longer match the admitted digest, dispatch fails closed and the caller must use semantic fallback. It must never silently execute mutated bytes.

No realization admission mutates or supersedes the underlying P7 mathematical admission.

## 8. Negative controls

P8 must mechanically prove at least these failures:

```text
changed semantic target -> rejected
changed specialization binding -> rejected
changed source digest -> rejected
changed toolchain identity -> rejected
changed binary byte -> rejected
255/257/missing outputs -> rejected
one wrong U8 output -> exact counterexample rejection
compiler/generator direct admission attempt -> architecture/API rejection
wrong U1 generation/world/authority/observer -> rejected
missing admitted binary at dispatch -> semantic fallback/fail-closed, never guessed result
```

Architecture tests must preserve the authority firewall: `formula-realize` cannot gain authority-writing capability merely because it generates native code.

## 9. Proof markers

The canonical P8 workflow must derive, not blindly print, these markers after their corresponding gates pass:

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

These are P8 markers only. `PASS D5_SECOND_QUERY_REUSE` and `PASS FIRST_LIGHT_COMPLETE` remain P9-only.

## 10. Canonical workflow gate

The P8 canonical workflow is read-only (`permissions: contents: read`) on Ubuntu 24.04 with Rust 1.98.0. It must use locked/offline Cargo execution where applicable after dependency priming and must prove a clean worktree.

It reruns predecessor architecture/workspace gates plus P8-specific realization tests, exhaustive integration, formatting, Clippy with warnings denied, dependency firewalls, and exact proof markers.

P8 is frozen only when the exact documentation-bearing P8 branch head passes this unchanged canonical workflow.

## 11. Scope freeze

Included:

```text
bounded FL-C U8->Bool specialization
standalone native Rust source generation
pinned rustc -O build identity
source/binary/toolchain/specialization realization identity
independent 256-input validation
opaque checker-issued realization authorization
minimal admitted realization registry/dispatch identity gate
mutation and authority-firewall negative controls
canonical P8 proof
```

Excluded:

```text
P9 second-query reuse
proof synthesis was skipped on reuse
FIRST_LIGHT_COMPLETE
GPU/SIMD/autotuning
external native libraries
Ptah/distributed execution
JIT
multiple backend selection policy
benchmark-driven planner
P10 realization-upgrade/supersession generalization
```
