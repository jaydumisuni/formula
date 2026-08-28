# Research Pass — Mathematical Search Economy and Compute Allocation

**Date:** 2026-08-28  
**Status:** RESEARCH supplement  

This note investigates how the unnamed mathematical project should allocate finite CPU/GPU/search resources across many candidate mathematical investigations without collapsing into either brute force or a brittle single heuristic.

The central research question is:

> Given thousands or millions of mathematically admissible next computations, transformations, proof attempts, representations, solver routes, or work cells, which ones should receive compute now, and how should newly obtained mathematical evidence change that allocation?

---

## 1. Universal search provides a safety floor, not a practical scheduler

Levin-style universal search gives a useful constitutional idea: if solutions are efficiently verifiable, computation can be shared across candidate procedures according to prior weights while retaining asymptotic competitiveness against the best procedure in the reference class, up to representation-dependent constants.

Recent review:

https://pmc.ncbi.nlm.nih.gov/articles/PMC13114910/

Classic overview:

https://www.scholarpedia.org/article/Universal_search

### Architectural implication

The project should preserve a non-starvation / fallback search mechanism so that a currently unfashionable but valid search family is not permanently excluded by a bad heuristic.

However, pure universal search is not practical as the primary scheduler because the constant factors depend strongly on representation and prior assignment.

Possible role:

```text
heuristic / structural scheduler
    +
small guaranteed exploration floor
```

rather than universal search as the main engine.

---

## 2. Value of computation gives the correct metalevel question

Rational metareasoning treats a computation itself as an action whose value depends on its expected improvement to the final decision or result minus its resource/time cost.

Foundational sources:

https://doi.org/10.1016/0004-3702(91)90015-C

https://www.microsoft.com/en-us/research/publication/ideal-partition-resources-metareasoning/

Mathematical-theorem-proving application:

https://www.microsoft.com/en-us/research/publication/reasoning-metareasoning-and-mathematical-truth-studies-of-theorem-proving-under-limited-resources/

### Architectural implication

The scheduler should not optimize raw throughput such as:

```text
operations / second
```

but something closer to:

```text
expected mathematical value
---------------------------
compute / memory / latency cost
```

where mathematical value may include:

- probability of proving/refuting a high-value claim;
- expected reduction in candidate-space size;
- expected information gain over unresolved worlds;
- probability of exposing a counterexample;
- expected dependency unlocks;
- expected discovery of a reusable primitive;
- proof/certificate value;
- reduction in future search cost;
- novelty/generalization potential.

This is a research target, not yet a frozen utility function.

---

## 3. Active learning provides a direct information-gain analogue

Active experimental-design methods choose the next experiment based on expected reduction of uncertainty over the candidate-model space. An experiment is valuable when its possible outcomes strongly partition the remaining candidates.

Representative review:

https://pmc.ncbi.nlm.nih.gov/articles/PMC5453429/

### Mathematical analogue

Suppose unresolved candidate constructions are:

```text
C1 C2 C3 ... Cn
```

A candidate computation `E` may have possible outcomes that partition this set.

A high-value computation is one expected to split the remaining hypothesis space sharply, rather than returning an outcome compatible with nearly everything.

Possible metaprimitive:

```text
choose_discriminator(candidate_worlds, available_tests)
```

This is particularly relevant for:

- counterexample selection;
- assumption-world branching;
- deciding which numerical experiment to run;
- selecting discriminating finite cases;
- selecting theorem lemmas to attempt;
- deciding which representation test to run next.

---

## 4. Proof search shows that local choice dominates runtime

State-of-the-art saturation theorem provers such as E and Vampire rely on repeated local selection among huge sets of available clauses/inferences.

E's given-clause algorithm maintains processed and unprocessed sets and uses heuristic evaluation functions, multiple priority queues, and weighted round-robin selection.

Source:

https://github.com/eprover/eprover/blob/master/DOC/eprover.tex

Historical explanation:

https://link.springer.com/article/10.1007/s10817-022-09628-0

### Architectural implication

The project should expect mathematical campaigns to be dominated by repeated **frontier-selection decisions** rather than one up-front strategy choice.

A work-cell scheduler may therefore need:

- several competing priority queues;
- structure-specific scoring functions;
- fairness/age mechanisms;
- periodic strategy rotation;
- dynamic reprioritization when new facts arrive.

One globally ranked queue is likely too brittle.

---

## 5. Different problem instances need different solvers/strategies

SATzilla established that there is no single dominant SAT solver across all instances; per-instance feature extraction and algorithm portfolios can outperform selecting one globally best solver.

Sources:

https://new.aaai.org/Library/JAIR/Vol32/jair32-014.php

https://www.cs.ubc.ca/labs/algorithms/Projects/SATzilla/

Automated theorem provers likewise use schedules of diverse strategies, and recent work continues to study strategy discovery and scheduling for Vampire/E-style provers.

Source:

https://link.springer.com/chapter/10.1007/978-3-031-63498-7_12

### Architectural implication

The project's earlier `Theory Profile` concept becomes even more important.

Search allocation should be conditioned on features of the mathematical region, for example:

```text
termination/confluence properties
algebraic structure
symmetry profile
sparsity
finite/infinite domain
branching factor
available certificate families
previous campaign outcomes
representation complexity
known solver strengths
```

The scheduler should choose **formations**, not one universal solver.

---

## 6. Multi-armed-bandit methods are useful but insufficient alone

Contextual multi-armed-bandit systems have been successfully applied to selecting SMT solvers and scheduling/restart strategies.

MedleySolver frames SMT solver selection as a modified/contextual bandit problem and selects sequences of solvers rather than only one solver.

Source:

https://federico.morarocha.ca/thesis/

Recent 2026 SAT work adapts bandit reward/selection to variable-duration restart strategies, showing that even the time scale of credit assignment matters.

Source:

https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.CP.2026.39

### Architectural implication

Bandit-style exploration/exploitation can be one scheduler component, but the project must not flatten mathematical work into generic stochastic arms.

Work cells differ structurally in:

- duration;
- verifiability;
- expected information gain;
- dependency unlocks;
- proof value;
- ability to create permanent pruning knowledge;
- ability to generate reusable primitives.

Therefore any bandit layer should consume mathematics-specific reward signals rather than define the search economy itself.

---

## 7. Conflict learning gives a stronger notion of useful failure

Conflict-Driven Clause Learning (CDCL) does not merely backtrack after failure. A conflict produces a learned clause that prevents the same incompatible region of assignment space from being explored again.

Sources:

https://collaborate.princeton.edu/en/publications/chapter-4-conflict-driven-clause-learning-sat-solvers/

https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0272967

CEGIS/abstraction-refinement systems similarly turn counterexamples into constraints that prune entire families of future candidates.

Example:

https://link.springer.com/chapter/10.1007/978-3-031-90653-4_8

### Architectural implication

The project should score a failed computation partly by whether it creates reusable **nogood/pruning knowledge**.

A failed work cell that proves:

```text
all constructions containing structural pattern P
under assumptions A
cannot satisfy target T
```

may be more valuable than a successful cell that merely evaluates one candidate.

This suggests a first-class artifact family:

```text
Search Constraint / Mathematical Nogood
```

with scope, assumptions, derivation/certificate, and applicability metadata.

---

## 8. Exploration must be protected against heuristic lock-in

Theorem provers deliberately combine heuristic priority with fairness mechanisms. For example, E can mix priority queues via weighted round-robin, and classic given-clause systems mix best-first and age/FIFO selection.

This prevents a heuristic from permanently starving clauses that later turn out to be necessary.

### Architectural implication

The mathematical search economy should likely reserve explicit budgets for:

```text
exploit high-value routes
explore under-sampled routes
revisit previously weak routes after theory changes
maintain universal/non-starvation floor
```

A mathematically promising route may receive most compute without being allowed to monopolize the entire campaign.

---

## 9. Search scheduling is itself expensive and must be bounded

Metareasoning consumes resources that could otherwise be spent doing mathematical work. The classic metareasoning-partition problem explicitly studies this tradeoff.

### Architectural implication

The scheduler should not continuously solve an expensive global optimization problem merely to decide where to spend the next microsecond.

Possible multi-timescale architecture:

```text
nanosecond/microsecond scale
    -> local deterministic queue policy

millisecond/second scale
    -> work-cell/frontier reprioritization

campaign-event scale
    -> recompute formation and representation strategy

major discovery/proof/refutation
    -> rebuild relevant search economy
```

The exact timescales remain open.

---

## 10. Current search-economy hypothesis

A future mathematical campaign scheduler may combine several layers:

```text
THEORY PROFILE
    -> which search families are admissible/promising

STRUCTURAL ROUTER
    -> initial formation / representation / solver selection

VALUE-OF-COMPUTATION ESTIMATOR
    -> expected mathematical value per resource cost

INFORMATION-GAIN / DISCRIMINATION LAYER
    -> select tests likely to fragment candidate worlds

PORTFOLIO / BANDIT LAYER
    -> adapt allocation using observed campaign performance

FAIRNESS / UNIVERSAL FLOOR
    -> prevent permanent heuristic starvation

CONFLICT / NOGOOD LEARNING
    -> failures permanently prune future search

EVENT-DRIVEN RECOMPILATION
    -> discoveries/refutations change the search landscape
```

This is not a frozen design. It is the strongest evidence-backed synthesis from this research pass.

---

## 11. New research obligations

1. Define mathematics-specific notions of information gain when candidate spaces are symbolic/infinite rather than finite probabilistic model sets.
2. Investigate proof-number / disproof-number search and AND/OR search for dependency/proof graphs.
3. Investigate clause activity measures such as VSIDS/LBD as analogues for prioritizing recently useful mathematical constraints.
4. Investigate algorithm-portfolio scheduling where jobs have variable runtimes and produce structured side effects such as proofs, counterexamples, or new primitives.
5. Investigate how to score **future reusable value** of a computation, not only probability of solving the current problem.
6. Investigate restart policies for mathematical search and when accumulated search state should be retained versus discarded.
7. Determine how the scheduler can remain deterministic/replayable while using empirical performance statistics or stochastic exploration policies.
8. Define certificate/provenance requirements for learned search constraints so invalid failures cannot poison future campaigns.
