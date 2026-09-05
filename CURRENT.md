# CURRENT — Cross-chat recovery authority

**Repository name:** `formula` is temporary only; it is not the final product name.

Recover repository evidence before reasoning. Do not reconstruct implementation state from chat memory when these files are available.

## Primary authorities

1. [`docs/design/README.md`](docs/design/README.md) — frozen D1–D5 design precedence.
2. [`docs/roadmap/2026-08-28-implementation-roadmap.md`](docs/roadmap/2026-08-28-implementation-roadmap.md) — frozen roadmap P0 onward.
3. [`docs/checkpoints/2026-09-04-p9-canonical-first-light-proof.md`](docs/checkpoints/2026-09-04-p9-canonical-first-light-proof.md) — finally frozen P9 checkpoint.
4. [`docs/checkpoints/2026-09-04-p8-native-realization-validation.md`](docs/checkpoints/2026-09-04-p8-native-realization-validation.md) — exact frozen P8 predecessor checkpoint.
5. [`docs/superpowers/specs/2026-09-04-p9-canonical-first-light-proof-design.md`](docs/superpowers/specs/2026-09-04-p9-canonical-first-light-proof-design.md) — frozen P9 design.
6. [`docs/superpowers/plans/2026-09-04-p9-canonical-first-light-proof.md`](docs/superpowers/plans/2026-09-04-p9-canonical-first-light-proof.md) — executed P9 implementation/freeze plan.
7. [`docs/research/`](docs/research/) — preserved research evidence; reopen only for a concrete contradiction or missing obligation.

## Exact current implementation state

**P9 — Canonical First-Light Proof: FINALLY FROZEN.**

`FIRST_LIGHT_COMPLETE` is recovered cross-chat authority.

Canonical branch:

```text
implementation/p9-canonical-first-light-proof
```

Exact finally frozen P8 predecessor:

```text
head:       fa369b6241c0c069176e5939acf4d5ec74eb8085
workflow:   P8 canonical proof
run:        33899079722
job:        101108627933
conclusion: success
```

Canonical P9 source-under-test proof boundary:

```text
head:       7b26c560b0edc5d7726e33f201e0a7be28ebcbcd
workflow:   P9 canonical proof
run:        33929718636
job:        101205682563
conclusion: success
```

Exact **finally frozen P9 documentation-bearing proof boundary**:

```text
head:       b353365fa8b20a13b658c07b3027334b69eff108
workflow:   P9 canonical proof
run:        33950470295
job:        101264153162
conclusion: success
```

The final run checked out exact SHA `b353365fa8b20a13b658c07b3027334b69eff108`, used the unchanged permanent read-only P9 workflow, completed the clean-state First-Light proof, executed NC-01…NC-12, reproduced all fifteen frozen PASS markers, passed every P9/predecessor/workspace gate, and finished with a clean worktree.

Any later commit that changes only recovery documentation to record this already-earned result is **post-freeze metadata**. It does not redefine `b353365fa8b20a13b658c07b3027334b69eff108` as the frozen proof head and does not create a recursive requirement to prove the metadata commit that records its own run ID.

## Canonical P9 workflow

The exact workflow blob on both the source proof and final docs-head proof was:

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

The temporary `.github/workflows/p9-development.yml` was removed before the source-under-test proof. It is not part of the frozen P9 boundary.

## Source → final structural review

```text
base:          7b26c560b0edc5d7726e33f201e0a7be28ebcbcd
final:         b353365fa8b20a13b658c07b3027334b69eff108
status:        ahead
ahead:         2 commits
behind:        0 commits
changed files: 2
```

The only changed files were:

```text
CURRENT.md
docs/checkpoints/2026-09-04-p9-canonical-first-light-proof.md
```

No source crate, test, Cargo metadata, fixture, implementation surface, or canonical workflow changed between the source proof and the finally frozen documentation-bearing proof head.

## P8 → P9 reviewed source delta

```text
base:    fa369b6241c0c069176e5939acf4d5ec74eb8085
head:    7b26c560b0edc5d7726e33f201e0a7be28ebcbcd
status:  ahead
ahead:   64 commits
behind:  0 commits
files:   48 changed files
```

The P9 source delta adds durable semantic activation, activation-derived U1 capability closure, explicit zero-rediscovery reuse compilation, admitted-binary second-query reuse, canonical First-Light proof/negative-control identities, the independent verifier, a single clean-state canonical proof harness, and the permanent read-only workflow. Temporary development/one-shot workflow helpers are absent from the source proof boundary.

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

The reuse path consumes U1 capability closure and selects the already-admitted native realization under the exact generation/world/authority/observer context. Missing capability or context mismatch fails closed.

## Source-run proof identities

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

`FirstLightProofManifest` binds the exact source commit string, so the manifest digest changes correctly between source head and docs head. U0, U1, the negative-control identity, semantic result, and marker contract reproduced unchanged.

## Executed NC-01…NC-12

The canonical negative-control manifest contains every required control exactly once and the proof executes their concrete rejection paths:

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

These are executed fail-closed controls, not identity definitions only.

## Frozen marker contract

The independent verifier reproduced exactly these fifteen ordered markers on the final documentation-bearing proof:

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

Run `33950470295`, job `101264153162`, passed:

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

Production dependency laws remain enforced. Search cannot publish authority; checker authority remains separate from search/realization; realization cannot self-authorize; capability closure cannot manufacture authority; exact reuse cannot silently fall back to primitive rediscovery under the canonical contract.

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

The next frozen-roadmap phase is **P10**.

P10 must start from exact finally frozen P9 proof head:

```text
b353365fa8b20a13b658c07b3027334b69eff108
```

P10 must not rewrite P9 source or proof authority.

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
16. `FIRST_LIGHT_COMPLETE` is recovered final authority only from the exact finally frozen P9 docs-head proof.

## Recovery procedure

1. Read this file.
2. Read the P9 checkpoint, P9 design, and P9 implementation plan.
3. Treat `fa369b6241c0c069176e5939acf4d5ec74eb8085` as the exact frozen P8 predecessor.
4. Treat `7b26c560b0edc5d7726e33f201e0a7be28ebcbcd`, run `33929718636`, job `101205682563`, as the P9 source-under-test proof boundary.
5. Treat `b353365fa8b20a13b658c07b3027334b69eff108`, run `33950470295`, job `101264153162`, as the **finally frozen P9 proof boundary**.
6. Verify the canonical workflow identity as `d9bc72e96be27259d948b2ced2a5cbe3de959755` with `contents: read` when auditing P9.
7. Treat later recovery-document-only commits as post-freeze metadata; they do not move the frozen proof boundary and do not require recursive proof.
8. Start P10 only from the exact frozen P9 authority boundary; do not rewrite P9.
9. Do not reopen broad research unless repository evidence exposes a concrete contradiction or missing requirement.

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
