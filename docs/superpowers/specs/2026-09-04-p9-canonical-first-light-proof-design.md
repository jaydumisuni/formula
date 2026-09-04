# P9 Canonical First-Light Proof Design

**Date:** 2026-09-04  
**Status:** implementation design derived from frozen First-Light specification + roadmap P9  
**Branch:** `implementation/p9-canonical-first-light-proof`  
**Exact verified P8 recovery predecessor:** `02af51ded5cbc8732017b57300f79e7fbe8cc80c`  
**Frozen P8 proof head:** `fa369b6241c0c069176e5939acf4d5ec74eb8085`  
**Authority:** `docs/design/2026-08-28-first-light-specification.md`, `docs/design/2026-08-28-d5-self-expansion-architecture.md`, `docs/roadmap/2026-08-28-implementation-roadmap.md`, frozen P8 checkpoint/current recovery authority.

## 1. Goal

P9 closes the canonical First-Light loop from clean local state and produces one independently replayable proof manifest.

The defining new P9 obligation is not another FL-C synthesis run. It is the second-query reuse proof under U1:

```text
U0 lacks FL-C primitive
 -> blind FL-C discovery/certification
 -> promotion -> immutable U1
 -> activation-derived capability closure includes promoted primitive
 -> admitted native realization exists
 -> second distinct query under U1
 -> compiler resolves promoted capability
 -> zero primitive-discovery CandidateSpaces
 -> zero primitive-discovery Work Cells
 -> exact admitted realization dispatch
 -> exact second-query result
 -> canonical proof manifest
 -> independent replay/verifier
 -> all frozen First-Light PASS markers
```

P9 is the first major implementation freeze. It does not implement P10 generalized self-expansion hardening.

## 2. Constitutional boundaries

P9 preserves these laws exactly:

1. Search may propose mathematics; only Certification + Promotion creates mathematical authority.
2. Activation/capability closure is derived from admitted authority state; callers may not inject a fake reusable primitive directly into compiler authority.
3. Reuse consumes an already-admitted primitive and realization; it must not silently rediscover, recompile, revalidate, or repromote that primitive.
4. Mathematical authority, capability activation, and realization authority remain distinct identities.
5. The second query is different from the original synthesis request while requiring the same promoted semantics.
6. A valid reuse campaign has no original FL-C synthesis CandidateSpace and launches no primitive-discovery Work Cell.
7. A missing/inactive/wrong-context primitive fails reuse closed instead of falling back to an unrecorded synthesis path.
8. A missing/tampered/wrong-context native realization fails dispatch closed; no unverified executable becomes authoritative.
9. The final First-Light PASS markers are issued only by an independent manifest verifier/replay path, never by search/discovery code.
10. Canonical First Light remains local, CPU-only, model-free, network-free during proof, Ptah-free, and GPU-free.
11. P10 promotion-class generalization, metaprimitive rollout, grammar evolution framework, and broad self-expansion hardening are out of scope.

## 3. Existing evidence to reuse

P9 reuses P0-P8 authority rather than recreating it.

Existing `CompilerV1::compile` produces the ordinary D3 campaign/work-cell path and remains unchanged for discovery behavior. Existing `RelevantRegion` already carries generation-scoped admitted capabilities from a `CompilerAuthoritySnapshot`.

Existing P7 promotion proves the FL-C semantic primitive is admitted into immutable U1, and existing `PromotionRecord` distinguishes `CERTIFIED`, `ADMITTED`, and `ACTIVATED` states.

Existing P8 realization proof already supplies:

```text
SpecializationIdentity
NativeToolchainIdentity
NativeRealizationManifest
opaque RealizationAuthorization
AuthorityStore::admit_realization
AuthorityStore::resolve_realization
exact RealizationDispatchContext
```

P9 consumes these contracts unchanged for native dispatch.

The existing capability-closure engine derives package capabilities from admitted witnesses. P9 extends closure minimally so an explicitly activated semantic primitive admitted in the same generation can become a reusable capability without caller-side manual injection.

## 4. Activated semantic capability derivation

P9 introduces the smallest generation-scoped activation derivation needed by First Light.

An `ActivatedSemanticCapability` is valid only when all of the following agree:

```text
activation record state == ACTIVATED
activation record generation == current U1 digest
activation record semantic_artifacts contains primitive digest
current U1 admitted set contains primitive digest
current world/authority scope matches the reuse context
```

The derived capability identity is the promoted primitive digest itself for this bounded First-Light primitive. Capability derivation does not create a new mathematical artifact and does not mutate U1.

The closure output must make the transition measurable:

```text
capability(U0) does not contain FL-C primitive
capability(U1) contains FL-C primitive
```

Removing/invalidating activation must remove automatic reuse eligibility without deleting the admitted mathematical primitive.

## 5. Canonical second query

P9 uses the frozen First-Light second-query family without changing QueryIR schema v1.

Canonical request:

```text
Given a canonical vector/list of U8 values,
count the values satisfying the promoted IsPowerOfTwoU8 semantics.
```

The `QueryIR` is bound to:

```text
U1 generation
same First-Light world
authority contract
observer
activated package context
known binding for the canonical input vector
requested result class = COUNT
```

The input vector is canonical/content-addressed. The query is not the original synthesis request and contains no hidden expression/program for IsPowerOfTwoU8.

## 6. Reuse resolution and compiler path

P9 adds an explicit reuse compilation path beside ordinary discovery compilation.

### 6.1 `ReuseRequest`

Canonical identity binding:

```text
query_digest
required_semantic_capability
universe_generation
world
authority_contract
observer
result_class
```

### 6.2 `ResolvedCapability`

Produced only when the exact required primitive appears in the exact `RelevantRegion` admitted capability set for the same generation/world/query authority context.

It binds:

```text
reuse_request_digest
primitive_digest
generation
world
authority_contract
observer
```

### 6.3 `CompilerV1::compile_reuse`

This path:

1. validates query/snapshot generation/world/package/observer/authority like normal compilation;
2. constructs the same authoritative `RelevantRegion` and `TheoryProfile` inputs;
3. resolves the required promoted capability from that region;
4. creates a reuse campaign referencing the U1 primitive identity;
5. creates an execution work item whose semantic input is the resolved primitive;
6. records reuse metrics proving no primitive-discovery work occurred.

It must not instantiate `ObservationalExprSpace`, `CandidateSpaceContext`, FL-C grammar, CEGIS refinement, or any primitive-discovery Work Cell.

Ordinary `CompilerV1::compile` remains the existing discovery path and is not silently changed into reuse behavior.

## 7. Reuse metrics

P9 introduces canonical comparative metrics sufficient to prove disappearance of primitive rediscovery rather than a timing claim.

`ReuseMetrics` binds at minimum:

```text
primitive_discovery_candidate_spaces
primitive_discovery_work_cells
resolved_capability_count
execution_work_items
```

For the canonical U1 reuse query:

```text
primitive_discovery_candidate_spaces = 0
primitive_discovery_work_cells = 0
resolved_capability_count = 1
execution_work_items = 1
```

The original U0 FL-C discovery evidence remains separately bound in the final manifest, so the verifier can establish that discovery existed before promotion but disappears on the second query.

Wall-clock speed is not proof of reuse and is excluded from structural identity.

## 8. Native realization reuse

The reuse executor constructs the exact P8 `RealizationDispatchContext` from the resolved primitive plus U1/world/authority/observer.

It calls the existing `AuthorityStore::resolve_realization` and executes only the returned verified immutable binary bytes/artifact.

P9 does not regenerate Rust source, invoke rustc, issue another realization authorization, or admit another realization for the second query.

The second-query result is obtained by evaluating each canonical input through the admitted realization and counting true outputs. The exact result is independently checked against admitted semantic evaluation.

If no matching admitted realization exists, reuse fails closed for canonical P9. Semantic fallback remains a D4 system concept but cannot be used to claim `D5_SECOND_QUERY_REUSE` for this canonical proof.

## 9. Canonical First-Light proof manifest

P9 introduces `FirstLightProofManifest`, a canonical content-addressed artifact binding all evidence required by the frozen specification:

```text
source_commit
U0_digest
U1_digest
world_digests
activated_package_set
FL-A query/campaign/candidate/evidence digests
FL-B query/campaign/reduction/evidence digests
FL-C query/campaign/grammar/candidate/evidence digests
mandatory FL-C near-miss rejection evidence
promotion transaction/record digests
capability closure before/after/delta digests
native source/toolchain/binary/realization evidence digests
second-query QueryIR/campaign/resolved-capability/result digests
reuse metrics digest
negative-control manifest digest
checker/verifier identity
```

Non-semantic timestamps, runner paths, and benchmark timings are excluded from structural identity.

The manifest must be complete: omitted required sections fail verification rather than producing a partial First-Light marker set.

## 10. Negative-control manifest

P9 binds the frozen negative-control suite into one canonical manifest. It must include evidence for at least:

```text
NC-01 modified sealed target digest -> manifest verification fails
NC-02 discovery-to-sealed import -> blindness/authority boundary fails
NC-03 FL-A sample-fitting near-miss -> rejected
NC-04 FL-B corrupted Boolean/GF2 translation -> rejected
NC-05 FL-C zero-accepting near-miss -> rejected
NC-06 forged/mismatched Evidence digest -> promotion fails
NC-07 changed candidate after certificate -> promotion fails
NC-08 search authority-write attempt -> denied
NC-09 changed binary after realization proof -> dispatch rejects
NC-10 U1 activation removed -> reuse cannot claim promoted capability
NC-11 stricter Authority Contract without new evidence -> rejected
NC-12 parent-generation race during promotion -> aborts
```

Existing predecessor tests may supply the evidence where already proved; P9 does not duplicate mechanisms solely to rename the negative control.

## 11. Independent First-Light verifier

`formula-check` owns the independent final verification path.

The verifier consumes frozen manifest/evidence identities and replayable artifacts; it does not call search/discovery to decide whether First Light passed.

Verification checks:

1. source/toolchain/environment bindings;
2. U0 and U1 replay and lineage;
3. FL-A independent certification evidence;
4. FL-B exact reduction/certification evidence;
5. FL-C blindness, near-miss rejection, frozen candidate, exhaustive certification;
6. atomic promotion and U0 history preservation;
7. capability closure expansion into U1;
8. P8 native realization equivalence and immutable dispatch;
9. second-query exact U1 reuse with zero primitive rediscovery metrics;
10. all negative controls;
11. manifest completeness and digest consistency.

Only after all checks succeed may the verifier return the complete frozen marker set.

No public constructor/API permits callers to manufacture a successful `FirstLightVerification` with arbitrary markers.

## 12. Frozen PASS markers

The independent verifier must derive exactly the frozen First-Light marker set:

```text
PASS D1_AUTHORITY_SEPARATION
PASS D2_IDENTITY_GENERATION_REPLAY
PASS D2_CERTIFICATE_ROUTING
PASS D2_SEARCH_STATE_SEPARATION
PASS D3_BLIND_SEMANTIC_ELABORATION
PASS D3_REPRESENTATION_REDUCTION
PASS D3_SYMBOLIC_CANDIDATE_SPACE
PASS D3_FALSE_NEARMISS_REJECTION
PASS D4_NATIVE_REALIZATION_EQUIVALENCE
PASS D4_CPU_LOCAL_OFFLINE
PASS D5_ATOMIC_PROMOTION
PASS D5_CAPABILITY_CLOSURE_EXPANDED
PASS D5_SECOND_QUERY_REUSE
PASS NEGATIVE_CONTROLS
PASS FIRST_LIGHT_COMPLETE
```

No partial marker set constitutes First Light.

`PASS D5_SECOND_QUERY_REUSE` specifically requires exact U1 capability resolution, exact admitted realization dispatch, correct result, zero primitive-discovery CandidateSpaces, and zero primitive-discovery Work Cells.

`PASS FIRST_LIGHT_COMPLETE` is emitted last only after every preceding marker is earned.

## 13. Canonical P9 workflow

The canonical P9 proof workflow is read-only (`permissions: contents: read`) on Ubuntu 24.04 with Rust 1.98.0.

Required sequence mirrors the frozen roadmap:

```text
create/load U0
FL-A blind discovery -> certify
FL-B representation/reduction -> certify
FL-C blind synthesis
mandatory near-miss rejection
freeze/certify FL-C
promote -> U1
compile/validate native realization
submit second reuse query under U1
prove zero primitive rediscovery
run negative controls
assemble proof manifest
independently replay manifest
emit complete marker set
```

After dependency priming, canonical execution uses locked/offline Cargo commands where applicable and proves a clean worktree.

The workflow reruns all predecessor P0-P8 architecture/workspace gates. P9 is finally frozen only when the exact documentation-bearing P9 head passes this unchanged canonical workflow.

## 14. Scope freeze

Included:

```text
activation-derived FL-C primitive capability in U1
canonical second COUNT query
explicit D3 reuse resolution/compilation path
zero-discovery structural metrics
existing P8 native realization selection/dispatch
exact second-query execution/result verification
canonical FirstLightProofManifest
canonical negative-control manifest
independent manifest verifier/replay
all fifteen frozen First-Light PASS markers
canonical P9 proof/freeze
```

Excluded:

```text
P10 generalized promotion class registry
metaprimitive shadow/default activation framework
new grammar-generation evolution machinery
broad primitive registries beyond the bounded First-Light need
GPU/SIMD/JIT
Ptah/distributed execution
new external solver/package federation
self-host/bootstrap trust reduction
performance/autotuning policy
product CLI/UI work
```
