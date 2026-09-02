# CURRENT — Cross-chat recovery authority

**Repository name:** `formula` is temporary only; it is not the final product name.

Recover repository evidence before reasoning. Do not reconstruct implementation state from chat memory when these files are available.

## Primary authorities

1. [`docs/design/README.md`](docs/design/README.md) — frozen D1–D5 design precedence.
2. [`docs/roadmap/2026-08-28-implementation-roadmap.md`](docs/roadmap/2026-08-28-implementation-roadmap.md) — frozen roadmap P0 onward.
3. [`docs/checkpoints/2026-09-02-p4-query-compiler-campaign-core.md`](docs/checkpoints/2026-09-02-p4-query-compiler-campaign-core.md) — current P4 source-proof checkpoint.
4. [`docs/checkpoints/2026-08-29-p3-theory-packages-capability-closure.md`](docs/checkpoints/2026-08-29-p3-theory-packages-capability-closure.md) — exact proved/frozen P3 predecessor.
5. [`docs/superpowers/specs/2026-09-02-p4-query-compiler-campaign-core-design.md`](docs/superpowers/specs/2026-09-02-p4-query-compiler-campaign-core-design.md) — approved P4 design.
6. [`docs/superpowers/plans/2026-09-02-p4-query-compiler-campaign-core.md`](docs/superpowers/plans/2026-09-02-p4-query-compiler-campaign-core.md) — executed P4 implementation plan.
7. [`docs/research/`](docs/research/) — preserved research evidence; reopen only for a concrete contradiction or missing obligation.

## Exact current implementation state

**P4 — Query Compiler + Campaign Core: SOURCE PROVED and review-clean; documentation-bearing head awaiting exact-head proof.**

Canonical branch:

```text
implementation/p4-query-compiler-campaign-core
```

Exact frozen P3 predecessor:

```text
5c15368440ad9cc387708dae3c3d73135009f053
```

Canonical P4 source-under-test proof boundary:

```text
2eb1ddef2530d3a46190b4bb62dc7f98ed85dc93
workflow: P4 canonical proof
workflow run: 33684127872
job: 100427383372
conclusion: success
```

Exact P3 -> P4 source compare:

```text
base:    5c15368440ad9cc387708dae3c3d73135009f053
head:    2eb1ddef2530d3a46190b4bb62dc7f98ed85dc93
status:  ahead
ahead:   45 commits
behind:  0 commits
```

Pinned proof toolchain:

```text
Rust 1.98.0
runner: ubuntu-24.04
Cargo proof commands: --locked / --offline where applicable
workflow permissions: contents: read
```

## What P4 now proves

P4 supplies the deterministic compiler/campaign front end required before discovery can begin:

```text
QueryIR exact semantic identity
CompilerAuthoritySnapshot immutable compilation input
RelevantRegion deterministic snapshot projection
TheoryProfile exact-fact / operational-estimate separation
RepresentationEdge preservation/information-loss/reconstruction contracts
ReductionEdge path-wide requested-result preservation
Decomposition explicit aggregation/reconstruction
CampaignIR deterministic typed AND/OR graph
ObligationIR distinct semantic/resource terminal states
WorkCellPlan authority-inert execution request
ReplayManifest complete deterministic binding
ResultBundle non-authoritative structural envelope
CompilerV1 deterministic structural compilation
```

Critical result-state law remains:

```text
REFUTED != SEMANTIC_UNKNOWN != RESOURCE_BOUNDED_UNKNOWN
```

Resource exhaustion cannot become mathematical refutation.

## Authority boundary

P4 consumes authority requirements; it does not manufacture authority.

The direct production dependency direction remains:

```text
formula-engine
├── formula-core
├── formula-store
└── formula-packages

formula-engine -/-> formula-check implementation
```

`CompilerAuthoritySnapshot` is immutable planning input. It contains no publication/rollback/promotion transaction handle.

`WorkCellPlan` contains no authority-store mutation handle or checker implementation pointer. Its public side-effect policies do not permit authority writes.

The authority-boundary test rejects P4 engine source containing known authority publication/mutation call paths or checker implementation coupling.

Search/compiler/campaign state remains outside admitted `U_g` unless later certified/promoted through the existing P1/P2 authority path.

## Representation/reduction law

No lossy morphism is inserted implicitly.

Representation routes must carry explicit preservation metadata. A lossy witness route requires reconstruction semantics.

Reduction composition is valid only when the requested result class survives every edge in the path.

A representation or route may exist as heuristic/approximate planning metadata, but that never rewrites the original obligation's required Authority Contract and never constitutes certification.

## Replay law

Replay identity binds the exact semantic and policy context, including:

```text
Universe generation
World
QueryIR
activated package context
RelevantRegion
TheoryProfile
compiler policy version
scheduler policy version
resource contract
deterministic random key
CampaignIR digest
```

Identical exact inputs produce identical campaign/replay identity. A semantically relevant input change changes replay binding.

## Canonical proof correction

The initial monolithic P4 canonical run failed closed.

Systematic debugging split the **same canonical commands in the same order** into named read-only workflow steps. This mechanically proved the first canonical-only failure was:

```text
cargo fmt --all -- --check
```

All preceding semantic/package/workspace tests and workspace build already passed.

A one-shot Rust 1.98.0 formatter helper applied `cargo fmt --all` under a strict `crates/**/*.rs` scope guard and deleted itself in the same commit. No semantic, authority, dependency, or contract weakening was used to satisfy the gate.

The complete source proof then passed on exact SHA `2eb1ddef2530d3a46190b4bb62dc7f98ed85dc93`, including rustfmt, Clippy `-D warnings`, dependency trees, and clean-tree verification.

## P4 proof markers

```text
P4-01 QueryIR exact semantics preserved                         PASS
P4-02 no lossy implicit morphism                              PASS
P4-03 representation preservation metadata enforced          PASS
P4-04 reduction result classes preserved                      PASS
P4-05 decomposition reconstruction explicit                   PASS
P4-06 CampaignIR deterministic AND/OR                         PASS
P4-07 terminal states remain distinct                         PASS
P4-08 WorkCells authority-inert                               PASS
P4-09 replay manifest complete/deterministic                  PASS
P4-10 P0-P3 gates preserved                                   PASS
```

## P0–P3 remain authority

P4 extends rather than replaces predecessor proofs:

```text
P0 reproducible repository/build + architecture firewall
P1 deterministic identity + immutable generation authority store
P2 independent checker/certificate authority
P3 theory packages + generation/world-scoped capability closure
```

The P4 canonical workflow reruns predecessor crate/workspace tests and architecture gates.

## Not proved by P4

Do not claim from P4:

```text
P5 CandidateSpace enumeration/refinement
CEGIS or adaptive discovery correctness
sealed First-Light target blindness
promotion orchestration beyond the existing P1/P2 substrate
U_g -> U_(g+1) First-Light growth proof
native CPU realization generation
related-query reuse without rediscovery
full First-Light run
external SAT/SMT/CAS federation execution
models as mathematical authority
Ptah/distributed execution
```

## Next implementation boundary

After the documentation-bearing P4 branch head passes the unchanged canonical P4 proof, P4 becomes frozen/proved and the next frozen roadmap phase is:

```text
P5 — CandidateSpace + bounded discovery
```

P5 must consume P4 CampaignIR/ObligationIR/WorkCellPlan contracts without allowing candidate/search state to create or weaken authority.

## Constitutional laws to preserve

1. Search may propose mathematics. Only Certification + Promotion can create mathematical authority.
2. Execution may consume authority. Execution cannot manufacture authority.
3. Mathematical correctness and realization correctness are separate proof obligations.
4. Candidate/search/compiler state is outside admitted `U_g` authority unless explicitly promoted.
5. Resource exhaustion never weakens the requested Authority Contract.
6. Promotion is generation-producing and atomic; accepted history is immutable.
7. A false realization cannot invalidate already admitted mathematics.
8. Capability closure is derived state from exact admitted inputs, not an authority source.
9. Weak Shared Facts cannot silently satisfy stronger obligations.
10. Federation/certificate routing cannot weaken authority for cost or availability.
11. Compiler/campaign/work-cell state cannot publish or mutate authority.
12. Replay/provenance identity must bind every semantic/policy input capable of changing the campaign or verdict.

## Recovery procedure

1. Read this file.
2. Read the P4 checkpoint and approved P4 design/plan.
3. Inspect `implementation/p4-query-compiler-campaign-core` before assuming the source-proof SHA is still the branch head.
4. Verify the post-checkpoint P4 canonical proof on the **exact documentation-bearing branch head** before treating P4 as finally frozen.
5. Preserve frozen P3 head `5c15368440ad9cc387708dae3c3d73135009f053` as the predecessor review boundary.
6. Do not start P5 unless P4's documentation-bearing exact head has passed canonical proof.
7. Do not reopen broad research unless implementation evidence exposes a concrete contradiction or missing requirement.

## Freeze state

P4 source is proved and review-clean on:

```text
2eb1ddef2530d3a46190b4bb62dc7f98ed85dc93
```

The P4 checkpoint and this `CURRENT.md` update now form the documentation-bearing branch candidate. **P4 is not finally frozen until the unchanged canonical P4 workflow succeeds on that exact documentation-bearing head.**

This branch has **not** been merged to `main`. P5 has **not** started.
