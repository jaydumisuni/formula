# Research Pass — Tabled Structure Resolution, Incremental Capability Closure, and Generation-Scoped Caching

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates how the unnamed mathematical project can maintain and query a very large graph of derived mathematical capabilities efficiently as new proofs, structure witnesses, theory packages, and assumption worlds appear.

The strongest conclusion is:

> **Accepted mathematical truth should be monotone within an immutable universe generation, while capability inference can use tabled/fixed-point evaluation and generation-scoped caches. Retraction/change belongs to a new generation or a separate candidate view.**

This separation makes large-scale structure inference much more tractable.

---

## 1. Tabled resolution prevents repeated recursive proof work

XSB's SLG resolution records calls/subgoals and their answers in tables. Later occurrences reuse existing answers instead of recursively deriving the same result again.

Sources:

https://xsb.sourceforge.net/shadow_site/manual1/node46.html

https://xsb.sourceforge.net/research.html

For useful bounded classes, tabling turns recursive logic programs that would loop under ordinary Prolog evaluation into terminating computations.

### Architectural implication

A structure goal such as:

```text
HasStructure(D, Field)
```

may recursively depend on many intermediate goals.

The project should memoize canonical subgoals:

```text
Goal G
    -> table G
    -> derive each distinct answer once
    -> reuse across every primitive-applicability query
```

This is essential when thousands of work cells repeatedly ask related structural questions.

---

## 2. Goal canonicalization allows broad cache reuse

Modern trait/tabled solvers canonicalize queries so equivalent goals differing only by local variables/renaming can share cached work.

Chalk similarly requires canonicalized goals before solving.

Sources:

https://rust-lang.github.io/chalk/chalk_solve/solve/trait.Solver.html

https://rust-lang.github.io/chalk/book/clauses/goals_and_clauses.html

### Architectural implication

Structure queries should have a canonical semantic form independent of local work-cell variable names.

For example:

```text
exists K. Field(K) and Module(K,V)
```

should not create a fresh unrelated cache entry every time a campaign renames `K` or `V`.

Canonical goal identity should bind to:

```text
semantic parent/object identities
assumption/world identity
universe/package generation
logical variables modulo alpha-renaming
required assurance class
```

---

## 3. Tabled logic can represent unknown/undefined states, not only true/false

XSB supports well-founded semantics and can handle undefined answers in recursive/non-monotonic reasoning contexts.

Sources:

https://xsb.sourceforge.net/about.html

https://xsb.sourceforge.net/changelog.html

### Architectural implication

The capability resolver should not force every structure goal into Boolean truth.

Possible result classes include:

```text
PROVEN
REFUTED
AMBIGUOUS
UNKNOWN
UNDEFINED_UNDER_CURRENT_RECURSION/NEGATION
RESOURCE_BOUNDED_UNKNOWN
```

These classes may map differently depending on the mathematical logic/package involved.

The project should preserve native logic semantics rather than invent one misleading global Boolean closure.

---

## 4. Incremental tabling updates only affected derived answers

XSB supports incremental tabling: when dynamic base facts change, dependencies are tracked and affected tables are updated/invalidated rather than all tabled queries being recomputed globally.

Source:

https://xsb.sourceforge.net/about.html

Release notes describe dependency-graph support and lazy incremental updating.

Source:

https://xsb.sourceforge.net/changelog.html

### Architectural implication

Candidate mathematical worlds can potentially maintain derived capability closure incrementally:

```text
add assumption/fact F
    -> find dependent tabled goals
    -> update only affected closure
```

Likewise, removing/invalidating candidate facts in a speculative world can trigger localized recomputation.

For the accepted permanent universe, however, see the stronger generation rule below.

---

## 5. Accepted universe generations should be immutable

Earlier package/change research already separates accepted generations from candidate updates.

This pass strengthens that boundary.

Within accepted generation `G`:

```text
accepted fact set
accepted package graph
accepted certificate lineage
```

should be immutable.

New accepted mathematics produces:

```text
G -> G+1
```

rather than deleting/mutating semantic truth in-place.

### Architectural implication

Capability inference **within one accepted generation** can exploit monotone assumptions aggressively:

```text
facts only accumulate during generation construction
closure is frozen at promotion
queries cache against generation digest
```

If a later correction removes a false theorem, that happens in a corrected new generation whose semantic diff explicitly invalidates dependent support.

The historical generation remains an immutable evidence artifact, not silently rewritten history.

---

## 6. Semi-naive fixed-point evaluation avoids repeated derivation

Datalog engines use semi-naive evaluation: each iteration combines rules with newly derived `delta` facts rather than recomputing every rule over the entire accumulated database.

Representative Rust systems:

https://docs.rs/datafrog/latest/datafrog/

https://docs.rs/oxirs-rule/latest/oxirs_rule/datalog/index.html

### Architectural implication

For monotone structure implications such as:

```text
Field(D) -> IntegralDomain(D)
IntegralDomain(D) -> Domain(D)
...
```

an accepted-generation capability closure can be compiled bottom-up efficiently:

```text
new certified witness
    -> delta facts
    -> fire affected implications
    -> continue until fixed point
```

This is preferable to recursively proving every obvious derived property from scratch on each query.

---

## 7. Top-down tabling and bottom-up closure are complementary

Bottom-up closure is excellent when many derived facts are frequently queried.

Top-down tabled resolution is excellent when the possible closure is enormous but only a small subset is relevant to current goals.

### Architectural implication

A hybrid resolver is likely stronger:

```text
EAGER / MATERIALIZED
    cheap high-value common structure closure

LAZY / TABLED
    rare expensive structure goals

ON DISCOVERY
    promote repeatedly demanded derived facts into compiled indexes
```

This resembles database materialized-view selection and can itself be search-economy optimized.

---

## 8. Subsumptive tabling can reuse broad queries for narrower ones

XSB supports call-subsumptive tabling, allowing answers computed for a more general query to be reused for more specific calls.

Source:

https://xsb.sourceforge.net/about.html

### Architectural implication

Suppose the system has already evaluated:

```text
all D such that Field(D)
```

A later query about one concrete `D0` may reuse that table rather than launch another proof search.

Likewise, a general morphism or parent-compatibility query may serve many concrete work cells.

This may become important in global campaign phases that repeatedly ask related structure questions.

---

## 9. Differential dataflow handles insertions and removals in recursive computations

Differential Dataflow maintains collection computations, including iterative/fixed-point computations, as input records are added or removed.

Source:

https://docs.rs/differential-dataflow/latest/differential_dataflow/

Related research develops mathematical derivatives of fixpoints for incremental Datalog-style computation.

Source:

https://arxiv.org/abs/1811.06069

### Architectural implication

Differential/incremental dataflow is especially relevant for **active candidate worlds** where assumptions may be added/removed frequently.

Possible split:

```text
ACCEPTED GENERATION
    frozen monotone compiled closure

CANDIDATE / ASSUMPTION WORLD
    differential incremental view over accepted base
```

This avoids imposing high dynamic-update overhead on ordinary stable mathematical queries while retaining efficient speculative reasoning.

---

## 10. Equality/canonicalization views should remain separate from monotone capability truth

Earlier substrate research found that contextual equality can collapse previously distinct canonicalized records, making materialized canonical views non-monotonic.

This pass reinforces the separation:

```text
MONOTONE CAPABILITY FACTS
    persistent semantic claims/witnesses

CONTEXTUAL CANONICALIZATION
    rebuildable/indexed equality views
```

### Architectural implication

A bottom-up Datalog-style capability closure should operate on stable semantic identities, not on mutable canonical representatives whose identity may collapse after new equalities.

Equality views can accelerate matching but must not erase source fact identity.

---

## 11. Derived capability caches need exact invalidation lineage

Chalk notes cached solver state is valid only when the program clauses remain unchanged.

Source:

https://rust-lang.github.io/chalk/chalk_solve/solve/trait.Solver.html

### Architectural implication

A capability cache entry should bind to:

```text
universe generation digest
assumption/world digest
structure-rule-set digest
query digest
certificate/assurance policy digest
```

No long-lived cache can survive a semantic rule/package change merely because the query text is identical.

---

## 12. Materialization itself is an optimization problem

A huge universe may contain millions of derivable structure facts, many never queried.

### Architectural implication

The system should distinguish:

```text
DERIVABLE
    can be proved from accepted rules

MATERIALIZED
    cached/precomputed for runtime efficiency
```

Repeated query frequency, unlock value, proof cost, memory footprint, and invalidation cost can determine which facts/indexes deserve materialization.

The search economy can therefore optimize **knowledge closure layout**, not just problem-solving tasks.

---

## 13. Proof evidence should survive cache eviction

Search/materialization caches are performance artifacts.

A certified structure witness is mathematical evidence.

### Architectural implication

Evicting:

```text
cached goal result
materialized closure table
index
```

must never delete the durable certificate/proof lineage establishing an accepted mathematical fact.

If the fact is needed again, the system can reconstruct or revalidate its derived cache from durable evidence.

This follows the broader separation between permanent mathematical truth and disposable operational state.

---

## 14. Current capability-closure hypothesis

```text
DURABLE ACCEPTED THEORY/PACKAGE GRAPH (generation G)
        |
        +-- explicit certified structure facts
        +-- certified implication rules
        |
        v
GENERATION-SCOPED CAPABILITY ENGINE
        |
        +-- semi-naive materialized closure
        +-- tabled top-down goal solver
        +-- canonical goal cache
        +-- subsumptive/general query reuse
        |
        v
COMPILED STRUCTURE CAPABILITY GRAPH
        |
        v
PRIMITIVE APPLICABILITY / SEARCH COMPILER

Candidate world W over G:
        G + speculative facts/assumptions
        -> incremental/differential capability view
```

This is a research hypothesis, not a frozen implementation choice.

---

## 15. New research obligations

1. Compare SLG/tabling, semi-naive Datalog, egglog, and differential dataflow for the same structure-inference workloads.
2. Study proof-producing tabling so derived capability answers retain compact explanation DAGs.
3. Determine which structure rules can be safely compiled into monotone closure and which require richer negation/non-monotonic semantics.
4. Investigate canonical/subsumptive query identity across heterogeneous mathematical logics.
5. Define generation/world cache-key semantics precisely.
6. Study incremental maintenance of ambiguity sets and alternative structure witnesses, not only Boolean facts.
7. Investigate materialized-view selection for frequently used mathematical structure facts.
8. Study distributed capability closure only after local performance is understood; do not assume Ptah is needed.
9. Investigate how theory-morphism transport can bulk-populate capability closure without materializing every transported theorem eagerly.
10. Determine how a new proof or package promotion computes the exact delta closure before generation acceptance.
11. Study compact provenance/explanation storage for heavily shared recursive capability derivations.
12. Investigate concurrency control so multiple work cells can query/build candidate-world closure deterministically.
13. Define when a candidate-world differential view can be frozen/promoted into a new accepted generation closure.
14. Study garbage collection of derived/materialized structure facts made redundant by stronger promoted primitives.
15. Investigate whether capability closure itself can produce reusable certificates proving closure completeness for bounded theory fragments.
