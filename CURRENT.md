# CURRENT — Cross-chat recovery authority

**Repository name:** `formula` is temporary only; it is not the final product name.

Recover repository evidence before reasoning. Do not reconstruct implementation state from chat memory when these files are available.

## Primary authorities

1. [`docs/design/README.md`](docs/design/README.md) — frozen D1–D5 design precedence.
2. [`docs/roadmap/2026-08-28-implementation-roadmap.md`](docs/roadmap/2026-08-28-implementation-roadmap.md) — frozen roadmap P0 onward.
3. [`docs/checkpoints/2026-09-04-p9-canonical-first-light-proof.md`](docs/checkpoints/2026-09-04-p9-canonical-first-light-proof.md) — P9 canonical source-proof checkpoint; final docs-head proof pending.
4. [`docs/checkpoints/2026-09-04-p8-native-realization-validation.md`](docs/checkpoints/2026-09-04-p8-native-realization-validation.md) — exact frozen P8 predecessor checkpoint.
5. [`docs/superpowers/specs/2026-09-04-p9-canonical-first-light-proof-design.md`](docs/superpowers/specs/2026-09-04-p9-canonical-first-light-proof-design.md) — frozen P9 design.
6. [`docs/superpowers/plans/2026-09-04-p9-canonical-first-light-proof.md`](docs/superpowers/plans/2026-09-04-p9-canonical-first-light-proof.md) — executed P9 implementation/freeze plan.
7. [`docs/research/`](docs/research/) — preserved research evidence; reopen only for a concrete contradiction or missing obligation.

## Exact current implementation state

**P9 — Canonical First-Light Proof: SOURCE PROVED; DOCUMENTATION-HEAD PROOF PENDING.**

Canonical branch:

```text
implementation/p9-canonical-first-light-proof
```

Exact finally frozen P8 predecessor:

```text
fa369b6241c0c069176e5939acf4d5ec74eb8085
workflow: P8 canonical proof
run: 33899079722
job: 101108627933
conclusion: success
```

Canonical P9 source-under-test proof boundary:

```text
7b26c560b0edc5d7726e33f201e0a7be28ebcbcd
workflow: P9 canonical proof
run: 33929718636
job: 101205682563
conclusion: success
```

The source run checked out exact SHA `7b26c560b0edc5d7726e33f201e0a7be28ebcbcd`, used the permanent read-only P9 workflow, ran on Ubuntu 24.04 with Rust 1.98.0, completed the single clean-state First-Light proof, executed all NC-01…NC-12, emitted all fifteen frozen PASS markers in order, passed every P9/predecessor/workspace gate, and finished with a clean worktree.

The temporary `.github/workflows/p9-development.yml` was removed before the source-under-test proof. It is not part of the canonical source boundary.

## Canonical P9 workflow

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

The final freeze requires this exact workflow blob to remain unchanged on the documentation-bearing candidate head.

## P8 → P9 reviewed source delta

```text
base:    fa369b6241c0c069176e5939acf4d5ec74eb8085
head:    7b26c560b0edc5d7726e33f201e0a7be28ebcbcd
status:  ahead
ahead:   64 commits
behind:  0 commits
files:   48 changed files
```

The P9 source delta adds durable semantic activation, activation-derived U1 capability closure, explicit zero-rediscovery reuse compilation, admitted-binary second-query reuse, canonical First-Light proof/negative-control identities, an independent verifier, a single clean-state canonical proof harness, and the permanent read-only workflow. Temporary development and one-shot helper workflows are absent at the source proof boundary.

## What P9 proves

P9 closes the frozen First-Light loop in one canonical authority state:

```text
FL-A
 -> FL-B
 -> FL-C
 -> checker-certified promotion
 -> U0 -> U1
 -> durable semantic ACTIVATED state
 -> U1 capability closure expansion
 -> deterministic native realization
 -> independent native validation
 -> admitted native binary
 -> second COUNT query
 -> reuse-only compilation
 -> exact dispatch of the already-admitted binary
 -> independent manifest replay/verification
```

For the second query P9 proves:

```text
primitive-discovery CandidateSpaces = 0
primitive-discovery WorkCells = 0
source regeneration after reuse boundary = 0
rustc invocations after reuse boundary = 0
realization reauthorizations after reuse boundary = 0
realization readmissions after reuse boundary = 0
COUNT = 9
```

The reuse path consumes the U1 capability closure and selects the already-admitted P8 native realization under the exact generation/world/authority/observer context. Missing capability or context mismatch fails closed.

## Source-run proof identities

Exact source-run evidence from run `33929718636`, job `101205682563`:

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

`FirstLightProofManifest` binds the exact source commit string. Therefore the final documentation-bearing run will correctly have its own source-bound manifest digest while U0, U1, the negative-control identity, semantic result, and marker contract remain unchanged unless proof semantics changed.

## Executed NC-01…NC-12

The canonical negative-control manifest contains the exact required controls once each and the clean-state run executes their concrete rejection paths:

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

This is execution evidence, not a list-only definition.

## P9 frozen marker contract

The source proof independently replayed the full manifest and emitted exactly these fifteen ordered markers:

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

`FIRST_LIGHT_COMPLETE` has been earned on the exact source-under-test run. Under the P9 freeze plan it does **not** become recovered cross-chat final authority until the unchanged canonical workflow also succeeds on the documentation-bearing head.

## Canonical source-run gates

Run `33929718636` passed:

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

## Authority boundary

P9 extends rather than weakens the frozen P0–P8 authority split:

```text
search/discovery -> candidate only
checker -> mathematical/realization verification authority
promotion -> atomic generation-producing mathematical publication
AuthorityStore -> authorization-consuming immutable persistence
formula-realize/rustc -> untrusted realization production
semantic activation -> active-generation-bound durable activation record
capability closure -> derived state only
reuse compiler -> exact admitted-capability consumption; no discovery authority
execution -> exact admitted realization dispatch only
```

Production dependency laws remain enforced. In particular, the checker cannot link search/engine/store/realization/First-Light implementation authority; the engine cannot link checker/store/First-Light authority; realization generation cannot reach checker/store/First-Light authority; and First-Light production dependencies remain sealed from checker/store/realize implementation dependencies.

## P0–P8 remain authority

P9 consumes and extends, but does not rewrite:

```text
P0 reproducible repository/build + architecture firewall
P1 deterministic identity + immutable atomic authority store
P2 independent checker/certificate authority
P3 theory packages + capability closure
P4 deterministic query/compiler/campaign planning
P5 bounded CandidateSpace + discovery
P6 sealed First-Light harness + blindness gates
P7 checker-authorized atomic promotion into immutable U1
P8 independently authorized native realization + exact dispatch
```

## Not proved by P9

Do not claim from P9:

```text
P10 generalized self-expansion machinery
unbounded/general discovery beyond the frozen First-Light targets
GPU/SIMD/JIT realization
Ptah/distributed execution
network-backed proof authority
model-backed proof authority
arbitrary realization/result-class policies outside the frozen P9 contract
P10 completion
```

## Next implementation boundary

After the P9 documentation-bearing candidate passes the unchanged canonical workflow, P9 may be labeled **FINALLY FROZEN** and the next frozen-roadmap phase is **P10**.

P10 must start from the exact finally frozen P9 head. It must not rewrite P9 source or proof authority.

## Constitutional laws to preserve

1. Search may propose mathematics. Only Certification + Promotion can create mathematical authority.
2. Execution may consume authority. Execution cannot manufacture authority.
3. Mathematical correctness and realization correctness remain separate proof obligations.
4. Candidate/search/compiler state remains outside admitted generation authority until explicitly promoted.
5. Resource exhaustion never becomes mathematical refutation or weakens an Authority Contract.
6. Promotion is generation-producing, atomic, and history preserving.
7. A false realization cannot invalidate already admitted mathematics.
8. Capability closure is derived state, never an authority source.
9. Heuristics cannot delete exact candidates or create authority.
10. Sealed First-Light targets cannot leak backward into discovery.
11. Raw generation publication cannot bypass checker-issued promotion authorization.
12. Realization admission requires independent checker authorization and exact semantic/source/toolchain/binary/context binding.
13. Compilation success alone never creates realization authority.
14. Dispatch may select only an admitted realization matching the exact authority context.
15. Semantic reuse requires exact active-generation capability evidence and cannot silently fall back to rediscovery under the canonical reuse contract.
16. `FIRST_LIGHT_COMPLETE` is final recovery authority only after the unchanged canonical P9 workflow proves the documentation-bearing head.

## Recovery procedure

1. Read this file.
2. Read the P9 checkpoint, P9 design, and P9 implementation plan.
3. Treat `fa369b6241c0c069176e5939acf4d5ec74eb8085` as the exact frozen P8 predecessor.
4. Treat `7b26c560b0edc5d7726e33f201e0a7be28ebcbcd`, run `33929718636`, job `101205682563`, as the exact P9 **source-under-test proof boundary**.
5. Verify the canonical workflow blob is `d9bc72e96be27259d948b2ced2a5cbe3de959755` and has `contents: read`.
6. Until the documentation-bearing exact-head run succeeds, report P9 as **source proved; docs-head proof pending**, not finally frozen.
7. Once that exact docs-head run succeeds, record the frozen head/run/job as post-proof recovery metadata without treating that recording commit as a new proof boundary.
8. Do not reopen broad research unless repository evidence exposes a concrete contradiction or missing requirement.

## Freeze state

Current state:

```text
P9 source proof:       PROVED
source head:           7b26c560b0edc5d7726e33f201e0a7be28ebcbcd
source run:            33929718636
source job:            101205682563
source conclusion:     success
canonical workflow:    d9bc72e96be27259d948b2ced2a5cbe3de959755
documentation proof:   PENDING
P9 final freeze:       NOT YET CLAIMED
FIRST_LIGHT_COMPLETE:  earned on source, pending final docs-head recovery authority
```

The next action is the final plan gate: run the **unchanged** canonical workflow on this documentation-bearing candidate head. Only success on that exact head may establish the finally frozen P9 boundary.
