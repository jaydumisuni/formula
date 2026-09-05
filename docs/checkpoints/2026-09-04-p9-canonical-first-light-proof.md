# P9 — Canonical First-Light Proof Checkpoint

## Status

**P9 FINALLY FROZEN. `FIRST_LIGHT_COMPLETE` IS RECOVERED CROSS-CHAT AUTHORITY.**

The frozen P9 proof boundary is the documentation-bearing head below. Later commits that change only recovery documentation to record this already-earned result are post-freeze metadata: they do not redefine the frozen proof head and do not create a recursive requirement to prove the run ID that records itself.

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

P9 source-under-test proof:

```text
head:       7b26c560b0edc5d7726e33f201e0a7be28ebcbcd
workflow:   P9 canonical proof
run:        33929718636
job:        101205682563
conclusion: success
```

P9 **finally frozen documentation-bearing proof**:

```text
head:       b353365fa8b20a13b658c07b3027334b69eff108
workflow:   P9 canonical proof
run:        33950470295
job:        101264153162
conclusion: success
```

The final workflow checked out exact head `b353365fa8b20a13b658c07b3027334b69eff108` and passed every canonical gate through clean-tree.

## Canonical workflow identity

The workflow used for the source run and the final documentation-bearing run was byte-for-byte unchanged:

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

The temporary `.github/workflows/p9-development.yml` was removed before the source-under-test proof and is not part of either canonical proof boundary.

## Structural freeze review

The exact source-proof → final-proof comparison was:

```text
base:          7b26c560b0edc5d7726e33f201e0a7be28ebcbcd
final:         b353365fa8b20a13b658c07b3027334b69eff108
status:        ahead
ahead:         2 commits
behind:        0 commits
changed files: 2
```

The two changed files were recovery documentation only:

```text
CURRENT.md
docs/checkpoints/2026-09-04-p9-canonical-first-light-proof.md
```

No source crate, test, Cargo metadata, fixture, design implementation surface, or canonical workflow changed between the proven source boundary and the final documentation-bearing proof head.

## Reviewed P8 → P9 source delta

```text
base:        fa369b6241c0c069176e5939acf4d5ec74eb8085
source head: 7b26c560b0edc5d7726e33f201e0a7be28ebcbcd
status:      ahead
ahead:       64 commits
behind:      0 commits
files:       48 changed files
```

The reviewed source delta adds durable semantic activation, activation-derived capability closure, explicit reuse compilation, admitted-binary second-query reuse, canonical First-Light manifest/verifier identities, the independent verifier, the one-state canonical proof harness, permanent read-only proof workflow, and supporting tests. Temporary development/one-shot helpers are absent from the source proof boundary.

## Single clean-state canonical proof

One clean authority state proves this sequence without replacing predecessor authority:

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

For the second query the proof establishes:

```text
primitive-discovery CandidateSpaces = 0
primitive-discovery WorkCells = 0
source regeneration after reuse boundary = 0
rustc invocations after reuse boundary = 0
realization reauthorizations after reuse boundary = 0
realization readmissions after reuse boundary = 0
COUNT = 9
```

## Source-run identities

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

## Finally frozen docs-head identities

```text
P9_SOURCE_SHA=b353365fa8b20a13b658c07b3027334b69eff108
P9_TOOLCHAIN_RELEASE=1.98.0
P9_TOOLCHAIN_HOST=x86_64-unknown-linux-gnu
P9_MANIFEST=sha256:b1ddcb7bb73cc1f7247e87c85371ffd5f6c82538e8480440a31583475c46e078
P9_U0=sha256:5eda5f9b76a4cad1a431c5712020b7024466617bc1b5236212993a711f979606
P9_U1=sha256:f3884926ef9eb477b19dcd3cc9056b01e65b285e38ef56209f65b7da8c9dcbe1
P9_NEGATIVE_CONTROLS=sha256:4a854e865807c9831001e6b82af0280c39836a0bba38457497bd034dfa6b67f8
P9_COUNT=9
```

`P9_MANIFEST` intentionally changes because the manifest binds the exact source-commit string. U0, U1, negative-control identity, COUNT result, proof semantics, and frozen marker order reproduced unchanged on the documentation-bearing head.

## Executed negative controls

All twelve controls are present exactly once in the negative-control manifest and are executed against concrete fail-closed paths:

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

These are executed rejection evidence, not list-only definitions.

## Frozen PASS markers

The independent verifier reproduced these fifteen markers in this exact order on both the source proof and the final documentation-bearing proof:

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

## Finally frozen canonical gates

Exact run `33950470295`, job `101264153162`, passed on head `b353365fa8b20a13b658c07b3027334b69eff108`:

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

P9 extends but does not weaken P0–P8:

- search/discovery may propose mathematics but cannot publish authority;
- only independent certification plus promotion can create mathematical authority;
- promotion remains atomic, generation-producing, and history preserving;
- native compilation cannot self-authorize or self-admit;
- realization correctness remains independently checked;
- semantic activation is durable derived authority state bound to the active admitted generation;
- capability closure is derived state and cannot manufacture authority;
- reuse requires exact admitted capability and authority context;
- second-query execution selects an already-admitted realization and cannot silently rediscover/recompile it;
- checker, search/compiler, realization, store, and sealed First-Light dependency firewalls remain enforced.

## What P9 proves

P9 closes the frozen First-Light loop for the canonical targets from one clean state: discover/certify/promote a semantic primitive, persist and activate U1 authority, realize and independently admit a native implementation, expand reusable capability closure, and answer a later COUNT query by exact dispatch of the already-admitted binary with zero primitive rediscovery.

`FIRST_LIGHT_COMPLETE` is now final recovery authority for P9.

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

The next frozen-roadmap boundary is **P10**. P10 must start from exact finally frozen P9 proof head:

```text
b353365fa8b20a13b658c07b3027334b69eff108
```

P10 must not retroactively rewrite P9 source or proof authority.

## Freeze state

```text
P9 source proof:       PROVED
source head:           7b26c560b0edc5d7726e33f201e0a7be28ebcbcd
source run:            33929718636
source job:            101205682563
source conclusion:     success
canonical workflow:    d9bc72e96be27259d948b2ced2a5cbe3de959755
docs-head proof:       PROVED
frozen proof head:     b353365fa8b20a13b658c07b3027334b69eff108
frozen run:            33950470295
frozen job:            101264153162
frozen conclusion:     success
P9 final freeze:       FINALLY FROZEN
FIRST_LIGHT_COMPLETE:  FINAL RECOVERY AUTHORITY
next roadmap phase:    P10
```

Any later commit that changes only this checkpoint and/or `CURRENT.md` to record the already-earned result is post-freeze metadata. It does not redefine `b353365fa8b20a13b658c07b3027334b69eff108` as the frozen proof head and does not require another recursive proof run.
