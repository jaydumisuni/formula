# CURRENT - Cross-chat recovery authority

Repository: jaydumisuni/formula

Recover repository evidence before reasoning.

## Primary authorities

1. docs/design/README.md - frozen D1-D5 design precedence.
2. docs/roadmap/2026-08-28-implementation-roadmap.md - frozen P0-P13 roadmap.
3. docs/checkpoints/2026-09-07-p12-self-hosting-bootstrap-trust-reduction.md - current P12 source-proof checkpoint.
4. docs/checkpoints/2026-09-06-p11-federation-breadth.md - exact finally frozen P11 predecessor.
5. docs/superpowers/specs/2026-09-06-p12-self-hosting-bootstrap-trust-reduction-design.md - frozen P12 design.
6. docs/superpowers/plans/2026-09-06-p12-self-hosting-bootstrap-trust-reduction.md - executed P12 implementation plan.

## Exact current implementation state

P12 Self-Hosting Bootstrap / Trust Reduction: SOURCE PROVED; documentation-head proof pending.

Canonical branch: implementation/p12-self-hosting-bootstrap-trust-reduction

Frozen P11 predecessor: 6f8ce7bb6702ea1baf119aab9950aa5ba0f87283
P11 docs run/job: 34031806639 / 101482593250

P12 source proof:
head: df9351f98d2f2a6633f6524ae24c59e9db92dcfa
workflow: P12 canonical proof
run/job: 34080398624 / 101614463057
result: success

## Permanent canonical P12 workflow

path: .github/workflows/p12-canonical-proof.yml
blob SHA: e790944ab57e003c597aeacc33ff6a96e1abba8f
permissions: contents: read
runner: ubuntu-24.04
Rust: 1.98.0

The workflow checks out exact source with persist-credentials false, binds GITHUB_SHA, hashes rustc/cargo/toolchain seed inputs, and runs proof gates offline after dependency priming.

## What P12 source proof establishes

explicit B0 seed provenance -> deterministic Formula-owned bootstrap core -> Formula-owned generator -> independent Formula validator -> separate bootstrap authority store -> T0 -> independently validated T1 -> admitted T1 generator -> T2 self-rebuild -> T1/T2 byte and semantic equivalence -> independent final replay -> active Universe authority unchanged.

B0 remains explicit provenance. Generator output cannot self-authorize. Bootstrap authority remains separate from mathematical Universe authority.

## Frozen P12 marker contract

PASS P12_B0_SEED_EXPLICIT
PASS P12_BOOTSTRAP_CORE_DETERMINISTIC
PASS P12_GENERATOR_VALIDATOR_DIVERSE
PASS P12_STAGE1_INDEPENDENTLY_VALIDATED
PASS P12_STAGE2_SELF_REBUILD_VALIDATED
PASS P12_BYTE_EQUIVALENCE
PASS P12_SEMANTIC_EQUIVALENCE
PASS P12_UNIVERSE_AUTHORITY_UNCHANGED
PASS P12_NEGATIVE_CONTROLS
PASS BOOTSTRAP_TRUST_REDUCED

## Canonical source gates

Run 34080398624 / job 101614463057 passed exact identity/toolchain; B0 seed provenance; locked offline metadata; bootstrap identities; workflow seed parser; generator; independent validator; final replay; separate bootstrap store; T0->T1->T2 proof; frozen P11 predecessor; architecture firewalls; all crate/workspace tests; build; rustfmt; clippy -D warnings; dependency trees; authority/source firewalls; clean worktree.

## Not yet proved

P12 final documentation freeze is not yet proved. BOOTSTRAP_TRUST_REDUCED is source-proved only. P12 does not remove B0, self-host Rust, prove Ptah/distributed execution, or permit network/model proof authority.

## Documentation-head freeze requirement

Only CURRENT.md and docs/checkpoints/2026-09-07-p12-self-hosting-bootstrap-trust-reduction.md may change from the source-proof head. The canonical workflow blob must remain e790944ab57e003c597aeacc33ff6a96e1abba8f. The unchanged workflow must prove the exact documentation-bearing head.

## Freeze state

P11 final freeze: FINALLY FROZEN
P11 frozen proof head: 6f8ce7bb6702ea1baf119aab9950aa5ba0f87283
P12 source proof: PROVED
P12 source head: df9351f98d2f2a6633f6524ae24c59e9db92dcfa
P12 source run/job: 34080398624 / 101614463057
P12 canonical workflow: e790944ab57e003c597aeacc33ff6a96e1abba8f
P12 documentation proof: PENDING
P12 final freeze: PENDING
BOOTSTRAP_TRUST_REDUCED: SOURCE-PROVED ONLY
next operation: prove documentation-bearing head
