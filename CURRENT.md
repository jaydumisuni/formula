# CURRENT — Cross-chat recovery authority

**Repository name:** `formula` is temporary only; it is not the final product name.

Recover repository evidence before reasoning. Do not reconstruct implementation state from chat memory when these files are available.

## Primary authorities

1. [`docs/design/README.md`](docs/design/README.md) — frozen D1–D5 design precedence.
2. [`docs/roadmap/2026-08-28-implementation-roadmap.md`](docs/roadmap/2026-08-28-implementation-roadmap.md) — frozen roadmap P0 onward.
3. [`docs/checkpoints/2026-09-02-p5-candidate-space-bounded-discovery.md`](docs/checkpoints/2026-09-02-p5-candidate-space-bounded-discovery.md) — current P5 source-proof checkpoint.
4. [`docs/checkpoints/2026-09-02-p4-query-compiler-campaign-core.md`](docs/checkpoints/2026-09-02-p4-query-compiler-campaign-core.md) — exact frozen P4 predecessor checkpoint.
5. [`docs/superpowers/specs/2026-09-02-p5-candidate-space-bounded-discovery-design.md`](docs/superpowers/specs/2026-09-02-p5-candidate-space-bounded-discovery-design.md) — approved P5 design and freeze boundary.
6. [`docs/superpowers/plans/2026-09-02-p5-candidate-space-bounded-discovery.md`](docs/superpowers/plans/2026-09-02-p5-candidate-space-bounded-discovery.md) — executed P5 implementation plan.
7. [`docs/research/`](docs/research/) — preserved research evidence; reopen only for a concrete contradiction or missing obligation.

## Exact current implementation state

**P5 — CandidateSpace + Bounded Discovery: SOURCE PROVED and scope/review-clean; documentation-bearing head awaiting exact-head proof.**

Canonical branch:

```text
implementation/p5-candidate-space-bounded-discovery
```

Exact frozen P4 predecessor:

```text
d6f92a3a5872d9634d74a5fef688e28adfcec2cf
workflow: P4 canonical proof
workflow run: 33684561410
conclusion: success
```

Canonical P5 source-under-test proof boundary:

```text
3ae3fd1432bae90e3af01d94ab967aa9d7cd7165
workflow: P5 canonical proof
workflow run: 33754844401
job: 100646677420
conclusion: success
```

Exact P4 -> P5 source compare:

```text
base:    d6f92a3a5872d9634d74a5fef688e28adfcec2cf
head:    3ae3fd1432bae90e3af01d94ab967aa9d7cd7165
status:  ahead
ahead:   34 commits
behind:  0 commits
```

Pinned proof toolchain:

```text
Rust 1.98.0
runner: ubuntu-24.04
Cargo proof commands: --locked / --offline where applicable
workflow permissions: contents: read
```

## What P5 now proves

P5 supplies the bounded exact discovery substrate required before sealed First-Light orchestration can begin:

```text
CandidateSpaceContext deterministic local semantic identity
CandidatePolarity / CompletenessClass explicit and non-upgradable
FrozenCandidateSpace deterministic structural identity
FrozenCandidate deterministic identity + structural cost
SearchAuthority::CandidateOnly
AffinePolynomialSpace exact rational family refinement
ReductionRouteSpace exact result/capability filtering
scoped route failure pruning
ObservationalExprSpace exact bounded behavior buckets
counterexample regeneration/rebucketing from grammar bound
bounded CEGIS with candidate freeze before validation
deterministic minimal-cost extraction
fair round-robin fallback
heuristic ranking remains non-authoritative
unaffected local CandidateSpace identity
checker/sealed-target/search-authority firewall
```

Search/resource exhaustion remains distinct from mathematical refutation.

A discovered candidate remains candidate-only until a later independent certification/promotion path acts on it.

## Authority boundary

P5 consumes P4 planning contracts; it does not manufacture authority.

The production dependency direction remains:

```text
formula-engine
├── formula-core
├── formula-store
└── formula-packages

formula-engine -/-> formula-check implementation
formula-engine -/-> formula-first-light implementation
```

P5 cannot certify, admit, activate, publish, promote, or mutate mathematical authority.

The P5 authority-boundary test rejects checker implementation coupling, sealed target imports, and authority publication/promotion paths from search code.

Candidate/search state remains outside admitted `U_g` unless later certified/promoted through the existing authority path.

## CandidateSpace identity law

Candidate-space identity binds local semantic inputs only:

```text
Universe generation
World
query digest
obligation digest
grammar-or-route-set digest
policy digest
exact backend state
```

Identical local semantic inputs and exact state reproduce identical frozen-space and candidate identity.

A relevant local change perturbs identity.

Unrelated campaign state does not perturb an otherwise unchanged local CandidateSpace.

## Exact affine-polynomial law

`AffinePolynomialSpace` stores an exact linear system over normalized bounded rational arithmetic rather than enumerating coefficient tuples.

Adding an exact sample refines the entire compatible affine family. Tests prove dimension/class refinement, deterministic minimal-degree unique extraction, and deterministic freeze independent of sample insertion order.

## Reduction-route law

Route admissibility is exact and precedes cost ranking.

Requested result class and capabilities must be preserved before a route can be considered. A cheaper inadmissible route remains inadmissible.

Failure subtraction is scoped by declared applicability; failure of one route cannot remove an unrelated route outside that scope.

## Observational bounded-space law

The P5 U8/Boolean grammar and evaluator are public and exact; sealed First-Light target definitions are not imported.

Behavior buckets retain deterministic lowest-cost representatives under the current exact sample set.

On a new counterexample/sample, P5 regenerates/rebuckets from the grammar bound. It does not refine only retained representatives, because previously equivalent expressions may split under the new sample.

## Bounded CEGIS law

The P5 loop is:

```text
exact samples
 -> build/refine CandidateSpace
 -> extract deterministic candidate
 -> freeze candidate
 -> invoke caller-supplied oracle
 -> Equivalent or exact Counterexample
 -> refine whole bounded space
 -> repeat within explicit bound
```

Candidate freeze occurs before oracle validation.

The oracle is caller supplied; P5 does not import sealed First-Light fixtures.

Iteration/resource exhaustion returns search/resource unknown, never mathematical refutation.

## Search-policy law

Within an exact space, extraction is deterministic by structural cost and stable identity.

Fair round-robin fallback prevents a heuristic preference from permanently starving another admissible candidate/route.

Heuristics may reorder work only. They cannot:

```text
remove sound candidates
strengthen CandidateSpace polarity
create a Judgement
create PASS
admit/publish/promote authority
```

## Canonical proof correction

The source implementation's semantic tests, build, and rustfmt gates were already green when pinned Clippy exposed three traversal/API-shape issues.

The exact correction was isolated and proved before application:

```text
affine Gaussian elimination: index loops -> slice/iterator traversal
FairRoundRobin::next        -> real Iterator implementation
```

The correction preserved behavior and changed no authority contract, candidate set, completeness claim, dependency direction, or sealed-target boundary.

An isolated diagnostic proof passed engine tests and full workspace Clippy under Rust 1.98.0 before the correction was applied to P5.

A self-removing helper then applied only the verified correction, reran engine tests + full Clippy, removed itself, and pushed the corrected source.

Canonical run `33754844401` subsequently passed the full unchanged P5 proof on exact SHA `3ae3fd1432bae90e3af01d94ab967aa9d7cd7165`.

## P5 proof markers

```text
P5-01 CandidateSpace identity deterministic                         PASS
P5-02 affine exact space refines whole family                      PASS
P5-03 route failure pruning scoped                                 PASS
P5-04 observational buckets exact under current samples            PASS
P5-05 counterexample regeneration preserves bounded candidate set  PASS
P5-06 bounded CEGIS freezes before validation                      PASS
P5-07 heuristics remain non-authoritative                          PASS
P5-08 unaffected local identity preserved                          PASS
P5-09 checker/sealed-target/search authority firewall              PASS
P5-10 P0-P4 gates preserved                                        PASS
```

## P0–P4 remain authority

P5 extends rather than replaces predecessor proofs:

```text
P0 reproducible repository/build + architecture firewall
P1 deterministic identity + immutable generation authority store
P2 independent checker/certificate authority
P3 theory packages + generation/world-scoped capability closure
P4 deterministic query compiler + campaign/obligation/work-cell planning
```

The P5 canonical workflow reruns predecessor crate/workspace tests, build, formatting, lint, dependency, architecture, and clean-tree gates.

## Not proved by P5

Do not claim from P5:

```text
sealed FL-A/B/C target knowledge
independent certification of discovered candidates
promotion/admission of discovered mathematics
U0 -> U1 First-Light growth
native CPU realization generation
full sealed First-Light execution
external SAT/SMT/CAS execution
model/LLM mathematical authority
Ptah/distributed execution
unbounded/universal search completeness
P6 completion
```

## Next implementation boundary

The frozen roadmap phase after P5 is P6.

Do **not** start P6 until the documentation-bearing P5 branch head passes the unchanged P5 canonical workflow.

P6 must consume P5's candidate-only outputs without weakening the independent certification/promotion authority boundary or leaking sealed targets backward into P5.

## Constitutional laws to preserve

1. Search may propose mathematics. Only Certification + Promotion can create mathematical authority.
2. Execution may consume authority. Execution cannot manufacture authority.
3. Mathematical correctness and realization correctness are separate proof obligations.
4. Candidate/search/compiler state is outside admitted `U_g` authority unless explicitly promoted.
5. Resource exhaustion never weakens the requested Authority Contract and never becomes mathematical refutation.
6. Promotion is generation-producing and atomic; accepted history is immutable.
7. A false realization cannot invalidate already admitted mathematics.
8. Capability closure is derived state from exact admitted inputs, not an authority source.
9. Weak Shared Facts cannot silently satisfy stronger obligations.
10. Federation/certificate routing cannot weaken authority for cost or availability.
11. Compiler/campaign/work-cell/search state cannot publish or mutate authority.
12. Replay/provenance/candidate identity must bind every local semantic input capable of changing the result while excluding unrelated state.
13. Heuristic ranking cannot delete exact candidates or create authority.
14. Sealed First-Light targets cannot leak backward into P5 search implementation.

## Recovery procedure

1. Read this file.
2. Read the P5 checkpoint and approved P5 design/plan.
3. Inspect `implementation/p5-candidate-space-bounded-discovery` before assuming the source-proof SHA is still the branch head.
4. Verify the post-checkpoint P5 canonical proof on the **exact documentation-bearing branch head** before treating P5 as finally frozen.
5. Preserve frozen P4 head `d6f92a3a5872d9634d74a5fef688e28adfcec2cf` as the predecessor review boundary.
6. Do not start P6 unless P5's documentation-bearing exact head has passed canonical proof.
7. Do not reopen broad research unless implementation evidence exposes a concrete contradiction or missing requirement.

## Freeze state

P5 source is proved and scope/review-clean on:

```text
3ae3fd1432bae90e3af01d94ab967aa9d7cd7165
```

The P5 checkpoint and this `CURRENT.md` update now form the documentation-bearing branch candidate. **P5 is not finally frozen until the unchanged P5 canonical workflow succeeds on that exact documentation-bearing head.**

This branch has **not** been merged to `main`. P6 has **not** started.
