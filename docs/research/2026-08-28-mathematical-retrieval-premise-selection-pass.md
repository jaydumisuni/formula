# Research Pass — Mathematical Retrieval, Term Indexing, and Large-Theory Premise Selection

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates how the unnamed project can make millions of proved formulas, transformations, certificates, structures, and primitives genuinely available at problem-solving speed rather than merely stored in a large knowledge base.

The central finding is:

> **Large automated-reasoning systems depend on compiled structural indexes and relevance filters. Retrieval is part of the execution architecture: matching, rewriting, subsumption, and premise selection must narrow the mathematical universe before expensive proof/search begins.**

---

## 1. Term indexing is a core theorem-prover performance mechanism

First-order theorem provers use specialized indexes for expensive retrieval operations:

- discrimination/fingerprint indexing for superposition/rewrite partners;
- path/fingerprint indexing for backward rewriting;
- feature-vector indexing for subsumption.

Source:

https://www.eprover.org/EVENTS/Superposition-25/TutorialSP_ho.pdf

### Architectural implication

The project should not expose the permanent mathematical ledger directly to hot search.

Instead it needs rebuildable indexes such as:

```text
exact semantic identity index
unification/matching index
rewrite index
subsumption/generalization index
structure/property index
theory/interface index
certificate/witness index
```

Each index is an operational projection of immutable accepted mathematics.

---

## 2. Different queries require different indexing structures

The theorem-proving literature distinguishes perfect and imperfect indexing and uses different data structures for matching, unification, generalization, rewriting, and subsumption.

Source:

https://resources.mpi-inf.mpg.de/departments/rg1/conferences/vtsa09/slides/schulz.pdf

### Architectural implication

There should be no universal “math search index.”

A query must declare retrieval semantics:

```text
find exact equivalent
find unifiable theorem
find rule whose LHS matches subobject
find general theorem subsuming this case
find specialization of current structure
find morphism-compatible capability
```

The compiler routes each retrieval to the appropriate index family.

---

## 3. Structural sharing makes large indexes practical

Discrimination trees share common structural prefixes of indexed terms. Substitution/context-tree research shares common substitutions or contexts even more aggressively.

Source:

https://people.mpi-inf.mpg.de/alumni/ag2/2011/hg/papers/journals/abstracts.html

### Architectural implication

The same structural compression principle seen throughout this research appears again:

```text
large theorem collection
    -> compact shared structural index
```

The number of stored theorems is not the same as the size/cost of the retrieval representation.

---

## 4. Lean/mathlib uses discrimination trees to shortlist applicable library mathematics

Mathlib's current premise-suggestion machinery builds refined discrimination trees for imported/current/local theorems and supports lookups for rewrite/apply-style use.

Sources:

https://leanprover-community.github.io/mathlib4_docs/Mathlib/Tactic/ClickSuggestions/FindPremises.html

https://leanprover-community.github.io/mathlib4_docs/Mathlib/Lean/Meta/RefinedDiscrTree.html

### Architectural implication

This provides a contemporary proof that a very large formal library can support interactive structural theorem retrieval without scanning every fact.

The project's primitive/certificate registry should similarly compile hot indexes when a mathematical generation is accepted.

---

## 5. Index build cost can be amortized over a mathematical generation

Mathlib documentation notes a measurable one-time cost to build imported discrimination trees, after which lookups are reused.

Source:

https://leanprover-community.github.io/mathlib4_docs/Mathlib/Tactic/ClickSuggestions/FindPremises.html

### Architectural implication

The generation model from earlier research fits naturally:

```text
accepted mathematical generation G_n
    -> build/rebuild immutable retrieval indexes once
    -> thousands/millions of queries reuse indexes

new generation G_(n+1)
    -> incremental/rebuilt indexes
```

This is another reason permanent truth should not mutate chaotically during active campaigns.

---

## 6. Premise selection is essential for large theories

Hammer/Sledgehammer-style systems first select a smaller subset of a huge library before sending a goal to automated theorem provers. Techniques include deterministic similarity/relevance filters such as SInE as well as learned selectors.

Source:

https://link.springer.com/article/10.1007/s10817-018-9458-4

### Architectural implication

The project should have **model-optional layered relevance selection**:

```text
hard admissibility filters
    structure/type/theory/assumption compatibility

structural retrieval
    matching/unification/subsumption/morphism indexes

symbolic relevance expansion
    dependency/theory-graph/SInE-like filtering

optional learned/model ranking
    only reorders admissible candidates
```

Models may improve ranking but are not required for correctness or basic capability.

---

## 7. Irrelevant mathematics actively hurts proof search

Large-theory systems report premise selection as necessary because irrelevant premises increase theorem-prover search difficulty. Current Naproche-ZF work describes lack of premise selection as a major scaling barrier.

Source:

https://link.springer.com/chapter/10.1007/978-3-031-63498-7_7

### Architectural implication

“Give the solver everything we know” is a bad architecture.

The Problem Compiler should construct a **bounded relevant mathematical context** while retaining a guaranteed/fair escape path to broaden the context if necessary.

This connects directly to the search-economy exploration/exploitation law.

---

## 8. Retrieval should be semantic/structural, not filename/name based

Lean's `#find`/library-search family can search theorem types/patterns rather than relying only on theorem names. Refined discrimination trees can also match under binders.

Sources:

https://leanprover-community.github.io/mathlib_docs/tactic/find.html

https://leanprover-community.github.io/mathlib4_docs/Mathlib/Lean/Meta/RefinedDiscrTree.html

### Architectural implication

Names and human taxonomy should be metadata.

Authoritative retrieval keys should derive from:

```text
semantic form
mathematical structures
relation signatures
assumptions
canonical/equivalence class
morphism/interface compatibility
proof/result class
```

This helps the machine reuse mathematics across domains with unrelated human terminology.

---

## 9. The same theorem may need different retrieval directions

A theorem may be useful as:

```text
forward implication
backward goal reducer
rewrite left-to-right
rewrite right-to-left
contrapositive
specialization
inverse/property bridge
```

Lean's premise-suggestion APIs even allow application-specific flags/directions.

Source:

https://leanprover-community.github.io/mathlib4_docs/Lean/LibrarySuggestions/Basic.html

### Architectural implication

Index entries should include **usable orientations/roles**, not only theorem identity.

The search compiler can compile several operational views of one semantic theorem while retaining one canonical mathematical authority.

---

## 10. Large watchlists show indexing architecture must evolve with scale

E prover work on watchlists found that a feature-vector approach which worked for small sets became a bottleneck at hundreds of thousands of clauses, motivating split specialized indexes.

Source:

https://www.eprover.org/EVENTS/PAAR-2020/papers2020/PAAR_2020_paper_3.pdf

### Architectural implication

The project should benchmark/reprofile index families as the mathematical universe grows.

A retrieval architecture optimal at 10,000 primitives may be wrong at 100 million.

Index strategy belongs to replaceable operational machinery, not mathematical semantics.

---

## 11. Retrieval success/failure can become search-control knowledge

Repeated campaigns can measure which structural features/index routes led to useful mathematics.

### Architectural implication

Ephemeral/learned search policy may accumulate statistics like:

```text
TheoryProfile P + goal shape G
    -> index route R often yields useful primitive family F
```

These statistics can improve ranking while remaining separate from mathematical truth.

Successful stable policies may later be distilled into deterministic search-policy primitives under the existing distillation rules.

---

## 12. Current mathematical-retrieval hypothesis

```text
PERMANENT MATHEMATICAL GENERATION
    -> compile multiple structural indexes

NEW PROBLEM / SUBGOAL
    -> infer structures / theory / semantic shape
    -> hard admissibility filtering
    -> term/unification/subsumption/morphism retrieval
    -> symbolic premise relevance filtering
    -> optional heuristic/model ranking
    -> bounded relevant context
    -> search/proof/transform campaign

IF STARVED
    -> systematically broaden context under fairness policy
```

This makes “millions of proved formulas at your fingertips” an executable property of the architecture.

---

## 13. New research obligations

1. Compare discrimination, substitution, context, fingerprint, feature-vector, and e-graph indexing for project-specific query families.
2. Study indexes over typed semantic hypergraphs rather than only first-order term trees.
3. Investigate morphism/theory-interface indexing for cross-domain transfer.
4. Study incremental index construction across immutable mathematical generations.
5. Investigate out-of-core indexes for mathematical libraries exceeding RAM.
6. Define deterministic SInE-like relevance expansion over the project's dependency/theory graph.
7. Study proof/certificate indexing by conclusion, assumptions, support, and witness type.
8. Investigate retrieval over canonical equivalence classes so syntactic variants do not fragment the index.
9. Design fairness/broadening policies so aggressive premise selection cannot permanently hide the needed theorem.
10. Benchmark retrieval latency as a constitutional performance metric alongside solver execution latency.
