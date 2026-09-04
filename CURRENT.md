# CURRENT — Cross-chat recovery authority

**Repository name:** `formula` is temporary only; it is not the final product name.

Recover repository evidence before reasoning. Do not reconstruct implementation state from chat memory when these files are available.

## Primary authorities

1. [`docs/design/README.md`](docs/design/README.md) — frozen D1–D5 design precedence.
2. [`docs/roadmap/2026-08-28-implementation-roadmap.md`](docs/roadmap/2026-08-28-implementation-roadmap.md) — frozen roadmap P0 onward.
3. [`docs/checkpoints/2026-09-04-p7-promotion-generation-transition.md`](docs/checkpoints/2026-09-04-p7-promotion-generation-transition.md) — current P7 source-proof checkpoint.
4. [`docs/checkpoints/2026-09-04-p6-first-light-target-harness-blindness.md`](docs/checkpoints/2026-09-04-p6-first-light-target-harness-blindness.md) — exact frozen P6 predecessor checkpoint.
5. [`docs/superpowers/plans/2026-09-04-p7-promotion-generation-transition.md`](docs/superpowers/plans/2026-09-04-p7-promotion-generation-transition.md) — executed P7 implementation plan.
6. [`docs/research/`](docs/research/) — preserved research evidence; reopen only for a concrete contradiction or missing obligation.

## Exact current implementation state

**P7 — D5 Promotion + Generation Transition: SOURCE PROVED and scope/review-clean; documentation-bearing head awaiting exact-head proof.**

Canonical branch:

```text
implementation/p7-promotion-generation-transition
```

Exact frozen P6 predecessor:

```text
035953854f33fe47dc884850dec4fdee7a3571e7
workflow: P6 canonical proof
workflow run: 33854369705
conclusion: success
```

Canonical P7 source-under-test proof boundary:

```text
ca61ed42ab47b3a79a3d258f015b2f6ac9979ec2
workflow: P7 canonical proof
workflow run: 33861803872
job: 100987699053
conclusion: success
```

Exact P6 -> P7 source compare:

```text
base:    035953854f33fe47dc884850dec4fdee7a3571e7
head:    ca61ed42ab47b3a79a3d258f015b2f6ac9979ec2
status:  ahead
ahead:   44 commits
behind:  0 commits
```

Pinned proof toolchain:

```text
Rust 1.98.0
runner: ubuntu-24.04
Cargo proof commands: --locked / --offline where applicable
canonical workflow permissions: contents: read
```

## What P7 now proves

P7 implements the bounded D5 authority transition required before native realization:

```text
PromotionCandidate exact structural identity
CERTIFIED / ADMITTED / ACTIVATED / QUARANTINED distinct states
PromotionRecord / QuarantineRecord structural identity
PromotionPolicyV1 checker policy
opaque checker-issued PromotionAuthorization
fail-closed quarantine path
raw public generation-publication bypass closed
AuthorityStore::promote requires PromotionAuthorization
atomic U_g -> U_(g+1) using the proved P1 transaction
parent-generation race rejection
publication-failure rollback
historical U0 replay preservation
CapabilityClosureDelta derived from admitted authority only
blind FL-C semantic primitive admission into U1
search -> checker/store/promotion authority firewall
```

## Authority boundary

P7 makes the certification/promotion separation executable.

The public authority transition is:

```text
frozen candidate
 -> independent checker validation
 -> opaque PromotionAuthorization
 -> AuthorityStore::promote
 -> new immutable UniverseGeneration
```

The low-level generation-publication primitive is crate-private inside `formula-store`.

Production discovery remains outside authority publication:

```text
formula-engine -/-> formula-check
formula-engine -/-> formula-store
formula-engine -/-> formula-first-light
```

The inherited `formula-engine -> formula-store` normal dependency was removed in P7 specifically to close that authority path.

`PromotionAuthorization` has private authority-bearing fields and no public constructor. The store consumes authorization; it does not independently decide mathematical correctness.

## Promotion policy law

Checker authorization binds and validates:

```text
frozen candidate identity
promotion manifest identity
active/expected parent generation
candidate generation
proof generation/freshness boundary
dependency cone
checked evidence bindings
supersession lineage
proposed admissions
```

Mismatch/conflict fails closed to rejection/quarantine. Search cannot upgrade itself into authority.

## Generation-transition law

`AuthorityStore::promote` requires exact parent-generation agreement, replays the parent, constructs only the authorized admission/binding delta, and uses the existing P1 atomic transaction.

Successful P7 integration proves:

```text
U0 digest unchanged
U0 canonical bytes unchanged
U0 remains replayable after U1
U1 parent = U0
U1 contains authorized FL-C primitive
U1 contains authorized evidence binding
failed/partial publication cannot expose U1
stale parent cannot win a race
```

## Capability-closure law

`CapabilityClosureDelta::between(before, after)` is deterministic derived state.

Capability availability still comes only from exact generation/world-scoped admitted inputs and authority-bound witnesses. Closure cannot manufacture authority.

## FL-C bounded promotion law

The P7 FL-C path is:

```text
public bounded U8/Boolean CandidateSpace
 -> extract/freeze candidate
 -> sealed oracle exact counterexample or equivalence
 -> refine bounded space
 -> final frozen candidate
 -> checker authorization
 -> atomic promotion
 -> U1 admission
```

`formula-first-light` uses checker/store only as **dev-dependencies** for this integration proof. Its production dependencies remain core + engine, preserving the P6 sealed-harness boundary.

## Canonical proof correction

The semantic implementation, architecture tests, workspace tests, and build were green before canonical proof failed closed only at rustfmt.

A read-only Rust 1.98.0 diagnostic recovered exactly 11 formatter-touched Rust paths. A one-shot helper enforced that exact allowlist, reran P7 crate/authority tests and full workspace Clippy with `-D warnings`, committed only canonical formatter output, and removed itself.

Canonical run `33861803872` then passed the unchanged full P7 proof on exact clean source SHA `ca61ed42ab47b3a79a3d258f015b2f6ac9979ec2`.

## P7 proof markers

```text
PASS P7_FROZEN_BEFORE_CERTIFICATION
PASS P7_LIFECYCLE_STATES_DISTINCT
PASS P7_CHECKER_AUTHORIZATION_REQUIRED
PASS P7_RAW_PUBLICATION_BYPASS_CLOSED
PASS P7_ATOMIC_U0_TO_U1
PASS P7_U0_HISTORY_REPLAY_PRESERVED
PASS P7_PARENT_RACE_REJECTED
PASS P7_QUARANTINE_FAILS_CLOSED
PASS P7_CAPABILITY_CLOSURE_DERIVED
PASS P7_FLC_PRIMITIVE_ADMITTED
PASS P7_SEARCH_AUTHORITY_FIREWALL
```

## P0–P6 remain authority

P7 extends rather than replaces:

```text
P0 reproducible build + architecture firewall
P1 deterministic identity + immutable atomic authority store
P2 independent checker/certificate authority
P3 theory packages + capability closure
P4 query/compiler/campaign planning
P5 bounded CandidateSpace + discovery
P6 sealed First-Light harness + blindness gates
```

## Not proved by P7

Do not claim from P7:

```text
native CPU realization generation
native realization equivalence/admission
realization dispatch integrity
second-query reuse under U1
proof that synthesis is skipped on reuse
complete P9 First-Light proof manifest
FIRST_LIGHT_COMPLETE
P8 completion
P9 completion
Ptah/distributed execution
```

## Next implementation boundary

The frozen roadmap phase after P7 is **P8 — D4 native realization and validation**.

Do **not** start P8 until the documentation-bearing P7 branch head passes the unchanged P7 canonical workflow.

P8 must consume the P7-admitted FL-C semantic construction, create the bounded native CPU realization, independently validate all 256 U8 inputs, bind source/binary/toolchain identity, and preserve the mathematical-authority vs realization-authority separation. P9 reuse logic is not P8 scope.

## Constitutional laws to preserve

1. Search may propose mathematics. Only Certification + Promotion can create mathematical authority.
2. Execution may consume authority. Execution cannot manufacture authority.
3. Mathematical correctness and realization correctness are separate proof obligations.
4. Candidate/search/compiler state remains outside admitted generation authority until explicitly promoted.
5. Resource exhaustion never becomes mathematical refutation or weakens an Authority Contract.
6. Promotion is generation-producing, atomic, and history preserving.
7. A false realization cannot invalidate already admitted mathematics.
8. Capability closure is derived state, never an authority source.
9. Heuristics cannot delete exact candidates or create authority.
10. Sealed First-Light targets cannot leak backward into discovery.
11. Raw generation publication cannot bypass checker-issued promotion authorization.
12. Realization admission in P8 must remain independently checked and separately bound to the admitted semantic target.

## Recovery procedure

1. Read this file.
2. Read the P7 checkpoint and P7 implementation plan.
3. Inspect `implementation/p7-promotion-generation-transition` before assuming the source-proof SHA is still branch head.
4. Verify the post-documentation P7 canonical proof on the **exact documentation-bearing branch head** before treating P7 as finally frozen.
5. Preserve frozen P6 docs head `035953854f33fe47dc884850dec4fdee7a3571e7` as predecessor review boundary.
6. Do not start P8 unless P7 documentation-bearing exact head has passed canonical proof.
7. Do not reopen broad research unless implementation evidence exposes a concrete contradiction or missing requirement.

## Freeze state

P7 source is proved and scope/review-clean on:

```text
ca61ed42ab47b3a79a3d258f015b2f6ac9979ec2
```

The P7 checkpoint and this `CURRENT.md` update now form the documentation-bearing branch candidate. **P7 is not finally frozen until the unchanged P7 canonical workflow succeeds on that exact documentation-bearing head.**

This branch has **not** been merged to `main`. P8 has **not** started.
