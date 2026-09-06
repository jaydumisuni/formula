# P10 — Self-Expansion Hardening Checkpoint

## Status

**P10 source proof: PROVED.**

**Documentation-head proof: PENDING.**

**P10 final freeze: NOT YET CLAIMED.**

This checkpoint records the exact P10 source-under-test proof boundary. It does not declare the documentation-bearing head finally frozen. Final recovery authority requires the unchanged permanent P10 canonical workflow to succeed on the exact head containing this checkpoint and the corresponding `CURRENT.md` update.

## Frozen predecessor

P10 starts from the exact finally frozen P9 boundary and does not rewrite P9 proof authority:

```text
P9 frozen head:  b353365fa8b20a13b658c07b3027334b69eff108
workflow:        P9 canonical proof
run:             33950470295
job:             101264153162
conclusion:      success
```

## Canonical P10 branch

```text
implementation/p10-self-expansion-hardening
```

## Canonical source proof authority

```text
source head:     4ca5f7d6bc5725ad41ab3afaf94fcf8493f2f696
workflow:        P10 canonical proof
workflow path:   .github/workflows/p10-canonical-proof.yml
workflow blob:   e2b3cc615c33a2ea6dbee3754df0e032c960bb0e
run:             34024050037
job:             101461753634
conclusion:      success
permissions:     contents: read
runner:          ubuntu-24.04
Rust:            1.98.0
rustc commit:    88d9e12ae178fab0fb5cc050a94da85685d449ea
cargo:           1.98.0 (797e8a9bc 2026-08-05)
host:            x86_64-unknown-linux-gnu
LLVM:            22.1.8
```

The temporary P10 development workflow and all one-shot write-capable formatter helpers were removed before this source proof. The canonical workflow checks out the exact source SHA with persisted credentials disabled and has read-only repository permissions.

## What the canonical proof executes

The canonical one-clean-state integration is:

```text
crates/formula-first-light/tests/p10_self_expansion_hardening.rs
```

It executes the P10 hardening chain rather than merely comparing a marker list:

```text
frozen P9 predecessor
 -> total promotion-class registry
 -> U_g with existing Integer/Rational package identities
 -> closure-before without Rational field capability
 -> checked StructureWitness promotion
 -> checker-issued P10 expansion authorization
 -> immutable AuthorityStore promotion U_g -> U_g+1
 -> admitted witness consumed by existing generic closure
 -> Rational field capability unlocked
 -> Rational package structural identity unchanged
 -> Lambda_g derivation
 -> scoped automatic nogood enforcement
 -> preservation-gated promoted route
 -> shadow-only metaprimitive handling
 -> Lambda_g+1 derivation
 -> CandidateSpace bound to exact Lambda generation
 -> semantic-change freshness classification
 -> checker-gated proof transport and repair
 -> independently admitted realization variants
 -> realization-only selection upgrade without changing U
 -> realization selection rollback without deleting newer history
 -> NC10-01...NC10-12 concrete failure/rejection paths
 -> canonical P10 proof manifest
 -> independent checker-owned replay verifier
 -> frozen ordered PASS transcript
```

## Frozen source-run identities

Exact evidence printed by run `34024050037`, job `101461753634`:

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

`SelfExpansionProofManifest` intentionally binds the exact source commit string. Therefore the final documentation-bearing proof will have a different `P10_SOURCE_SHA` and, consequently, a different `P10_MANIFEST` digest. The semantic identities below must remain unchanged unless P10 semantics changed:

```text
P10_U_G
P10_U_G1
P10_REGISTRY
P10_LAMBDA_G
P10_LAMBDA_G1
P10_UNLOCKED_CAPABILITY
P10_NEGATIVE_CONTROLS
ordered marker transcript
```

## Existing Rational package remains unchanged

The source proof derives the Rational field capability by promoting a `StructureWitness` and reusing the existing generic capability-closure machinery. It explicitly proves:

```text
rational_package_before.structural_digest()
    == rational_package_after.structural_digest()
```

The new capability is therefore an authority/closure effect of the admitted witness, not a solver-source or Rational-package rewrite.

## Executed NC10-01...NC10-12

The canonical negative-control manifest requires each P10 control exactly once, and the clean-state proof executes the corresponding failure/rejection path before building that manifest:

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

These controls prove, among other things, that promotion classification remains base-bound, class policy cannot be bypassed, unadmitted/unbound witnesses cannot alter closure, automatic nogoods require scope, route activation requires preservation evidence, CandidateSpace cannot silently cross grammar generations, automatic metaprimitives require the strict gate, non-conservative changes cannot silently transport proof authority, proof repair/transport remains checker-authorized, realization-only upgrades cannot smuggle semantic admission, and rollback cannot rewrite immutable history.

## Frozen canonical marker contract

The independent final verifier emitted exactly this ordered transcript on the source proof:

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

`SELF_EXPANSION_HARDENED` is earned on the exact source-under-test proof. Under the P10 freeze plan it does **not** become final cross-chat recovery authority until the unchanged canonical workflow also succeeds on the exact documentation-bearing candidate head.

## Source-run canonical gates

Run `34024050037`, job `101461753634` passed every permanent P10 gate:

```text
exact source identity + pinned toolchain
locked/offline metadata
canonical one-clean-state self-expansion proof
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
rustfmt --check
Clippy with -D warnings
dependency trees
authority dependency firewall
source authority firewall
clean worktree
```

## P9 -> P10 reviewed source delta

```text
base: b353365fa8b20a13b658c07b3027334b69eff108
head: 4ca5f7d6bc5725ad41ab3afaf94fcf8493f2f696
```

The reviewed delta is confined to the P10 design/plan, permanent P10 canonical workflow, P10 core/checker/engine/package/store/realization/First-Light implementation and tests, plus Cargo manifest/lock adjustments required by those crate changes. No unrelated application surface is part of the P10 source proof boundary.

## Authority model preserved and extended

P10 preserves the P0-P9 authority split:

```text
search/discovery -> candidates only
checker -> mathematical / realization / proof-evolution verification authority
promotion -> atomic generation-producing mathematical publication
AuthorityStore -> authorization-consuming immutable persistence + append-only selection history
capability closure -> generation/world/package-bound derived state only
grammar generation -> explicit structural identity bound to U
CandidateSpace -> exact Lambda-bound candidate state
metaprimitive automation -> shadow first; strict checker gate before automatic activation
proof transport/repair -> checker-authorized exact plans only
realization upgrade -> selection-only; cannot create semantic admission
rollback -> active-selection change only; immutable historical generations/variants remain replayable
execution -> exact admitted realization dispatch only
```

Production dependency and source firewalls remain canonical gates. P10 does not grant search, engine, realization generation, or First-Light production code new certification/store authority.

## What P10 establishes

P10 closes the roadmap's self-expansion-hardening boundary for the implemented Formula architecture:

1. Promotion classes have one deterministic policy registry and class-specific activation/effect constraints.
2. A newly admitted structure witness can unlock a capability already defined by an unchanged theory package.
3. Scoped learned nogoods and preservation-gated routes can affect later grammar/search state without becoming independent authority sources.
4. Grammar is generation-addressed, and CandidateSpace identity is bound to the exact grammar generation.
5. Metaprimitive/search-method promotion is admissible in shadow mode; default automation requires a stricter checker-issued gate.
6. Semantic change has explicit freshness outcomes; non-conservative change cannot silently inherit proof authority.
7. Proof transport and repair create new structural evidence only under exact checker authorization.
8. Multiple admitted realizations can coexist and be selected/upgraded/rolled back without changing mathematical universe generation.
9. Supersession and rollback preserve immutable historical authority and realization variants.
10. A single canonical replay manifest plus independent verifier binds the complete P10 evidence and negative controls.

## Explicit exclusions

Do not claim from P10:

```text
unbounded/general theorem discovery
proof transport across arbitrary unproved relations
automatic metaprimitive activation without the frozen strict gate
semantic admission through realization upgrade
history deletion or mutation during rollback
GPU/SIMD/JIT realization completeness
Ptah/distributed execution completeness
network-backed proof authority
model-backed proof authority
P11 completion
```

P10 hardens self-expansion authority; it does not collapse mathematical authority, execution authority, search policy, and realization selection into one subsystem.

## Final freeze rule

The final P10 recovery boundary requires all of the following:

1. Update only this checkpoint and `CURRENT.md` from the proved source head.
2. Confirm `.github/workflows/p10-canonical-proof.yml` is byte-for-byte unchanged with blob SHA `e2b3cc615c33a2ea6dbee3754df0e032c960bb0e`.
3. Confirm the source-proof-to-docs-head delta contains only:

```text
CURRENT.md
docs/checkpoints/2026-09-05-p10-self-expansion-hardening.md
```

4. Run the unchanged permanent canonical workflow on that exact documentation-bearing head.
5. Require every canonical step through clean-worktree to succeed.
6. Require the stable semantic identities and all thirteen ordered markers to match the source proof.
7. Permit only `P10_SOURCE_SHA` and its source-bound `P10_MANIFEST` identity to change as a consequence of the documentation commit.
8. Record the successful documentation-head SHA/run/job afterward as post-proof recovery metadata. That metadata recording does not redefine the already-proved final P10 boundary and does not create a recursive proof requirement.

## Recovery procedure

1. Read `CURRENT.md`.
2. Read this checkpoint.
3. Read the frozen P10 design and implementation plan.
4. Treat `b353365fa8b20a13b658c07b3027334b69eff108` as the exact P9 predecessor.
5. Treat `4ca5f7d6bc5725ad41ab3afaf94fcf8493f2f696`, run `34024050037`, job `101461753634`, as the exact P10 source-under-test proof boundary.
6. Verify canonical workflow blob `e2b3cc615c33a2ea6dbee3754df0e032c960bb0e` remains unchanged.
7. Until the exact documentation-bearing candidate passes that workflow, report P10 as **source proved; documentation-head proof pending**, not finally frozen.
8. After that exact docs-head success, record its SHA/run/job as final P10 recovery metadata without recursively moving the proof boundary.
9. Only then may the next frozen-roadmap phase, P11, begin.

## Freeze state

Current state at this checkpoint commit:

```text
P10 source proof:       PROVED
source head:            4ca5f7d6bc5725ad41ab3afaf94fcf8493f2f696
source run:             34024050037
source job:             101461753634
source conclusion:      success
canonical workflow:     e2b3cc615c33a2ea6dbee3754df0e032c960bb0e
documentation proof:    PENDING
P10 final freeze:       NOT YET CLAIMED
SELF_EXPANSION_HARDENED: earned on source; pending final docs-head recovery authority
```

The next and only freeze action is to prove the exact documentation-bearing candidate with the **unchanged** permanent P10 canonical workflow.
