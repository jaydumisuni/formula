# Unnamed Mathematical Project — Research Checkpoint

**Date:** 2026-08-28  
**Status:** RESEARCH — architecture hypotheses only; not a frozen product design or implementation roadmap  
**Repository name:** `formula` is temporary and must not be treated as the final product name  

## 0. Purpose of this checkpoint

This file preserves the research state reached before implementation planning.

The project began from a fictional inspiration: a mathematical breakthrough whose useful power is not dependent on an AI model. The engineering goal is not to recreate a fictional character, nor to build another AI mathematician. The goal is to investigate whether a new deterministic mathematical problem-solving substrate can be built from real mathematics, program synthesis, proof systems, compilers, search systems, and the existing THETECHGUY ecosystem.

The central constraint is:

> **Models may borrow mathematical power. Mathematical truth must not depend on models.**

The system may use models as optional search accelerators, hypothesis generators, translators, or human-facing explainers. A model must never become mathematical authority merely because it proposes something convincing.

This checkpoint intentionally preserves both strong findings and unresolved questions. Future work must not silently convert hypotheses into facts.

---

# 1. Current vision

The project should not be designed as a conventional computer algebra system, theorem prover, solver, chatbot, or programming language.

The emerging target is broader:

> **A self-expanding deterministic mathematical problem-solving architecture that searches mathematical state-space, discovers or composes useful mathematical structures, independently establishes what those structures guarantee, compiles accepted structures into efficient reusable primitives, and uses the enlarged primitive set to attack progressively harder problems.**

A conventional formula may be one possible object in the system, but not the only one.

The system must be able to represent mathematical capability that is naturally expressed as:

- equations;
- relations;
- transformations;
- recurrences;
- graphs and hypergraphs;
- constraint systems;
- state machines;
- synthesized programs;
- algorithms;
- proofs and certificates;
- multi-input/multi-output constructions;
- structures too large or unnatural for elegant human notation.

Human readability is a presentation concern. It must not limit mathematical capability.

---

# 2. Key correction: mathematics does not have to look like human mathematics

An early working idea was an **Executable Mathematical Construction**: a formula whose operational form is a program.

That remains useful, but current research suggests it may still be too restrictive as the deepest internal model.

A discovered mathematical structure may exist canonically as a relation or semantic graph, while a program is only one executable realization of it.

Therefore:

```text
MATHEMATICAL SEMANTICS
        !=
PROGRAM REPRESENTATION
```

A single semantic object may have multiple realizations:

```text
semantic mathematical relation
        |
        +-- symbolic realization
        +-- exact CPU realization
        +-- SIMD realization
        +-- GPU realization
        +-- inverse/search realization
        +-- proof/checker realization
```

The mathematics should remain identifiable independently of any one implementation.

---

# 3. Relational semantics is now a major hypothesis

The ordinary function model is directional:

```text
x -> f -> y
```

Relational programming suggests a more general semantic model:

```text
R(x, y, z, ...)
```

The same mathematical relation can then be queried in different directions:

```text
x known, y unknown        -> calculation

y known, x unknown        -> inversion

x/y partly known          -> constraint solving

program unknown           -> synthesis

proof unknown             -> proof search
```

This does **not** mean a generic relational interpreter should be the final runtime.

Research into miniKanren and multi-stage relational programming shows the important distinction:

> **Relational semantics can define the meaning once; direction-specific execution plans should be specialized and compiled separately.**

Generic relational search can diverge or perform badly depending on search order. Therefore the current hypothesis is:

```text
RELATIONAL SEMANTICS
        |
        +-- specialize(query direction, known values, domain, assumptions)
                |
                +-- fast forward implementation
                +-- inverse/search implementation
                +-- enumerator
                +-- synthesis implementation
                +-- proof-oriented implementation
```

This is a research direction, not yet a frozen design.

Relevant donors:

- miniKanren relational interpreters / synthesis
- multi-stage relational programming (PLDI 2025)
- Futamura projections / partial evaluation
- Souffle staged Datalog compilation

---

# 4. Mathematical universe: strongest current substrate hypothesis

A plain AST is almost certainly too narrow.

A plain graph is likely too weak.

A plain e-graph is excellent for equivalence saturation but too narrow for the whole mathematical universe.

A plain hypergraph supports rich multi-input/multi-output structure but does not by itself provide the full equivalence, world, relational, and provenance machinery needed.

Current research points toward a **layered semantic substrate**.

## 4.1 Semantic mathematical hypergraph

The base structure may need to represent:

- mathematical objects;
- relations;
- transformations;
- multi-input operations;
- multi-output operations;
- composition;
- parallel composition;
- feedback/recurrence;
- structure-preserving mappings;
- alternative representations of the same semantic object.

The strongest donor found is:

**Equivalence Hypergraphs: DPO Rewriting for Monoidal E-Graphs (LICS 2025)**  
https://doi.org/10.1109/LICS65433.2025.00023

This work generalizes e-graphs toward hypergraph/monoidal structure and gives a formal rewriting foundation.

This does not prove it is sufficient for this project, but it proves that equality-saturation-like semantics and hypergraph composition can be joined mathematically.

## 4.2 Equivalence and world structure

Recent e-graph work adds several capabilities needed by the project:

- **Dis/Equality Graphs (POPL 2025)** — native disequality alongside equality.
- **Slotted E-Graphs (2025)** — improved support for variables and binding.
- **Versioned E-Graphs (PLDI 2026)** — multiple equality/assumption worlds sharing common structure.
- **Semantic E-Graphs (PLDI 2026)** — domain semantic values can participate directly in equivalence rather than relying only on syntactic rewrite rules.

Versioned e-graphs are especially important because the project should be able to explore different mathematical worlds simultaneously:

```text
World A: assume H
World B: assume not-H
World C: weaken assumption A
World D: change representation
```

without cloning the entire shared mathematical universe for every branch.

Relevant source:

Versioned E-Graphs (PLDI 2026)  
https://pldi26.sigplan.org/details/pldi-2026-papers/6/Versioned-E-Graphs

Semantic E-Graphs (PLDI 2026)  
https://pldi26.sigplan.org/details/pldi-2026-papers/56/Improving-Equality-Saturation-for-EDA-via-Semantic-E-Graphs

## 4.3 Relational / deductive fabric

The mathematical universe contains relations that are not simply equalities:

```text
prime(x)
preserves(T, I)
requires(T, A)
refutes(C, E)
depends_on(A, B)
solves(C, P)
derived_from(C, X)
```

`egglog` is an important donor because it combines equality saturation with Datalog-style relational reasoning and fixed-point deduction.

Relevant source:

egglog — Better Together: Unifying Datalog and Equality Saturation  
https://doi.org/10.1145/3591239

## 4.4 Provenance and certification fabric

Facts and derived structures should not carry only opaque logs.

Datalog provenance research shows derivations can carry algebraic provenance that records alternative support paths.

For example, if result `R` is supported by either:

```text
(A and B) OR (C and D)
```

then invalidating `A` should not automatically invalidate `R` if `(C and D)` remains sufficient.

This is richer than a simple dependency list and may become important for exact mathematical invalidation.

Relevant direction:

- semiring provenance for Datalog
- determination provenance for multiple admissible outcomes

## 4.5 Current layered substrate hypothesis

The current model is therefore:

```text
+---------------------------------------------+
| SEMANTIC MATHEMATICAL HYPERGRAPH            |
| objects / relations / transformations       |
| multi-input / multi-output composition      |
+---------------------+-----------------------+
                      |
+---------------------v-----------------------+
| EQUIVALENCE + WORLD STRUCTURE               |
| semantic equivalence / disequality          |
| assumption versions / representations       |
+---------------------+-----------------------+
                      |
+---------------------v-----------------------+
| RELATIONAL / DEDUCTIVE FABRIC               |
| facts / constraints / implications          |
| fixed-point reasoning / relational queries  |
+---------------------+-----------------------+
                      |
+---------------------v-----------------------+
| PROVENANCE + CERTIFICATION FABRIC           |
| derivations / assumptions / certificates    |
| counterexamples / independent proof         |
+---------------------------------------------+
```

This is a **research hypothesis**, not a frozen architecture.

The next research must determine whether these four planes can coexist efficiently or whether one or more must remain separate services/representations.

---

# 5. Equality saturation is powerful but must be bounded

Equality saturation is a major donor, but the project must not simply saturate all known mathematics.

General e-graph extraction can be NP-hard, and equality-saturation termination is itself nontrivial.

Therefore the project should likely use **goal-directed, bounded local saturation**:

```text
goal
  -> select relevant mathematical region
  -> select rewrite families
  -> define cost / information objective
  -> allocate budget
  -> saturate locally
  -> extract / branch / stop
```

The system should not become an elegant RAM heater.

Recent formal work improves the trust boundary:

**Checking Equality-Saturation Merge and Extraction Certificates (Isabelle AFP, July 2026)** provides an independently verified checker for egg equality explanations and extraction certificates.

https://isa-afp.org/entries/Equality_Saturation_Checker.html

This strongly supports the system-wide law:

> **The search engine may be large and aggressive. Accepted mathematical results should be supported by independently checkable evidence whenever possible.**

---

# 6. Search is not only value search — representation search may be central

One of the most important findings is that difficult problems can become easier when represented differently.

The project should therefore search over **representations**, not only candidate answers.

Possible obligations include:

```text
find a basis that exposes sparsity
find coordinates that linearize behavior
quotient by symmetry
find invariant observables
reduce dimensionality
find a canonical form
find a decomposition
map the problem to a known problem class
find a representation where inversion is simpler
```

Research donors:

## AI Feynman

AI Feynman combines structural tests such as:

- symmetry;
- dimensional analysis;
- additive/multiplicative separability;
- compositional structure;
- inversion;

and recursively reduces the problem before equation search.

https://pmc.ncbi.nlm.nih.gov/articles/PMC7159912/

The neural component is not the important donor here. The important donor is **structural simplification before brute-force formula search**.

## Koopman methods

Koopman theory seeks observables in which nonlinear dynamics can be represented by a linear operator.

This is a direct example of:

```text
hard representation
    -> better coordinates
    -> much simpler mathematics
```

https://pmc.ncbi.nlm.nih.gov/articles/PMC4769143/

## Symmetry-informed equation discovery

Recent work shows known or discovered symmetries/invariants can drastically reduce the search space for governing equations.

The project should treat **discover symmetry / discover invariant / discover coordinate system** as first-class mathematical work obligations.

---

# 7. Mathematical metaprimitives

The primitive set should include not only ordinary mathematical operations but also **operations on mathematical structures and theories themselves**.

Ordinary primitives might include:

```text
add
multiply
factor
compose
solve
reduce
```

Metaprimitives may include:

```text
generalize(C1...Cn)
invert(C)
factor-construction(C)
change-representation(C, R)
discover-invariant(C)
derive-rewrite(C1, C2)
abstract-common-structure(C1...Cn)
search-symmetry(C)
search-decomposition(C)
compress-theory(T)
```

This distinction matters because a system capable of manipulating its own mathematical structures can improve its search vocabulary without model dependence.

Relevant donor:

**Symbolic metaprogram search / AntiUnify metaprimitives (Nature Communications, 2024)**  
https://www.nature.com/articles/s41467-024-50966-x

---

# 8. Automatic primitive formation and self-expansion

The project should not have a permanently fixed mathematical instruction set.

Let the trusted primitive set at generation `n` be:

```text
B_n
```

If a new mathematical structure `C` is discovered and independently certified, then it may become:

```text
B_(n+1) = B_n U {C}
```

Future problem-solving campaigns can then use `C` as a primitive rather than rediscovering it.

This is **self-expansion through proven mathematics**, not self-modifying AI behavior.

Relevant donors:

## DreamCoder / Stitch

DreamCoder introduced library learning where repeated solved programs become reusable abstractions.

Stitch makes symbolic abstraction learning dramatically faster and more memory-efficient.

https://arxiv.org/abs/2211.16605

The donor idea is:

```text
solved programs
  -> identify repeated structure
  -> synthesize abstraction
  -> rewrite old solutions
  -> use abstraction in future search
```

The project must add mathematical certification before promotion.

## Babble

Babble combines equality saturation and anti-unification so semantically equivalent but syntactically different programs can contribute to common abstraction discovery.

This suggests a stronger loop:

```text
certified constructions
  -> semantic equivalence expansion
  -> anti-unification / generalization
  -> candidate general law
  -> counterexample search
  -> proof / certificate
  -> new primitive
```

## QuickSpec

QuickSpec automatically discovers equational properties and uses newly discovered equalities to reduce future term generation.

Important law:

> **New mathematical knowledge should change the geometry of future search, not merely be stored.**

## HR (Automated Theory Formation)

Simon Colton's HR system is a historical but highly relevant donor for:

- concept invention;
- example generation;
- conjecture formation;
- theorem-prover integration;
- counterexample/model generation;
- using discoveries to guide further concept formation.

This supports a distinction between two growth modes:

### Compression growth

Repeated successful structures become one stronger primitive.

### Theory growth

Existing primitives are combined/manipulated to introduce new concepts, conjectures, relations, and transformations.

The project likely needs both.

---

# 9. Failure must become capability

A failed path should not merely become history.

CEGIS, CEGAR, CDCL, nogood learning, and abstraction-refinement systems show that a counterexample can eliminate entire families of future candidates.

Desired behavior:

```text
candidate fails
    -> recover counterexample / proof of failure
    -> generalize failure condition where justified
    -> create pruning knowledge
    -> future search excludes whole invalid region
```

A future artifact might look conceptually like:

```text
Failure theorem F72:
Any construction containing pattern P
under assumptions A
cannot satisfy target T.
```

This makes failure an active mathematical primitive for reducing future search.

Relevant donor families:

- CEGIS
- CEGAR
- CDCL/nogood learning
- proof-based abstraction refinement

---

# 10. Mathematical search should optimize information, not only runtime

A generic scheduler optimizes CPU, RAM, throughput, or latency.

A mathematical scheduler should eventually consider:

- expected information gain;
- expected search-space reduction;
- falsification probability;
- dependency unlock potential;
- novelty;
- proof value;
- representation simplification;
- verification cost;
- compute cost.

A slower operation can be superior if it collapses most of the remaining search space.

This may eventually lead to an objective roughly shaped like:

```text
expected useful mathematical information / compute cost
```

The exact metric is unresolved and requires research.

Ptah may later supply machine scheduling and durable workspace execution. This project should own the **mathematical value model** that decides which mathematical work is worth scheduling.

---

# 11. Inversion should be first-class

For every transformation/relation, the system should track whether inversion is:

- exact;
- partial;
- left-invertible;
- right-invertible;
- set-valued;
- approximate;
- information-losing;
- unknown.

If the inverse does not exist directly, the system may search for:

- a partial inverse;
- a representation where inversion becomes easier;
- a decomposition into invertible components;
- a relation that solves the inverse query;
- a synthesized inverse under restricted assumptions.

Relational semantics may make forward and inverse meaning share one semantic definition while compiled execution differs by query direction.

---

# 12. Proof/search should be co-designed

The project should not always:

```text
find construction first
then prove it
```

Sometimes a slightly slower construction is preferable if it admits a tiny, independently checkable certificate.

Therefore search may need to optimize pairs:

```text
(construction, proof/certificate route)
```

Verification cost should participate in mathematical optimization.

---

# 13. Solver power versus mathematical authority

A system-wide constitutional principle has emerged from SAT/SMT, optimization, theorem proving, and the existing THETECHGUY ecosystem:

> **Solving can be expensive and heuristic. Acceptance should rely on the strongest independently checkable witness available.**

Examples:

- cvc5 can emit Alethe proofs checked by Carcara;
- SAT solvers emit proof certificates checked independently;
- VIPR checks mixed-integer-programming results using exact rational certificates;
- Lean can verify certificates produced from external CAS calculations;
- equality-saturation certificates can now be independently verified in Isabelle AFP.

The project should prefer architectures in which large search engines are replaceable and small checkers form the trusted boundary.

---

# 14. Truth must be multidimensional

A single field such as:

```text
verified = true
```

is unacceptable.

At minimum, mathematical state should distinguish independent dimensions such as:

```text
truth:
    PROVEN
    REFUTED
    OPEN
    EMPIRICAL
    UNKNOWN

scope:
    UNIVERSAL
    ASSUMPTION_BOUND
    FINITE_EXHAUSTIVE
    BOUNDED
    SAMPLED

verification:
    FORMAL_KERNEL
    INDEPENDENT_CERTIFICATE
    EXACT_RECOMPUTATION
    EXHAUSTIVE
    RIGOROUS_ENCLOSURE
    PROBABILISTIC
    EMPIRICAL

freshness:
    CURRENT
    STALE
```

A billion observed examples must not become a universal theorem.

A theorem proven only under assumptions must not silently become assumption-free.

A completed computation must not be confused with a proved claim.

---

# 15. Provenance and stale-proof invalidation

The project must distinguish immutable mathematics from versioned human/machine knowledge about mathematics.

Tenfold Gen2 is an important THETECHGUY donor here.

Tenfold G2-06 established:

- canonical artifact semantics;
- independent Python/Rust/verifier representations;
- cross-language disagreement detection;
- permanent adversarial fixtures for discovered divergences;
- reject-unknown / reject-lossy / reject-ambiguous policy.

Tenfold G2-12 established:

- proof graphs;
- no vacuous `PROVEN`;
- evidence-bearing proof transitions;
- mandatory assurance derivation;
- independent assurance reconciliation;
- exact input-digest binding;
- stale proof rejection when a closed input changes.

Donor repository:

https://github.com/jaydumisuni/tenfold

For this project, the analogous rule should be:

```text
claim
+ assumptions
+ definitions
+ dependencies
+ exact semantic identities
+ certificate/proof
+ verifier identity
= certification state
```

If a material dependency, assumption, or definition changes, downstream certifications become stale automatically.

---

# 16. Existing THETECHGUY donor map

The project should reuse mechanisms from the ecosystem without collapsing ownership boundaries.

## Tenfold

Use as donor for:

- canonical IR discipline;
- independent verifier architecture;
- proof graphs;
- hermetic input binding;
- mutation/adversarial qualification;
- cross-implementation semantic agreement;
- fail-closed policy.

Tenfold must not become the mathematics runtime.

## Sergeant

Sergeant currently defines a proven scaling rule for investigation work:

```text
private_force_size = max(20, human_equivalent_workers * 10)
```

and preserves authority boundaries in which workers gather evidence but cannot issue final verdicts or expand scope.

Donor repository:

https://github.com/jaydumisuni/Sergeant

For this project, the organizational theorem can become **Mathematical Work Cells**.

A cell may contain:

- one deterministic algorithm;
- one solver;
- one prover;
- one CPU core;
- many CPU threads;
- a GPU kernel;
- one model proposing candidates;
- nested bounded campaigns.

The cell contract matters more than the implementation inside the cell.

## Pete

Pete's Deterministic Distillation doctrine is highly relevant:

```text
repeated successful operations
  -> candidate deterministic workflow
  -> provenance
  -> negative controls
  -> unrelated transfer tests
  -> independent review
  -> deterministic capability
```

Donor repository:

https://github.com/jaydumisuni/pete

This project needs the mathematical analogue, with stronger proof/certification requirements.

## Origins Factory

Origins already defines ecosystem capability discovery/compilation and separates semantic, mechanical, and assurance truth.

Donor repository:

https://github.com/jaydumisuni/origins-factory

The mathematical project should expose capabilities to Origins rather than invent a second ecosystem-wide capability framework.

## Ptah Space

Ptah should eventually supply:

- durable Workspaces;
- Activities;
- Nodes;
- Facilities;
- artifact persistence;
- resource/compute scheduling;
- checkpoints;
- distributed execution.

Donor/project repository:

https://github.com/jaydumisuni/Ptah-space

The mathematics project must **not** invent another generic workspace system. Ptah changes come later after mathematical workload requirements are understood.

## Wolf-Coin

Wolf-Coin remains a trading-domain authority and future consumer.

The mathematical project may provide transformations, optimization, statistics, structure discovery, or certified mathematical results. It must not issue trading actions or become Wolf's risk/execution authority.

Repository:

https://github.com/jaydumisuni/Wolf-Coin

## Odysseus / Hunter / other intelligence systems

These may become clients and research interfaces.

They can:

- formulate problems;
- suggest candidate transformations;
- propose search grammars;
- interpret results;
- explain mathematical output to humans.

They do not become mathematical truth authority.

---

# 17. External mathematical/computational donors discovered so far

## SPIRAL

SPIRAL is one of the closest partial analogues found.

It represents mathematical transforms as formulas, searches alternative formula decompositions, optimizes at formula level, and emits architecture-tuned code.

https://spiral-software.github.io/spiral-software/introduction.html

Important proof:

> Mathematical formulas themselves can serve as a searched/optimized intermediate language before ordinary code generation.

## HELIX

HELIX combines SPIRAL-like optimization with formal/certified translation layers and can treat SPIRAL as an optimization oracle.

https://www.spiral.net/software/helix.html

Important proof:

> A powerful optimizer can be treated as untrusted while a separate certification path establishes admissibility.

## FLINT / Arb

FLINT provides high-performance exact arithmetic and algebra.

Arb provides rigorous ball arithmetic where results are guaranteed enclosures rather than heuristic decimal approximations.

https://flintlib.org/doc/

Important lesson:

> Rigorous numerical semantics can coexist with high performance.

## PARI/GP

Strong number-theory donor with explicit distinctions between probable-prime and proven-prime operations.

Important lesson:

> Truth status must preserve the difference between probable and proven mathematical results.

## Lean / Rocq / formal proof systems

Potential proof/certificate authority routes.

Current strategy is not to make one theorem prover the project identity. External provers should sit behind a proof/certificate contract.

## cvc5 / Carcara / Alethe

Useful proof-producing SMT pattern:

```text
complex solver -> proof certificate -> independent checker
```

## SAT proof systems / CakeML checkers

Demonstrate high-performance search with independently verified proof checkers.

## VIPR

Demonstrates independently checking optimization results with exact rational certificates.

## OpenMath

OpenMath provides semantic mathematical objects and Content Dictionaries.

https://openmath.org/standard/

Important limitation:

OpenMath intentionally allows multiple equivalent encodings. Therefore it may be useful for semantic interoperability but is not sufficient by itself for Tenfold-style byte-canonical mathematical identity.

A stricter project-owned canonical profile may still be required.

---

# 18. Program synthesis and executable mathematics

## SyGuS

Syntax-Guided Synthesis formalizes:

> Given a specification and grammar, find an executable expression satisfying the specification.

https://sygus-org.github.io/

This directly supports the idea of executable mathematics discovered by deterministic search.

## CEGIS

Counterexample-Guided Inductive Synthesis gives the important failure-learning loop:

```text
candidate
 -> verifier
 -> counterexample
 -> constrain all future candidates
```

## Ruler

Ruler automatically discovers rewrite rules using equality saturation.

https://arxiv.org/abs/2108.10436

Important proof:

> The system does not necessarily need humans to provide every transformation rule.

## Synthesizing Mathematical Identities with E-Graphs

Demonstrates automated composition/search producing thousands of candidate identities and reducing them to a smaller core identity set.

https://arxiv.org/abs/2206.07086

## Physics-informed program synthesis (2026)

A March 2026 JACS paper used physics-informed program synthesis to discover new electronic-structure algorithms, including alternatives that avoid normal self-consistent-field iteration.

Important proof:

> Program search can discover a genuinely new executable algorithm by exploiting mathematical structure in a narrow scientific domain.

This is direct evidence for the larger vision, though not proof that it generalizes automatically.

---

# 19. Mathematical compression and generalization

The system should actively seek **compressed/generalized mathematical structure**, not only solve individual cases.

If many constructions:

```text
C1, C2, ... Cn
```

are generated by one general construction:

```text
G(theta)
```

then `G` may represent a more powerful piece of mathematics.

This suggests connections to:

- Minimum Description Length;
- anti-unification;
- library learning;
- symbolic regression;
- theory formation;
- grammar induction;
- algorithmic-information approaches.

A promoted generalization must still survive counterexample search and certification.

---

# 20. CPU-first execution and specialization

The project should be useful on ordinary hardware before exceptional compute is required.

Execution policy should be based on mathematical workload structure, not a blanket GPU preference.

Examples:

```text
branch-heavy symbolic work         -> CPU
exact large-integer arithmetic     -> optimized CPU/native libraries
SAT/SMT                            -> specialized CPU solvers
dense linear algebra               -> SIMD / multicore / GPU
huge regular enumeration           -> GPU where justified
proof/certificate checking         -> CPU
```

The ideal asymmetry is:

```text
cost of discovering mathematics
    >>
cost of executing discovered mathematics
```

A very expensive campaign may ultimately distill into a microsecond-scale native primitive.

Relevant donors:

- SPIRAL
- HELIX
- FLINT/GMP/MPFR
- MLIR/LLVM
- Julia Symbolics / Nemo
- Futamura projections / partial evaluation
- multi-stage relational programming
- Souffle

The likely long-term compilation shape is:

```text
mathematical semantics
    -> query specialization
    -> mathematical optimization
    -> domain-specific lowering
    -> native IR
    -> CPU machine code
    -> optional SIMD/GPU/distributed realization
```

The language/runtime implementation is **not frozen**.

Rust remains an attractive control/kernel/compiler candidate but should not be treated as decided until FFI, licensing, performance, and prototype evidence are complete.

---

# 21. Out-of-core mathematics

RAM should not automatically define the maximum mathematical object size.

FORM demonstrates symbolic computation strategies capable of handling expressions larger than available RAM through streaming/disk-backed processing.

https://github.com/form-dev/form

Future research must determine how out-of-core mathematical structures can preserve:

- deterministic identity;
- incremental checkpoints;
- proof/certificate lineage;
- replay;
- distributed partitioning.

---

# 22. Workspace and large-scale campaigns

Human mathematicians use notebooks and whiteboards.

This project should eventually use persistent mathematical workspaces, but the generic Workspace substrate belongs to Ptah later.

A large campaign may logically contain:

```text
original problem
formalizations
representations
assumption worlds
known mathematics
constructions
transformations
experiments
conjectures
counterexamples
proofs
failed paths
equivalence classes
discovered invariants
synthesized programs
certified results
```

The mathematical project should define the semantics of those artifacts.

Ptah should eventually own persistence, Activities, scheduling, and machine placement.

---

# 23. Mathematical Work Cells and scaling

The Sergeant 20-for-2 doctrine is an orchestration donor, not a literal permanent mathematical scale law.

A mathematical campaign may create many heterogeneous cells:

```text
relation search
invariant search
symmetry search
representation search
inverse search
lattice search
SAT search
SMT search
polynomial search
equality saturation
counterexample generation
proof search
proof minimization
program synthesis
numerical certification
complexity estimation
```

A single cell may itself perform millions or billions of deterministic operations.

Therefore:

> **Number of cells is not the same thing as number of mathematical attempts.**

The outer campaign controls obligations, evidence, budgets, and authority boundaries. The inner mathematical engines define the actual computational scale.

---

# 24. Blind rediscovery as First-Light methodology

The first proof of capability should not be “solve a famous unsolved problem.”

The first serious benchmark should hide known mathematical results and require the system to rediscover them without direct access to the answer.

Example methodology:

```text
known theorem/formula/construction
   -> hide target result
   -> expose permitted primitives and evidence
   -> run discovery campaign
   -> freeze candidate
   -> compare against hidden truth
```

Include:

- true targets;
- convincing false near-misses;
- formulas true only under missing assumptions;
- finite numerical coincidences;
- deliberate counterexamples.

The system must:

- rediscover known truths;
- reject false candidates;
- preserve assumptions;
- independently certify survivors;
- compile at least one survivor into a reusable primitive;
- demonstrate that the new primitive makes a later problem easier.

That final step proves **self-expansion**, not merely automated solving.

---

# 25. Current strongest open questions

These questions are architecture-changing and should drive the next research pass.

## A. Can the four substrate planes coexist efficiently?

Need deeper research into combinations of:

- e-hypergraphs;
- semantic e-graphs;
- versioned/contextual e-graphs;
- disequality;
- egglog/Datalog;
- provenance;
- proof-producing equality saturation.

Question:

> Has any system already combined several of these successfully, and where are the scaling/fundamental conflicts?

## B. Can relational semantics support the whole problem-solving family?

Need evidence for:

- forward calculation;
- inverse calculation;
- partial information queries;
- program synthesis;
- proof search;
- finite enumeration;
- optimization;

from one semantic relation while still allowing direction-specific compilation.

Question:

> Where does relational meaning stop being practical, and what must become separate specialized semantics?

## C. Automatic representation discovery

Need deeper research into:

- canonical forms;
- quotient spaces;
- symmetry discovery;
- invariant theory;
- basis discovery;
- coordinate discovery;
- dimensional reduction;
- Koopman-style lifting;
- graph embeddings;
- algebraic decomposition;
- topology-preserving transformations.

Question:

> Can representation change become a deterministic first-class search operation instead of depending on human insight or models?

## D. Mathematical generalization

Need deeper research into:

- anti-unification;
- inductive generalization;
- abstraction learning;
- theory formation;
- rewrite-rule synthesis;
- general theorem formation;
- semantic compression.

Question:

> How can several certified constructions become a candidate general law without overgeneralizing finite coincidence?

## E. Failure compilation

Need deeper research into:

- CDCL;
- nogood learning;
- CEGIS;
- CEGAR;
- proof-based abstraction refinement;
- counterexample generalization.

Question:

> How much can one failure eliminate from future mathematical search while preserving soundness?

## F. Mathematical information-value scheduling

Question:

> What metrics can estimate expected mathematical information gain, search-space reduction, novelty, proof value, and dependency unlock potential before spending compute?

## G. Proof/certificate map

Need a domain-by-domain map for:

- integer arithmetic;
- primality;
- factoring;
- polynomial identities;
- Grobner bases;
- SAT;
- SMT;
- graph/combinatorial results;
- linear programming;
- MILP;
- rigorous numerics;
- synthesized executable constructions.

Question:

> Which result families already admit small independently checkable witnesses, and which require new certificate designs?

## H. Canonical mathematical identity

Need mapping against:

- OpenMath;
- SMT-LIB;
- TPTP;
- Lean terms;
- MathJSON and related systems;
- e-graph/e-hypergraph representations.

Question:

> What strict canonical representation is required for deterministic hashing, proof freshness, and cross-language agreement without inventing unnecessary proprietary semantics?

---

# 26. Current research priority order

The next research should proceed in this order unless new evidence changes the ranking:

1. **Mathematical generalization + automatic representation change**
2. **Four-plane substrate integration (e-hypergraph / semantic equality / relational deduction / provenance)**
3. **Relational semantics plus direction-specific specialization**
4. **Failure compilation and search-space learning**
5. **Proof-producing equality saturation / certificate routing**
6. **Mathematical metaprimitives**
7. **Information-value scheduling**
8. **Native specialization / compilation architecture**
9. **Out-of-core/distributed mathematical state**

Broad “find another CAS” research is now lower priority unless it resolves one of these specific design uncertainties.

---

# 27. Explicit non-decisions

The following are **not decided**:

- final repository/product name;
- whether the deepest substrate is definitely an e-hypergraph;
- whether category-theoretic semantics become constitutional;
- whether the primary implementation language is Rust;
- whether MLIR is the final lowering infrastructure;
- whether Lean/Rocq/another prover becomes the primary formal proof route;
- whether one relational semantic language can cover every problem family;
- exact problem/object IR;
- exact truth-status schema;
- exact primitive-promotion rules;
- exact Ptah integration contract;
- GPU architecture;
- distributed architecture;
- public API;
- UI;
- roadmap phases.

These require additional research or a later owner-approved design freeze.

---

# 28. Anti-drift rules for future research

1. **Do not rename the project conceptually based on the temporary repository name.**
2. **Do not turn models into mathematical authority.**
3. **Do not duplicate Ptah workspace/execution ownership.**
4. **Do not duplicate Origins ecosystem capability ownership.**
5. **Do not turn Wolf-Coin into the mathematical project.**
6. **Do not constrain internal mathematics to human-readable equations.**
7. **Do not equate computation completion with proof.**
8. **Do not equate cross-solver agreement with mathematical proof.**
9. **Do not promote a discovered primitive without falsification and the strongest practical certification route.**
10. **Do not assume GPU use is inherently superior to CPU/native mathematical specialization.**
11. **Do not saturate/search globally where bounded structural search can be derived.**
12. **Do not freeze implementation details before the mathematical substrate survives adversarial examples across unrelated domains.**

---

# 29. Working summary

The current strongest picture is:

```text
KNOWN MATHEMATICAL UNIVERSE
        |
        v
semantic objects / relations / transformations
        |
        +-- alternate representations
        +-- equivalence / disequality
        +-- assumption worlds
        +-- provenance / proof obligations
        |
        v
TARGET CONDITION
        |
        v
MATHEMATICAL SEARCH CAMPAIGN
        |
        +-- representation search
        +-- inverse search
        +-- decomposition
        +-- synthesis
        +-- equality saturation
        +-- generalization
        +-- counterexample search
        +-- proof search
        |
        v
CANDIDATE MATHEMATICAL STRUCTURE
        |
        +-- refuted -> failure/pruning knowledge
        |
        +-- uncertified -> retained candidate/open knowledge
        |
        +-- certified
                |
                v
        SPECIALIZE / COMPILE
                |
                v
        FAST EXECUTABLE REALIZATION
                |
                v
        PROMOTED MATHEMATICAL PRIMITIVE
                |
                v
EXPANDED MATHEMATICAL UNIVERSE
```

The key recursive property is:

```text
M_(n+1) is computationally stronger than M_n
```

only when newly admitted mathematics has survived the required certification boundary.

The project is therefore not merely a repository of known formulas and not merely a solver.

The research target is a machine in which **mathematical knowledge changes the future search space and becomes executable capability**.

---

# 30. Next action

Continue targeted research from this checkpoint, beginning with:

1. automatic mathematical generalization;
2. automatic representation discovery/change;
3. combinations of semantic e-hypergraphs, versioned equality, relational deduction, and provenance;
4. sound failure generalization/pruning.

Update this file or create a superseding checkpoint when new research materially changes the architecture hypotheses.