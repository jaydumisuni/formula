# P9 — Canonical First-Light Proof Checkpoint

## Status

**SOURCE PROVED. DOCUMENTATION-HEAD PROOF PENDING.**

This checkpoint records the exact P9 source-under-test proof required by the frozen P9 design and implementation plan. P9 is not `FINALLY FROZEN` until the unchanged read-only canonical workflow succeeds on the documentation-bearing head containing this checkpoint and the corresponding `CURRENT.md` update.

## Authority chain

Frozen P8 predecessor:

```text
head:       fa369b6241c0c069176e5939acf4d5ec74eb8085
workflow:   P8 canonical proof
run:        33899079722
job:        101108627933
conclusion: success
```

P9 canonical branch:

```text
implementation/p9-canonical-first-light-proof
```

P9 canonical source-under-test proof:

```text
head:       7b26c560b0edc5d7726e33f201e0a7be28ebcbcd
workflow:   P9 canonical proof
run:        33929718636
job:        101205682563
conclusion: success
```

The temporary P9 development workflow was removed before this source proof. The source head therefore contains the permanent read-only canonical workflow only for P9 closure.

## Canonical workflow identity

```text
path:        .github/workflows/p9-canonical-proof.yml
blob SHA:    d9bc72e96be27259d948b2ced2a5cbe3de959755
permissions: contents: read
runner:      ubuntu-24.04
Rust:        1.98.0
rustc:       88d9e12ae178fab0fb5cc050a94da85685d449ea
cargo:       1.98.0 (797e8a9bc 2026-08-05)
host:        x86_64-unknown-linux-gnu
LLVM:        22.1.8
```

The workflow primes locked dependencies, then runs proof execution with locked/offline Cargo operations where applicable. The canonical proof itself is CPU-local, model-free, GPU-free, Ptah-free, and network-free after dependency priming.

## Reviewed P8 → P9 source delta

```text
base:        fa369b6241c0c069176e5939acf4d5ec74eb8085
head:        7b26c560b0edc5d7726e33f201e0a7be28ebcbcd
status:      ahead
ahead:       64 commits
behind:      0 commits
files:       48 changed files
```

The reviewed delta adds the P9 design/plan and implementation surfaces for durable semantic activation, activation-derived capability closure, explicit reuse compilation, second-query admitted-binary reuse, canonical First-Light manifest/verifier identities, the independent verifier, the canonical proof harness, the permanent read-only proof workflow, and supporting tests. Temporary development/one-shot workflow helpers are not present at the source boundary.

## Single clean-state canonical proof

The canonical run constructs one clean authority state and proves this sequence without replacing predecessor authority:

```text
FL-A
 -> FL-B
 -> FL-C
 -> checker-certified promotion
 -> U0 -> U1
 -> durable ACTIVATED semantic primitive
 -> U1 capability closure expansion
 -> deterministic native realization
 -> independent realization validation
 -> admitted native binary
 -> second COUNT query
 -> reuse-only compilation
 -> exact dispatch of the already-admitted binary
 -> independent First-Light replay/verifier
```

The second query creates zero primitive-discovery CandidateSpaces and zero primitive-discovery WorkCells. After the reuse-query boundary it does not regenerate source, invoke rustc, reauthorize, or readmit the realization.

Canonical second-query result:

```text
COUNT = 9
```

## Frozen source-run identities

```text
P9_SOURCE_SHA=7b26c560b0edc5d7726e33f201e0a7be28ebcbcd
P9_TOOLCHAIN_RELEASE=1.98.0
P9_TOOLCHAIN_HOST=x86_64-unknown-linux-gnu
P9_MANIFEST=sha256:8d19f240fe676285d13b905524ce374f12d01d25f81e239f4e55383bec19be09
P9_U0=sha256:5eda5f9b76a4cad1a431c5712020b7024466617bc1b5236212993a711f979606
P9_U1=sha256:f3884926ef9eb477b19dcd3cc9056b01e65b285e38ef56209f65b7da8c9dcbe1
P9_NEGATIVE_CONTROLS=sha256:4a854e865807c9831001e6b82af0280c39836a0bba38457497bd034dfa6b67f8
P9_COUNT=9
```

`P9_MANIFEST` binds the exact source commit string, so a documentation-bearing head is expected to have its own source-bound manifest identity. U0, U1, the negative-control manifest, semantic behavior, and marker order must remain stable unless source semantics change.

## Executed negative controls

All twelve required controls are present exactly once in the negative-control manifest and are executed against concrete fail-closed paths:

```text
NC-01 ModifiedSealedTarget
NC-02 SealedImportAttempt
NC-03 FlASampleNearMiss
NC-04 FlBCorruptedTranslation
NC-05 FlCZeroNearMiss
NC-06 ForgedEvidence
NC-07 CandidateMutationAfterCertificate
NC-08 SearchAuthorityWrite
NC-09 MutatedRealizationBinary
NC-10 ActivationRemoved
NC-11 StricterAuthorityWithoutEvidence
NC-12 PromotionParentRace
```

The source run proves these are rejection evidence, not merely named identities.

## Frozen PASS markers

The independent verifier emitted the following fifteen markers in this exact order after replaying and validating the complete manifest:

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

## Canonical source-run gates

Exact run `33929718636`, job `101205682563`, passed all of the following on source head `7b26c560b0edc5d7726e33f201e0a7be28ebcbcd`:

```text
identity + toolchain
locked/offline metadata
canonical clean-state First-Light proof
independent final verifier
reuse compiler
semantic activation store
activation-derived capability closure
admitted-binary second-query reuse
manifest identities
predecessor architecture firewalls
formula-core tests
formula-check tests
formula-store tests
formula-realize tests
formula-packages tests
formula-engine tests
formula-first-light tests
canonical harness tests
workspace tests
workspace build
rustfmt
Clippy with -D warnings
dependency trees
authority dependency firewall
clean worktree
```

## Authority properties preserved

P9 preserves the P0–P8 constitutional split:

- search/discovery may propose mathematics but cannot publish authority;
- only independent certification plus promotion can create mathematical authority;
- promotion remains atomic, generation-producing, and history preserving;
- native compilation cannot self-authorize or self-admit;
- realization correctness remains independently checked;
- semantic activation is durable derived authority state bound to the active admitted generation;
- capability closure is derived state and cannot manufacture authority;
- reuse requires the exact admitted capability and authority context;
- second-query execution selects an already-admitted realization and cannot silently rediscover/recompile it;
- checker, search/compiler, realization, store, and sealed First-Light dependency firewalls remain enforced.

## What P9 proves

P9 closes the frozen First-Light loop for the canonical targets by proving, from one clean state, that the system can discover/certify/promote a semantic primitive, realize and independently admit a native implementation, make that primitive reusable through U1 capability closure, and answer a later COUNT query by dispatching the already-admitted binary with zero primitive rediscovery.

The exact source proof earns `FIRST_LIGHT_COMPLETE` subject to the final documentation-head reproducibility gate required by the frozen plan.

## Exclusions

P9 does **not** prove:

```text
P10 generalized self-expansion machinery
unbounded/general semantic discovery beyond the frozen First-Light targets
GPU/SIMD/JIT realization
distributed/Ptah execution
network-backed proof authority
model-backed proof authority
arbitrary new result-class realization policies outside the frozen P9 contract
```

## Next boundary

After P9 is finally frozen, the next roadmap boundary is **P10**. P10 must start from the exact P9 frozen head and must not retroactively rewrite P9 proof authority.

## Final freeze gate

Current state of this checkpoint:

```text
source proof:      PROVED
docs-head proof:   PENDING
P9 final freeze:   NOT YET CLAIMED
```

The next required action is to run the **unchanged** `.github/workflows/p9-canonical-proof.yml` on the documentation-bearing head containing this checkpoint and the matching `CURRENT.md` update. Only a successful exact-head run may change P9 status to `FINALLY FROZEN` and make `FIRST_LIGHT_COMPLETE` recovered cross-chat authority.
