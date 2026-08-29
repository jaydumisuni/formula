# CURRENT — Cross-chat recovery authority

**Repository name:** `formula` is temporary only; it is not the final product name.

This file is the shortest authoritative recovery path for a new chat/session. Recover repository evidence before reasoning. Do not reconstruct state from chat memory when these files are available.

## Exact current state

Broad research is closed as the lead activity. The canonical D1–D5 architecture, First-Light specification, and implementation roadmap are frozen.

Primary authorities:

1. [`docs/design/README.md`](docs/design/README.md) — canonical D1–D5 design index and precedence.
2. [`docs/roadmap/2026-08-28-implementation-roadmap.md`](docs/roadmap/2026-08-28-implementation-roadmap.md) — frozen implementation phases P0 onward.
3. [`docs/checkpoints/2026-08-29-p3-theory-packages-capability-closure.md`](docs/checkpoints/2026-08-29-p3-theory-packages-capability-closure.md) — proved P3 source milestone and current implementation authority.
4. [`docs/checkpoints/2026-08-29-p2-independent-checker-certificate-core.md`](docs/checkpoints/2026-08-29-p2-independent-checker-certificate-core.md) — proved P2 predecessor retained as independent-checker authority.
5. [`docs/checkpoints/2026-08-29-p1-core-identity-authority-store.md`](docs/checkpoints/2026-08-29-p1-core-identity-authority-store.md) — proved P1 authority-store predecessor.
6. [`docs/checkpoints/2026-08-29-p0-repository-reproducible-build-skeleton.md`](docs/checkpoints/2026-08-29-p0-repository-reproducible-build-skeleton.md) — proved P0 predecessor evidence.
7. [`docs/superpowers/plans/2026-08-29-p3-theory-packages-capability-closure.md`](docs/superpowers/plans/2026-08-29-p3-theory-packages-capability-closure.md) — executed P3 RED/GREEN/freeze plan.
8. [`docs/research/`](docs/research/) — preserved research evidence; reopen only for concrete design/implementation gaps.

## Current implementation milestone

**P3 — Theory Packages + Capability Closure: SOURCE PROVED and review-clean on the isolated implementation branch.**

Canonical branch:

```text
implementation/p3-theory-packages-capability-closure
```

Exact final P2 predecessor branch head:

```text
05d2c433f89c02ebe5187151284d1442c65bfe8e
```

Canonical P3 source-under-test proof boundary:

```text
296902f4dac1d3e0f2b4e6e2e64cfb3c7306c4f7
workflow: P3 canonical proof
workflow run: 33263907506
job: 99130422586
conclusion: success
```

Exact P2 -> P3 compare at the source proof boundary:

```text
base:    05d2c433f89c02ebe5187151284d1442c65bfe8e
head:    296902f4dac1d3e0f2b4e6e2e64cfb3c7306c4f7
status:  ahead
ahead:   46 commits
behind:  0 commits
```

P3 proof checkpoint:

```text
docs/checkpoints/2026-08-29-p3-theory-packages-capability-closure.md
```

Pinned toolchain:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
cargo 1.98.0 (797e8a9bc 2026-08-05)
```

### P3 proof markers

```text
P3-01 theory package structural identity deterministic          PASS
P3-02 minimum builtin package manifests dependency-bound      PASS
P3-03 capability closure scoped by generation/world           PASS
P3-04 witness activation/deactivation deterministic           PASS
P3-05 package composition/interference fails closed           PASS
P3-06 canonical/common-parent ambiguity fails closed          PASS
P3-07 shared mathematical fact polarity enforced              PASS
P3-08 federation adapter cannot manufacture authority         PASS
P3-09 certificate router preserves Authority Contract         PASS
P3-10 P0/P1/P2 architecture and authority gates preserved    PASS
```

## What P3 now proves

P3 adds the minimum D2 semantic package/capability substrate required by the frozen roadmap before P4 can begin:

```text
content-addressed TheoryPackageManifest identity
CapabilityContract
StructureGoal / StructureWitness
minimum Integer/Rational/Boolean/U8/GF2/Polynomial/GF2Vector/GF2Matrix manifests
generation/world-scoped ClosureContext
generation-admitted structure witnesses
deterministic capability derivation
package activation/deactivation
fail-closed package interference/composition
canonical/lossless morphism registry
bounded common-parent resolution
Shared Fact polarity enforcement
FederationAdapter contract validation
Certificate Router v1 exact authority-route selection
```

Capability closure is derived state. It does not become durable authority merely because a package, witness, route, or federation adapter exists.

## Generation-bound authority rules

The corrected P3 authority boundary is explicit:

```text
package activation requires package digests admitted by the exact UniverseGeneration
composition activation requires the CompositionClaim admitted by that generation
composition evidence must also be authority-bound in that generation
morphism registry accepts only generation-admitted canonical morphisms
structure witnesses require semantic admission + evidence authority membership
ActivatedPackageSet is bound to its exact generation digest
closure rejects package/witness state from another generation
closure remains World-scoped
```

The dedicated `p3_authority_admission` and generation-witness tests are part of the successful canonical P3 proof.

## Authority/non-authority separation

P3 preserves the constitutional authority direction:

```text
package manifest             != authority
active package selection     != authority
structure inference proposal != authority
federation producer output   != authority
cheap certificate route      != authority
candidate-only adapter       != authority
```

Only inputs already admitted/certified through the existing P1/P2 authority substrate can enable authoritative package/capability behavior.

Search/compiler/campaign implementation does not yet exist in P3.

## Shared Fact polarity

Shared Mathematical Facts carry explicit polarity. The compatibility gate rejects strength escalation.

Examples:

```text
OVER_APPROXIMATION -/-> EXACT
OVER_APPROXIMATION -/-> existence witness
LOWER_BOUND        -/-> upper-bound consumer
UPPER_BOUND        -/-> lower-bound consumer
heuristic fact     -/-> exact authority requirement
```

Exact facts may satisfy weaker semantically compatible consumers; weak facts cannot discharge stronger obligations.

## Canonical morphisms/common parents

Only generation-admitted, canonical, lossless morphisms can participate in the bounded common-parent path.

```text
unique admissible common parent -> RESOLVED
missing admissible path          -> UNKNOWN
multiple non-equivalent parents  -> AMBIGUOUS
```

No lossy/noncanonical coercion is silently chosen.

## Federation boundary

P3 validates FederationAdapter contracts only; it does not invoke external SAT/SMT/CAS binaries.

```text
CANDIDATE_ONLY result -> non-authoritative
checked/certified mode -> exact declared translation/checker route required
undeclared side effect -> fail closed
unsupported result class -> fail closed
producer identity -> no authority
```

## Certificate Router v1

Route selection preserves the exact requested Authority Contract.

```text
1. filter out every route that fails authority/checker/trust-root requirements
2. order cost only among already-admissible routes
```

A cheaper probabilistic/empirical/heuristic route cannot beat an exact request. If no exact route exists, routing fails closed.

## Dependency authority

Canonical P3 dependency trees:

```text
formula-packages
└── formula-core

formula-check
├── formula-core
└── num-bigint 0.4.8
```

P3 adds no external runtime dependency to `formula-packages`. The P2 checker dependency closure remains unchanged.

The canonical architecture tests still preserve:

```text
formula-check -/-> formula-engine
formula-check -/-> formula-realize
formula-check -/-> formula-first-light
formula-check -/-> formula-store

formula-engine -/-> formula-check implementation
```

## P0/P1/P2 remain proved

The canonical P3 workflow reruns and preserves predecessor gates, including:

```text
P0 repository/build architecture firewall
P0 sealed First-Light fixture identity and boundary
P0 runtime-network allowlist boundary
P1 deterministic structural identity
P1 immutable content-addressed backing
P1 atomic generation publication/rollback
P1 historical replay and corruption rejection
P2 certificate-envelope exact binding
P2 no silent Authority Contract downgrade
P2 malicious-producer rejection
P2 exact polynomial/GF2/U8 checker families
P2 promotion-manifest structural preflight
P2 independent realization-equivalence harness
P2 checker/producer dependency firewall
```

P3 extends these boundaries; it does not replace them.

## Not proved yet

Do **not** claim any of the following from P3:

```text
P4 QueryIR semantic elaboration
P4 TheoryProfile
P4 Representation/Reduction/Campaign IR
P4 Obligation IR / WorkCellPlan / Result Bundle
P4 replay-manifest compiler correctness
P5 CandidateSpace/discovery correctness
full promotion orchestration / U_g -> U_(g+1) beyond existing P1/P2 substrate
First-Light target blindness/discovery proof
native realization generation correctness
external SAT/SMT/CAS federation execution
models as mathematical authority
Ptah/distributed execution
```

## Next implementation boundary

The frozen roadmap names the next phase:

**P4 — Query, Theory Profile, Campaign IR, Obligation compiler.**

P4 has not been started by this checkpoint. P4 must preserve exact query semantics, reject lossy implicit morphisms, maintain representation/reduction result-class metadata, keep Work Cells outside authority writes, and produce complete replay manifests.

P4 is downstream of P3. It must consume the proved package/capability substrate rather than bypassing its generation/world/authority boundaries.

## Canonical First-Light end target remains unchanged

First Light must eventually prove the complete growth loop on ordinary local hardware:

```text
U_0
 -> blind target
 -> structured discovery
 -> reject false candidates
 -> independently certify
 -> admit + activate
 -> U_1
 -> generate native realization
 -> independently validate realization
 -> solve a related second query using the promoted primitive
 -> prove reuse without rediscovery
```

Canonical First Light remains:

```text
local
CPU-only
model-free
network-free during execution
exact/replay-bound
independently checked
content-addressed
bound to exact Universe/package/grammar/source digests
```

Ptah remains explicitly deferred until the local First-Light proof requires distributed execution.

## Constitutional laws to preserve

1. Search may propose mathematics. Only Certification + Promotion can create mathematical authority.
2. Execution may consume authority. Execution cannot manufacture authority.
3. No representation, implementation, proof language, model, solver, federation adapter, or theory package is the mathematics itself.
4. Mathematical correctness and realization correctness are separate proof obligations.
5. Candidate/search/compiler state is outside admitted `U_g` authority unless explicitly promoted through the authority path.
6. Resource exhaustion never weakens the requested Authority Contract.
7. Models may generate candidates later but have no mathematical authority.
8. Promotion is generation-producing and atomic; accepted history is immutable.
9. A false/broken realization cannot invalidate already admitted mathematics.
10. Capability closure is rebuildable derived state from exact admitted inputs; it is not an independent authority source.
11. Weak Shared Facts cannot be silently consumed as stronger facts.
12. Federation/certificate routing cannot weaken authority for cost or availability.

## Recovery procedure for any new chat

1. Read this file.
2. Read [`docs/design/README.md`](docs/design/README.md).
3. Read the frozen implementation roadmap.
4. Read the P3 checkpoint and inspect `implementation/p3-theory-packages-capability-closure` before assuming the source-proof SHA is still the branch head.
5. Verify the post-checkpoint P3 canonical proof on the exact documentation-bearing branch head before treating P3 as finally frozen.
6. Preserve the exact final P2 predecessor head `05d2c433f89c02ebe5187151284d1442c65bfe8e` when reviewing P3 history.
7. Do not start P4 until P3's documentation-bearing head has passed the unchanged P3 canonical proof and P3 is explicitly treated as frozen.
8. Do not return to the older B01 precursor as canonical authority; the frozen D1-D5 roadmap supersedes it.
9. Do not reopen broad research or redesign frozen milestones unless concrete implementation evidence exposes a contradiction or unsupported obligation.
10. Preserve the temporary project-name rule until the mathematical product identity is mature enough to name.

## Evidence precedence

When sources disagree, use this order:

```text
current repository implementation/proof evidence
    > later frozen design amendment explicitly superseding an older milestone
    > D5/D4/D3 for later operational/build/self-expansion choices
    > D2 for operational machine contracts
    > D1 for mathematical constitution
    > preserved research checkpoints
    > chat recollection
```

No chat should claim a later phase is complete merely because a design or implementation plan exists. Every roadmap phase requires its own proof gate.

## Freeze state

P3 source is proved and review-clean on exact commit:

```text
296902f4dac1d3e0f2b4e6e2e64cfb3c7306c4f7
```

The P3 checkpoint and this `CURRENT.md` update now form the documentation-bearing branch candidate. **P3 is not finally frozen until the unchanged canonical P3 workflow succeeds on that exact documentation-bearing head.**

This branch has **not** been merged to `main`. P4 has **not** been started.
