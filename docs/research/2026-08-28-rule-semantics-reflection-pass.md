# Research Pass — Rule Semantics, Constraint Propagation, Rewriting Logic, and Reflective Metamathematics

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates whether the unnamed mathematical project can use one declarative rule semantics for mathematical propagation, rewriting, search strategies, theory transformation, and metaprimitives.

The result is more nuanced:

> **Constraint rules, rewrite rules, and metatheory can share a broad semantic family, but they should not be collapsed into one operational engine.**

CHR is especially strong for constraint-store propagation and concurrent local deduction. Rewriting logic/Maude is stronger for general state transitions, rules modulo equations, strategies, theories-as-data, and reflection. The likely project architecture should preserve a common mathematical semantic boundary while allowing multiple certified executable rule fabrics.

---

## 1. Constraint Handling Rules are a mature declarative propagator language

Constraint Handling Rules (CHR) are guarded multiset-rewrite rules designed for defining user constraints and solvers.

Rules support:

- simplification: replace constraints with logically equivalent simpler constraints;
- propagation: add logically implied constraints;
- simpagation: retain part of a matching constraint set while simplifying another part.

Sources:

https://doi.org/10.1016/S0743-1066(98)10005-5

https://dtai.cs.kuleuven.be/projects/CHR/about.shtml

### Architectural implication

The earlier `relational-propagation-pass` does not require every propagator to be opaque native code.

A family of mathematical narrowing operations may be expressible declaratively as rules over a constraint store:

```text
R1, R2, guard
    ==> derived restriction

R3, R4
    <=> canonical simplified constraints
```

This is attractive for relations whose deductions are naturally local, incremental, and compositional.

---

## 2. Confluence can make rule scheduling semantically irrelevant

For terminating CHR programs, confluence ensures that different valid orders of rule application reach the same final result. Observable-confluence work weakens the requirement to states reachable from declared initial goals and gives decidable tests in useful classes.

Sources:

https://research.monash.edu/en/publications/observable-confluence-for-constraint-handling-rules/

https://doi.org/10.1016/S0743-1066(98)10005-5

### Architectural implication

A rule family may advertise a structural property such as:

```text
rule_family:
    termination: PROVEN
    confluence: PROVEN
```

When both properties hold for the relevant scope, the scheduler gains considerable freedom:

```text
fire applicable rules in parallel / varying orders
    -> same certified normal result
```

This is highly relevant to Mathematical Work Cells and multicore execution.

The project should not infer confluence merely from empirical agreement.

---

## 3. CHR has inherent parallel semantics, but operational restrictions matter

CHR literature provides parallel, concurrent, and distributed semantics with soundness correspondences to sequential computations. Implementations and prototypes have targeted multicore software, GPU execution, and FPGA/hardware compilation.

Sources:

https://arxiv.org/abs/1703.10959

https://arxiv.org/abs/1808.07788

https://www.research.unipd.it/handle/11577/3567026

### Architectural implication

The project can potentially compile certified rule families into:

```text
sequential CPU
multicore CPU
GPU subset
specialized hardware later
```

without changing their semantic identity.

However, GPU/hardware subsets impose representation/memory restrictions. Execution placement remains a realization concern, not a semantic law.

---

## 4. Rule scheduling/indexing is a major performance problem

Efficient CHR compilation relies on:

- constraint indexing;
- partner lookup order / join ordering;
- functional dependencies;
- rule priorities;
- refined operational semantics;
- event/reactivation policies.

Sources:

https://www.cambridge.org/core/journals/theory-and-practice-of-logic-programming/article/logical-algorithms-meets-chr-a-metacomplexity-result-for-constraint-handling-rules-with-rule-priorities/9BCC6DFCF47EA535BB0385855C93963C

https://citeseerx.ist.psu.edu/document?doi=82eb6e00c043b3eb4d94156b21add7388d48cd0d&repid=rep1&type=pdf

### Architectural implication

Declarative mathematics does not imply interpreted slowness.

A rule compiler can specialize:

```text
which constraints wake a rule
how partners are indexed
which match occurs first
which joins are materialized
which rule priority applies
```

This fits the project's self-specialization hypothesis: high-level mathematics remains declarative while operational machinery is compiled away.

---

## 5. Rewriting logic provides a broader state-transition semantics

Maude is based on rewriting logic, where rules are simultaneously:

- local state transitions computationally;
- logical inference rules semantically.

Rewriting occurs modulo an equational theory, including common algebraic laws such as associativity, commutativity, identity, and idempotency.

Sources:

https://maude.cs.illinois.edu/maude1/manual/maude-manual-html/maude-manual_3.html

https://maude.cs.illinois.edu/overview

### Architectural implication

For mathematical structures involving:

- concurrent state;
- nondeterministic transitions;
- symbolic operational semantics;
- transformations modulo algebraic laws;
- theories and strategies;

rewriting logic is a stronger donor than CHR alone.

A future semantic relation may therefore compile into a CHR-like propagation regime in one fragment and a rewrite-theory regime in another.

---

## 6. Reflection makes theories and strategies manipulable mathematical objects

Rewriting logic is reflective: a finitely presented rewrite theory and its terms can be represented as data inside a universal rewrite theory. Maude exposes this through `META-LEVEL`; modules, terms, rules, and strategies can be reified and manipulated at the metalevel.

Sources:

https://maude.cs.illinois.edu/maude1/manual/maude-manual-html/maude-manual_19.html

https://maude.cs.illinois.edu/maude1/manual/maude-manual-html/maude-manual_20.html

https://doi.org/10.1016/S1571-0661(05)80020-9

### Architectural implication

The project can treat not only values and formulas but also **mathematical machinery itself** as semantic data:

```text
object
relation
rewrite rule
rule family
theory
strategy
completion procedure
search policy
```

This provides a rigorous analogue for the earlier `metaprimitive` concept.

However:

> reifying a rule/theory does not authorize changing trusted mathematics.

Metalevel transformations remain candidate artifacts until independently certified.

---

## 7. Reflective towers are real, but naive reflection is expensive

Maude supports arbitrary levels of reflective interpretation in principle, but its documentation explicitly notes that naive reflection can be expensive in time and memory. The implementation uses built-in descent functions and optimized metalevel operations to lower reflective computations to efficient lower-level execution.

Sources:

https://maude.cs.illinois.edu/maude1/manual/maude-manual-html/maude-manual_19.html

https://maude.cs.illinois.edu/maude1/manual/maude-manual-html/maude-manual_21.html

### Architectural implication

The project should not execute routine mathematics through an indefinitely nested meta-interpreter.

Preferred pattern:

```text
reflective discovery / theory transformation
    -> freeze candidate theory/strategy
    -> certify
    -> specialize / lower
    -> native residual implementation
```

Reflection belongs primarily in discovery, analysis, and controlled evolution. Mature primitives should descend to cheaper realizations.

---

## 8. Strategy should be separate from mathematical rules

Maude's strategy language exists precisely because unrestricted nondeterministic rule application can explode or choose undesirable paths. Strategy is represented separately from the underlying rewrite theory and can itself be manipulated at the metalevel.

Sources:

https://doi.org/10.1016/j.jlamp.2023.100887

https://maude.lcc.uma.es/maude-manual/maude-manualch17.html

https://doi.org/10.1016/j.jlamp.2021.100728

### Architectural implication

The project should separate:

```text
MATHEMATICAL RULE
    semantic transformation / inference

SEARCH STRATEGY
    when, where, and in what order to apply rules
```

A better strategy can improve runtime without changing mathematical truth.

This directly matches the permanent-mathematics / disposable-search-state split already identified in the search-economy pass.

---

## 9. Strategies themselves can be synthesized, but do not gain truth authority

2025 work on automatic confluence proving shows that automated strategy invention can outperform human-designed prover strategies and prove/disprove instances previously unresolved by the baseline automated system.

Source:

https://www.ijcai.org/proceedings/2025/526

The reported system uses AI/learning, which is optional from this project's perspective.

### Architectural implication

A search strategy may be discovered by:

- deterministic search;
- portfolio optimization;
- evolutionary methods;
- models;
- human design.

Its source is irrelevant to mathematical authority.

Only the proof/certificate produced by the underlying mathematical analyzer establishes the property.

---

## 10. Rules can also be synthesized and verified

Program-synthesis work on Halide's term rewriting system generated thousands of candidate rewrite rules while checking semantic equality and constraining rules by a reduction order to preserve useful direction/termination behavior.

Source:

https://inst.eecs.berkeley.edu/~cs294-260/sp24/2024-01-29-halide-rewriting

Ruler/equality-saturation work independently supports automatic rewrite-rule discovery.

### Architectural implication

The project may eventually discover not only mathematical formulas but **new transformation laws**:

```text
candidate rule L -> R
    -> semantic equivalence proof
    -> termination/orientation analysis
    -> confluence/coherence impact analysis
    -> bounded deployment/canary
    -> promoted rewrite primitive
```

This is a concrete self-expansion mechanism at the transformation-language level.

---

## 11. Theory-transforming tools can themselves be executable formal systems

Maude's confluence, termination, completion, coherence, and theorem-proving tools have historically been implemented in Maude by exploiting reflection: formal inference systems are represented as executable rewrite theories acting on reified target theories.

Sources:

https://maude.cs.illinois.edu/maude1/tools/

https://maude.cs.illinois.edu/maude1/tools/coherence/

https://maude.cs.illinois.edu/papers/abstract/Dknuthbendix_2000.html

### Architectural implication

This provides a real model for metaprimitives such as:

```text
analyze(theory)
complete(theory)
derive_strategy(theory)
transform(theory)
```

where the transformation procedure itself is mathematically specified/executable.

The project can therefore have a **metamathematical work layer** without requiring human-like cognition.

---

## 12. Structural proofs can grant parallelization and canonicalization privileges

Current research now suggests that Theory Profile properties should affect execution permissions.

For example:

```text
termination = PROVEN
confluence = PROVEN
```

may authorize aggressive parallel rule firing and canonical normalization.

```text
termination = UNKNOWN
confluence = UNKNOWN
```

may require bounded/strategic execution and explicit cycle/resource limits.

```text
coherence = PROVEN
```

may authorize safe interaction between equations and rewrite rules under the declared theory.

### Architectural implication

Theory Profile is not merely descriptive metadata.

It can become an **execution-capability contract**.

---

## 13. Current rule-fabric hypothesis

The strongest current synthesis is not one universal rule engine.

Instead:

```text
SEMANTIC MATHEMATICAL RELATION / THEORY
        |
        +-- propagation rule realization (CHR-like)
        +-- equational normalization realization
        +-- rewrite-theory realization (Maude-like semantics)
        +-- e-graph equality realization
        +-- specialized solver realization
        +-- native compiled residual realization
```

Each realization declares:

- exact semantic scope;
- soundness/completeness properties;
- termination/confluence/coherence profile where meaningful;
- proof/certificate route;
- execution cost model;
- supported parallelism;
- assumption/world identity.

The semantic object remains authoritative; rule engines are replaceable realizations.

---

## 14. Metamathematical evolution must use promotion gates

Reflection makes self-manipulation possible, but unrestricted self-modification would destroy the project's truth boundary.

A proposed evolution loop should therefore resemble:

```text
active certified theory T
    -> reflective/meta search proposes T'
    -> isolate T' in candidate world
    -> analyze termination/confluence/coherence/decidability changes
    -> regression + adversarial testing
    -> independently certify new/changed rules
    -> certify inherited theorem validity or invalidate affected descendants
    -> explicit promotion
    -> T' becomes active generation
```

Search machinery may freely invent candidates.

Only certified promotion changes the permanent mathematical universe.

This is the mathematical analogue of governed capability evolution already present elsewhere in the THETECHGUY ecosystem.

---

## 15. New research obligations

1. Investigate whether CHR-style rules can be compiled automatically from the relational/propagator semantics proposed in `relational-propagation-pass`.
2. Study compositional confluence: when separately certified rule families remain confluent after combination.
3. Study modular termination/coherence criteria so primitive packages can be combined without re-proving the entire universe from zero.
4. Investigate rule-explanation/certificate generation for CHR-style propagation.
5. Compare CHR, rewriting logic, egglog, and Datalog as execution fabrics over identical small mathematical relations.
6. Study metalevel theory transformation with independently checkable transformation certificates.
7. Investigate proof/certificate formats for strategy correctness and strategy-dependent termination.
8. Study compilation of reflective/metalevel mathematics into native residual rule engines.
9. Investigate safe dynamic addition/removal of rewrite/propagation rules under versioned assumption worlds.
10. Determine which Theory Profile properties should grant which execution privileges.
11. Study modularity failures: termination/confluence properties are not automatically preserved when theories are combined.
12. Investigate how rule-family promotion interacts with dependency invalidation and transported theorems.
13. Study metaprimitive synthesis without model dependence, including enumeration, CEGIS, equality saturation, completion, and anti-unification.
14. Determine whether a small formally specified `rule kernel` is useful, or whether domain-native engines plus certificate envelopes remain the better trust boundary.
