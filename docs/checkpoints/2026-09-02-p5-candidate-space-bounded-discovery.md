# P5 Checkpoint — CandidateSpace + Bounded Discovery

**Recorded:** 2026-09-04  
**Status:** PROVED SOURCE — bounded exact CandidateSpace/discovery substrate; documentation-bearing head still requires exact-head canonical proof  
**Branch:** `implementation/p5-candidate-space-bounded-discovery`  
**P4 exact frozen predecessor:** `d6f92a3a5872d9634d74a5fef688e28adfcec2cf`  
**Source-under-test commit:** `3ae3fd1432bae90e3af01d94ab967aa9d7cd7165`  
**Canonical source proof run:** `33754844401`  
**Canonical source proof job:** `100646677420`  
**Roadmap authority:** `docs/roadmap/2026-08-28-implementation-roadmap.md`, phase P5  
**Design spec:** `docs/superpowers/specs/2026-09-02-p5-candidate-space-bounded-discovery-design.md`  
**Implementation plan:** `docs/superpowers/plans/2026-09-02-p5-candidate-space-bounded-discovery.md`

---

## 1. Scope

This checkpoint records canonical roadmap phase **P5 — CandidateSpace + Bounded Discovery** on the proved source-under-test commit above.

P5 adds the smallest exact symbolic discovery substrate required between the frozen P4 compiler/campaign core and later sealed First-Light execution. It remains deliberately bounded and candidate-only.

Frozen P5 source surfaces include:

- deterministic `CandidateSpaceContext` identity;
- explicit candidate polarity and completeness contracts;
- immutable `FrozenCandidateSpace` / `FrozenCandidate` identity and cost contracts;
- exact bounded affine-polynomial CandidateSpace over normalized rational arithmetic;
- exact bounded reduction-route CandidateSpace with scoped failure pruning;
- typed U8/Boolean observational grammar and exact evaluator;
- exact bounded observational behavior buckets with regeneration after counterexamples;
- generic bounded CEGIS that freezes candidates before validation;
- deterministic minimal-cost extraction;
- fair round-robin fallback so heuristics cannot permanently starve admissible candidates;
- local CandidateSpace identity unaffected by unrelated campaign state;
- architecture/adversarial gates keeping search outside checker, sealed-target, publication, and promotion authority.

P5 does **not** implement sealed First-Light fixtures, independent target certification, promotion/admission, the `U0 -> U1` transition, native realization, model authority, external SAT/SMT/CAS execution, Ptah/distributed execution, or P6.

---

## 2. Exact predecessor and review boundary

P5 was implemented from the exact final P4 documentation-bearing branch head:

```text
d6f92a3a5872d9634d74a5fef688e28adfcec2cf
```

The proved P5 source boundary is:

```text
3ae3fd1432bae90e3af01d94ab967aa9d7cd7165
```

Exact compare evidence reports:

```text
base:    d6f92a3a5872d9634d74a5fef688e28adfcec2cf
head:    3ae3fd1432bae90e3af01d94ab967aa9d7cd7165
status:  ahead
ahead:   34 commits
behind:  0 commits
```

The P4 -> P5 source delta is confined to intended P5 surfaces:

```text
.github/workflows/p5-canonical-proof.yml
crates/formula-engine/src/affine_polynomial.rs
crates/formula-engine/src/candidate_space.rs
crates/formula-engine/src/discovery.rs
crates/formula-engine/src/lib.rs
crates/formula-engine/src/observational.rs
crates/formula-engine/src/route_space.rs
crates/formula-engine/src/search_policy.rs
crates/formula-engine/tests/p5_adversarial.rs
crates/formula-engine/tests/p5_affine_polynomial.rs
crates/formula-engine/tests/p5_candidate_contract.rs
crates/formula-engine/tests/p5_cegis_policy.rs
crates/formula-engine/tests/p5_first_light_ready.rs
crates/formula-engine/tests/p5_observational.rs
crates/formula-engine/tests/p5_recompilation.rs
crates/formula-engine/tests/p5_route_space.rs
docs/superpowers/plans/2026-09-02-p5-candidate-space-bounded-discovery.md
docs/superpowers/specs/2026-09-02-p5-candidate-space-bounded-discovery-design.md
tests/authority-boundary/tests/p5_search_authority_inert.rs
```

No P1 authority-store production implementation, P2 checker implementation, P3 package/closure implementation, P4 compiler/campaign production implementation, sealed First-Light target implementation, realization implementation, or `main` branch was modified by the reviewed P5 range.

---

## 3. Canonical source proof environment

The canonical source proof ran from exact commit `3ae3fd1432bae90e3af01d94ab967aa9d7cd7165` on GitHub-hosted Ubuntu 24.04 using pinned Rust 1.98.0.

```text
workflow: P5 canonical proof
run:      33754844401
job:      100646677420
result:   success
runner:   ubuntu-24.04
```

The canonical workflow is read-only (`permissions: contents: read`) and uses locked/offline Cargo execution after cache priming.

The proof sequence includes:

```bash
git rev-parse HEAD
rustc -vV
cargo -V
cargo metadata --locked --offline --format-version 1
cargo test -p formula-archtest --locked --offline
cargo test -p formula-core --all-targets --locked --offline
cargo test -p formula-store --all-targets --locked --offline
cargo test -p formula-check --all-targets --locked --offline
cargo test -p formula-packages --all-targets --locked --offline
cargo test -p formula-engine --all-targets --locked --offline
cargo test --workspace --all-targets --locked --offline
cargo build --workspace --all-targets --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo tree --locked --offline -p formula-check --edges normal
cargo tree --locked --offline -p formula-packages --edges normal
cargo tree --locked --offline -p formula-engine --edges normal
test -z "$(git status --porcelain)"
```

The engine dependency firewall also proves that production `formula-engine` does not depend on `formula-check` or `formula-first-light` implementation.

Every named canonical gate completed successfully on the exact source-under-test commit.

---

## 4. Canonical-proof correction history

The first complete P5 implementation already passed its semantic tests, workspace build, and rustfmt gate, but canonical Clippy failed closed under pinned Rust 1.98.0.

Diagnostics were recovered without weakening the proof contract. The exact failures were limited to:

- range-index loops in the affine Gaussian-elimination implementation; and
- a cyclic `FairRoundRobin::next` API that Clippy correctly identified as an iterator-shaped contract.

The correction was isolated and verified before it was applied to the P5 branch:

1. convert the affine elimination loops to slice/iterator traversal while preserving exact arithmetic semantics;
2. implement `Iterator` for `FairRoundRobin<T: Clone>` while preserving cyclic scheduling behavior;
3. run `cargo test -p formula-engine --all-targets --locked --offline`;
4. run full workspace Clippy with `-D warnings` under Rust 1.98.0;
5. apply only the already-verified two-file source correction through a self-removing helper;
6. require a clean canonical exact-head proof afterward.

The verified correction commit is in the ancestry of the source-under-test boundary. No lint suppression, authority weakening, candidate-set weakening, dependency change, or sealed-target leakage was used to satisfy Clippy.

---

## 5. Common CandidateSpace contract

P5 freezes a local semantic context containing the exact generation/world/query/obligation/grammar-or-route-set/policy identity needed by one CandidateSpace.

Candidate-space state explicitly carries:

```text
CandidatePolarity
CompletenessClass
SearchAuthority::CandidateOnly
FrozenCandidateSpace
FrozenCandidate
```

A frozen candidate binds its exact space digest, candidate digest, structural cost, and candidate-only authority class.

Search cannot manufacture `Judgement`, PASS, admission, activation, publication, or promotion authority.

No backend operation may silently strengthen polarity or completeness.

---

## 6. Exact affine-polynomial space

`AffinePolynomialSpace` represents bounded polynomial families as exact linear constraints over normalized signed `i128` rational arithmetic.

The state is an affine linear system, not tuple enumeration.

Adding an exact sample therefore refines the whole compatible coefficient family. Canonical tests prove family-level refinement, deterministic rank/dimension behavior, deterministic minimal-degree unique extraction, and freeze identity independent of sample insertion order.

The pinned-Clippy correction changes only traversal shape inside the exact elimination algorithm; engine tests and canonical proof demonstrate preserved behavior.

---

## 7. Exact reduction-route space

`ReductionRouteSpace` keeps route admissibility separate from ranking.

Routes are filtered by exact requested result class and required capabilities before cost can influence extraction.

Scoped failure subtraction removes only routes inside the failure artifact's declared applicability set. An unrelated route survives a failure that does not cover it.

This preserves the P4 law that a cheap route cannot weaken or bypass the requested Authority Contract.

---

## 8. Observational bounded space

P5 implements the public frozen U8/Boolean grammar family required for later First-Light integration without importing sealed targets.

The evaluator is exact, including wrapping U8 subtraction and bitwise/Boolean semantics.

The bounded space is generated by structural cost and stores deterministic lowest-cost representatives for behavior buckets under the current exact sample set.

When a counterexample/sample is added, the space is regenerated/rebucketed from the grammar bound rather than refining only previously retained representatives. This preserves the complete bounded candidate family because expressions that were observationally equivalent on old samples may split on the new sample.

---

## 9. Bounded CEGIS contract

P5's generic bounded CEGIS loop obeys the sequence:

```text
exact samples
 -> build/refine CandidateSpace
 -> extract deterministic candidate
 -> freeze candidate structurally
 -> invoke caller-supplied oracle
 -> receive Equivalent or exact Counterexample
 -> refine whole bounded space
 -> repeat within explicit bound
```

Candidate freeze is observable before validation.

The oracle interface is caller supplied. P5 does not import sealed First-Light target definitions.

Iteration/resource exhaustion remains search/resource unknown. It never becomes mathematical refutation.

A successful search result remains candidate-only until a later independent authority path certifies it.

---

## 10. Minimal cost, fair fallback, and heuristic non-authority

Within one exact space, deterministic extraction uses structural cost and stable identity ordering.

Portfolio/fallback policy includes deterministic fair round-robin scheduling. Heuristic preference cannot permanently starve another admissible route/candidate.

Heuristics may reorder work only. They cannot:

```text
remove sound candidates
strengthen CandidateSpace polarity
create a Judgement
create PASS
publish/admit/promote authority
```

---

## 11. Local identity / recompilation law

P5 CandidateSpace identity binds local semantic inputs only.

Identical local generation/world/query/obligation/grammar-or-route-set/policy inputs and exact state reproduce identical frozen identities.

Changing a relevant local input changes identity.

Changing unrelated campaign state does not perturb an otherwise unchanged local CandidateSpace.

This preserves D3-P10's unaffected-local-identity requirement.

---

## 12. Search authority and sealed-target firewall

P5 source remains downstream of P4 planning contracts and outside admitted mathematical authority.

The production dependency direction remains:

```text
formula-engine
├── formula-core
├── formula-store
└── formula-packages

formula-engine -/-> formula-check implementation
formula-engine -/-> formula-first-light implementation
```

The P5 authority-boundary gate rejects checker implementation coupling, sealed fixture imports, and authority publication/promotion paths from search code.

Public P5 integration fixtures demonstrate the required structures without embedding sealed FL-A/FL-B/FL-C answers as privileged search constants.

---

## 13. P5 canonical proof markers

The successful source proof emitted all required markers:

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

---

## 14. P0–P4 remain authority

P5 extends rather than replaces predecessor proofs:

```text
P0 reproducible repository/build + architecture firewall
P1 deterministic identity + immutable generation authority store
P2 independent checker/certificate authority
P3 theory packages + generation/world-scoped capability closure
P4 deterministic query compiler + campaign/obligation/work-cell planning
```

The P5 canonical workflow reruns predecessor crate/workspace tests, build, formatting, lint, dependency, architecture, and clean-tree gates.

---

## 15. Not proved by P5

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

---

## 16. Freeze procedure and next boundary

The approved P5 design requires this sequence:

```text
RED/GREEN implementation
canonical exact-head source proof
exact P4 -> P5 scope/authority review
checkpoint + CURRENT update
canonical exact-head proof with documentation present
```

The first three stages are complete at the source-under-test boundary recorded here.

This checkpoint plus the subsequent `CURRENT.md` update creates the documentation-bearing P5 freeze candidate. P5 is **not finally frozen** until the unchanged `P5 canonical proof` workflow succeeds on that exact documentation-bearing branch head.

Only after that exact-head success may the repository advance to P6.

No P6 implementation is part of this checkpoint.
