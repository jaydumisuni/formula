# CURRENT — Cross-chat recovery authority

**Repository name:** `formula` is temporary only; it is not the final product name.

Recover repository evidence before reasoning. Do not reconstruct implementation state from chat memory when these files are available.

## Primary authorities

1. [`docs/design/README.md`](docs/design/README.md) — frozen D1-D5 design precedence.
2. [`docs/roadmap/2026-08-28-implementation-roadmap.md`](docs/roadmap/2026-08-28-implementation-roadmap.md) — frozen roadmap P0 onward.
3. [`docs/checkpoints/2026-09-05-p10-self-expansion-hardening.md`](docs/checkpoints/2026-09-05-p10-self-expansion-hardening.md) — exact finally frozen P10 checkpoint.
4. [`docs/superpowers/specs/2026-09-05-p10-self-expansion-hardening-design.md`](docs/superpowers/specs/2026-09-05-p10-self-expansion-hardening-design.md) — frozen P10 design.
5. [`docs/superpowers/plans/2026-09-05-p10-self-expansion-hardening.md`](docs/superpowers/plans/2026-09-05-p10-self-expansion-hardening.md) — executed P10 implementation/freeze plan.
6. [`docs/checkpoints/2026-09-04-p9-canonical-first-light-proof.md`](docs/checkpoints/2026-09-04-p9-canonical-first-light-proof.md) — exact frozen P9 predecessor checkpoint.
7. [`docs/research/`](docs/research/) — preserved research evidence; reopen only for a concrete contradiction or missing obligation.

## Exact current implementation state

**P10 — Self-Expansion Hardening: FINALLY FROZEN.**

**`SELF_EXPANSION_HARDENED`: FINAL RECOVERY AUTHORITY.**

Canonical branch:

```text
implementation/p10-self-expansion-hardening
```

Exact finally frozen P9 predecessor:

```text
b353365fa8b20a13b658c07b3027334b69eff108
workflow: P9 canonical proof
run:      33950470295
job:      101264153162
result:   success
```

Canonical P10 source-under-test proof boundary:

```text
4ca5f7d6bc5725ad41ab3afaf94fcf8493f2f696
workflow: P10 canonical proof
run:      34024050037
job:      101461753634
result:   success
```

Exact finally frozen P10 documentation proof boundary:

```text
3aeb61daf4d575db0f018245ee271597ad475e7b
workflow: P10 canonical proof
run:      34024846890
job:      101463880804
result:   success
```

The source proof established the full P10 implementation under the permanent read-only workflow. The final documentation proof then reran that **unchanged** workflow on the exact documentation-bearing head and passed every canonical gate through clean worktree. The post-proof commits that record this result are recovery metadata only; they do not redefine the frozen proof boundary.

All temporary P10 development and write-capable formatter workflows were removed before the source-under-test proof. They are not part of canonical P10 authority.

## Permanent canonical P10 workflow

```text
path:        .github/workflows/p10-canonical-proof.yml
blob SHA:    39dd9ec67a416b097f4c31d6d2b10120e617a875
permissions: contents: read
runner:      ubuntu-24.04
Rust:        1.98.0
rustc:       88d9e12ae178fab0fb5cc050a94da85685d449ea
cargo:       1.98.0 (797e8a9bc 2026-08-05)
host:        x86_64-unknown-linux-gnu
LLVM:        22.1.8
```

This exact workflow blob was verified on both the proved source head and the final documentation-bearing head. The source-to-docs comparison contained only:

```text
CURRENT.md
docs/checkpoints/2026-09-05-p10-self-expansion-hardening.md
```

No workflow or implementation file changed across the final documentation proof boundary.

## What P10 proves

P10 extends the frozen P0-P9 system with explicit, replayable self-expansion authority rather than allowing search, grammar changes, proof reuse, or realization selection to mutate semantics implicitly.

The canonical proof establishes:

```text
frozen P9 predecessor
 -> deterministic PromotionClassRegistryV1
 -> existing Integer/Rational packages in U_g
 -> no Rational field capability before new evidence
 -> checked StructureWitness promotion
 -> checker-issued expansion authorization
 -> immutable U_g -> U_g+1 promotion
 -> existing generic capability closure consumes admitted witness
 -> Rational field capability becomes available
 -> Rational package digest remains unchanged
 -> explicit Lambda_g and Lambda_g+1 identities
 -> CandidateSpace exact Lambda binding
 -> scoped automatic nogoods
 -> preservation-gated promoted routes
 -> shadow-first metaprimitive behavior
 -> explicit semantic-change freshness
 -> checker-gated proof transport/repair
 -> multiple independently admitted realization variants
 -> realization-only upgrade and rollback without changing U
 -> immutable historical generation/variant replay
 -> NC10-01...NC10-12
 -> source-bound SelfExpansionProofManifest
 -> independent final verifier
 -> PASS SELF_EXPANSION_HARDENED
```

### Promotion/activation policy

Every frozen P10 promotion class has one deterministic policy defining whether it may affect universe admission, capability closure, grammar, or realization selection and which activation modes are legal. A classified P10 authorization remains bound to the exact underlying checker-authorized promotion.

### Non-primitive capability expansion

A promoted `StructureWitness` can unlock a capability already described by an unchanged theory package. The canonical Rational proof demonstrates this with `cap:rational:field`; no Rational solver/package source rewrite is used to manufacture the capability.

### Grammar and CandidateSpace binding

Grammar generations are structural identities bound to the universe generation and active/shadow route/metaprimitive state. CandidateSpace state is bound to the exact `Lambda` digest, so a candidate built under `Lambda_g` cannot be silently reinterpreted under `Lambda_g+1`.

### Proof evolution

Semantic change receives an explicit freshness class. Non-conservative change cannot silently transport existing evidence. Transport and repair plans require exact checker-issued authorization and produce new structural evidence identities.

### Realization-only evolution

Multiple admitted realizations may coexist for one exact semantic dispatch context. Selection can upgrade to a different admitted variant or roll back to an older one without creating a new mathematical universe generation, deleting the newer variant, or rewriting supersession/history.

## Source-run proof identities

Exact source-run evidence from run `34024050037`, job `101461753634`:

```text
P10_SOURCE_SHA=4ca5f7d6bc5725ad41ab3afaf94fcf8493f2f696
P10_P9_PREDECESSOR=b353365fa8b20a13b658c07b3027334b69eff108
P10_MANIFEST=sha256:17e6798fcb991cb280b83084fff0374f4c3ed4f0af330e8cdac4e8b93feda16e
P10_U_G=sha256:697a498ab0e445168b361a1c84ba9679aa1b829111ff0fd88ae6118849009e23
P10_U_G1=sha256:1c241afd0832fbf9fcfd3c1c98ea86b0fa532afff3e574ba34409a16332b167c
P10_REGISTRY=sha256:af2672745b79096bca39d0f4259832df3bbd57bfbc97610973b23f127423b116
P10_LAMBDA_G=sha256:d5e93854c360d41fe12d6503ca651192364aea2268059d1c41bc2384f1959f0c
P10_LAMBDA_G1=sha256:da34606abc4fa9cc4b15a41d739acdf1e078c4dbeb487b30ae9d76c9019addbc
P10_UNLOCKED_CAPABILITY=sha256:9ea6e3e3482b2f44c71af2026e42b4e1a2c706c791cc67bcb5dc9409331e7583
P10_NEGATIVE_CONTROLS=sha256:f6f0ffd18fd26939531d403c9ee9f641b76c38dd7cb594ae357fe3f7b117fb25
```

## Final documentation-run proof identities

Exact final recovery evidence from run `34024846890`, job `101463880804`:

```text
P10_SOURCE_SHA=3aeb61daf4d575db0f018245ee271597ad475e7b
P10_P9_PREDECESSOR=b353365fa8b20a13b658c07b3027334b69eff108
P10_MANIFEST=sha256:3b4a652eddd7db96d49994733db673d195ae10c315c6e518ead607a22cef9732
P10_U_G=sha256:697a498ab0e445168b361a1c84ba9679aa1b829111ff0fd88ae6118849009e23
P10_U_G1=sha256:1c241afd0832fbf9fcfd3c1c98ea86b0fa532afff3e574ba34409a16332b167c
P10_REGISTRY=sha256:af2672745b79096bca39d0f4259832df3bbd57bfbc97610973b23f127423b116
P10_LAMBDA_G=sha256:d5e93854c360d41fe12d6503ca651192364aea2268059d1c41bc2384f1959f0c
P10_LAMBDA_G1=sha256:da34606abc4fa9cc4b15a41d739acdf1e078c4dbeb487b30ae9d76c9019addbc
P10_UNLOCKED_CAPABILITY=sha256:9ea6e3e3482b2f44c71af2026e42b4e1a2c706c791cc67bcb5dc9409331e7583
P10_NEGATIVE_CONTROLS=sha256:f6f0ffd18fd26939531d403c9ee9f641b76c38dd7cb594ae357fe3f7b117fb25
```

The final proof satisfies the frozen identity rule exactly. Only the documentation head's `P10_SOURCE_SHA` and source-bound `P10_MANIFEST` changed. `P10_U_G`, `P10_U_G1`, registry, both `Lambda` identities, unlocked capability, negative-control identity, predecessor identity, and the marker contract remained identical.

## Executed NC10-01...NC10-12

The canonical clean-state integration executes these exact negative controls and only afterward constructs the complete negative-control manifest:

```text
NC10-01 WrongBasePromotion
NC10-02 ForbiddenClassEffect
NC10-03 UnadmittedStructureWitness
NC10-04 UnboundStructureEvidence
NC10-05 UnscopedAutomaticNogood
NC10-06 RouteMissingPreservationEvidence
NC10-07 GrammarGenerationMismatch
NC10-08 UngatedAutomaticMetaprimitive
NC10-09 NonConservativeSilentTransport
NC10-10 UnauthorizedProofRepairOrTransport
NC10-11 RealizationUpgradeSemanticAdmission
NC10-12 RollbackHistoryRewrite
```

This is executable failure-path evidence, not a list-only declaration.

## P10 frozen marker contract

Both the source proof and final documentation-head proof independently replayed the complete manifest and emitted exactly:

```text
PASS P10_PROMOTION_CLASS_REGISTRY
PASS P10_STRUCTURE_WITNESS_PROMOTION
PASS P10_NON_PRIMITIVE_CAPABILITY_UNLOCK
PASS P10_NOGOOD_SCOPE_ENFORCED
PASS P10_ROUTE_PROMOTION_GATED
PASS P10_GRAMMAR_GENERATION_BOUND
PASS P10_METAPRIMITIVE_SHADOW_GATE
PASS P10_SEMANTIC_CHANGE_REVALIDATION
PASS P10_PROOF_TRANSPORT_REPAIR_GATED
PASS P10_REALIZATION_ONLY_UPGRADE
PASS P10_ROLLBACK_HISTORY_PRESERVED
PASS P10_NEGATIVE_CONTROLS
PASS SELF_EXPANSION_HARDENED
```

`SELF_EXPANSION_HARDENED` is final cross-chat recovery authority at exact proof boundary `3aeb61daf4d575db0f018245ee271597ad475e7b`.

## Canonical proof gates

Both canonical proofs passed:

```text
exact identity + pinned toolchain
locked/offline metadata
canonical one-clean-state P10 proof
independent P10 final verifier
frozen P9 predecessor canonical proof
predecessor architecture firewalls
formula-core tests
formula-check tests
formula-store tests
formula-realize tests
formula-packages tests
formula-engine tests
formula-first-light tests
workspace tests
workspace build
rustfmt
Clippy with -D warnings
dependency trees
authority dependency firewall
source authority firewall
clean worktree
```

## Authority boundary

P10 extends rather than weakens the frozen architecture:

```text
search/discovery -> candidate only
checker -> mathematical / realization / proof-evolution verification authority
promotion -> atomic generation-producing mathematical publication
AuthorityStore -> authorization-consuming immutable persistence
capability closure -> generation/world/package-bound derived state only
grammar generation -> explicit structural state bound to U
CandidateSpace -> exact Lambda-bound search state
metaprimitive automation -> shadow by default; strict checker gate for automatic use
proof transport/repair -> exact checker-authorized plan only
realization generation -> untrusted production
realization upgrade -> selection-only over already admitted variants
rollback -> active selection only; immutable history remains replayable
execution -> exact admitted realization dispatch only
```

The permanent workflow enforces both dependency and source authority firewalls. Checker, engine, realization generation, First-Light production code, and store retain separate roles.

## P0-P9 remain authority

P10 consumes and extends, but does not rewrite:

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
P9 canonical First-Light closure + durable activation + zero-rediscovery reuse
```

The exact P9 predecessor is `b353365fa8b20a13b658c07b3027334b69eff108`.

## Not proved by P10

Do not claim from P10:

```text
unbounded/general theorem discovery
proof transport across arbitrary unproved semantic relations
automatic metaprimitive activation without the strict frozen gate
semantic admission through realization-only upgrade
history deletion/mutation during rollback
GPU/SIMD/JIT realization completeness
Ptah/distributed execution completeness
network-backed proof authority
model-backed proof authority
P11 completion
```

## P10 final freeze — satisfied

The P10 freeze plan is closed:

1. Pinned Rust 1.98 formatting normalization was applied to the exact proof-file scope.
2. The one-clean-state `p10_self_expansion_hardening` integration executes all twelve NC10 controls and derives all thirteen markers from actual evidence.
3. The harness passed Clippy, dependency/source firewalls, and clean-tree.
4. Temporary development/write workflows were removed and replaced by the permanent read-only canonical workflow.
5. Exact source head `4ca5f7d6bc5725ad41ab3afaf94fcf8493f2f696` passed canonical proof run `34024050037`, job `101461753634`.
6. Source SHA/run/job and all frozen identities were recovered from the exact job log.
7. `CURRENT.md` and the P10 checkpoint formed the only documentation delta.
8. Exact docs head `3aeb61daf4d575db0f018245ee271597ad475e7b` passed the **unchanged** canonical workflow in run `34024846890`, job `101463880804`.
9. Stable semantic identities and all thirteen markers matched the source proof exactly; only the source SHA and source-bound manifest changed.
10. The exact docs SHA/run/job are now recorded as post-proof recovery metadata without recursively redefining the proof boundary.

## Next implementation boundary

The next frozen-roadmap phase is **P11**.

P11 must start from the exact finally frozen P10 authority state and must not rewrite P10 source/proof authority. The frozen P10 proof boundary remains:

```text
3aeb61daf4d575db0f018245ee271597ad475e7b
run: 34024846890
job: 101463880804
```

Later post-proof metadata commits are not replacements for that boundary.

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
16. Promotion-class effects and activation modes are checked policy, not producer discretion.
17. CandidateSpace semantics are bound to an explicit grammar-generation identity.
18. Non-conservative change cannot silently transport prior proof authority.
19. Proof transport/repair requires exact checker authorization and produces new evidence identity.
20. Realization-only evolution cannot create mathematical admission or advance the universe generation.
21. Supersession and rollback are append-only/history-preserving selection operations.
22. `SELF_EXPANSION_HARDENED` is final recovery authority only because the unchanged canonical P10 workflow proved the exact documentation-bearing head.

## Recovery procedure

1. Read this file.
2. Read the P10 checkpoint, P10 design, and P10 implementation/freeze plan.
3. Treat `b353365fa8b20a13b658c07b3027334b69eff108` as the exact frozen P9 predecessor.
4. Treat `4ca5f7d6bc5725ad41ab3afaf94fcf8493f2f696`, run `34024050037`, job `101461753634`, as the exact P10 source-under-test proof boundary.
5. Treat `3aeb61daf4d575db0f018245ee271597ad475e7b`, run `34024846890`, job `101463880804`, as the exact **finally frozen P10 documentation proof boundary**.
6. Verify `.github/workflows/p10-canonical-proof.yml` blob is `39dd9ec67a416b097f4c31d6d2b10120e617a875` and has `contents: read`.
7. Treat later documentation-only commits that merely record the successful proof metadata as post-proof recovery metadata; they do not move the frozen P10 boundary.
8. P11 may now begin from this frozen P10 authority state.

## Freeze state

```text
P10 source proof:          PROVED
source head:               4ca5f7d6bc5725ad41ab3afaf94fcf8493f2f696
source run:                34024050037
source job:                101461753634
source conclusion:         success
canonical workflow blob:   39dd9ec67a416b097f4c31d6d2b10120e617a875
documentation proof:       PROVED
final proved docs head:    3aeb61daf4d575db0f018245ee271597ad475e7b
final docs run:            34024846890
final docs job:            101463880804
final docs conclusion:     success
final docs P10_MANIFEST:   sha256:3b4a652eddd7db96d49994733db673d195ae10c315c6e518ead607a22cef9732
P10 final freeze:          FINALLY FROZEN
SELF_EXPANSION_HARDENED:   FINAL RECOVERY AUTHORITY
next roadmap phase:        P11
```

P10 is closed. The exact frozen proof boundary remains `3aeb61daf4d575db0f018245ee271597ad475e7b`; post-proof recovery metadata commits do not move it.
