# Inductive Proof and Lemma Discovery Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Why this pass exists

Many mathematical claims concern:

- recursively defined functions;
- natural numbers;
- trees/lists/terms;
- inductively generated structures;
- recursive algorithms;
- all finite objects of a structural class.

Brute-force checking finitely many examples cannot prove such universal claims.

A capable problem-solving system must discover:

- the right induction principle;
- the right induction variable/measure;
- stronger induction hypotheses;
- auxiliary lemmas;
- useful generalizations.

These are not merely proof-format details. They are **mathematical discoveries required to make the proof possible**.

## 1. ACL2: recursion suggests induction

ACL2 associates admitted recursive definitions with well-founded termination measures. Applications of recursive functions then suggest corresponding induction schemes that unwind the same recursive structure.

Sources:
- https://acl2.org/doc/index-seo.php?path=5096%2F7679%2F56%2F350&xkey=ACL2____INDUCTION
- https://www.cs.utexas.edu/~moore/acl2/v2-8/INDUCTION.html

Example principle:

```
recursive definition:
    f(x) -> f(smaller(x))

termination proof:
    measure(smaller(x)) < measure(x)

induction candidate:
    assume P(smaller(x))
    prove P(x)
```

This demonstrates a powerful self-duality:

> **the mathematical evidence that recursion terminates can generate the induction structure used to prove properties of the recursion.**

That relation should be first-class in the project.

## 2. Multiple induction schemes and strategy selection

One conjecture may contain several recursive functions and suggest many competing induction schemes. ACL2 combines, vetoes, or selects schemes through induction heuristics and also supports explicit custom induction rules.

Sources:
- https://acl2.org/doc/index-seo.php?xkey=ACL2____INDUCTION
- https://www.cs.utexas.edu/~moore/publications/acl2-induction-heuristics.pdf

A 2026 ACL2 induction-heuristics paper reports statistics over roughly 467k named theorems in Community Books. Induction was invoked more than 72k times, and explicit user induction hints were needed in a substantial minority, illustrating that automatic induction choice remains a real search problem rather than a solved syntactic step.

Source:
- https://www.cs.utexas.edu/~moore/publications/acl2-induction-heuristics.pdf

So the project should treat induction-scheme choice as a Search Economy decision with evidence, not a hard-coded `induct on first integer variable` rule.

## 3. Generalization is often necessary

ACL2's prover architecture explicitly contains generalization and cross-fertilization heuristics. A conjecture may need to be replaced by a stronger/more general statement so that the induction hypothesis becomes strong enough to prove the desired special case.

Source:
- https://acl2.org/doc/index-seo.php?path=4986%2F7401%2F831%2F974%2F63%2F1531&xkey=ACL2____ARCHITECTURE-OF-THE-PROVER

This gives a key proof-search pattern:

```
target theorem T
      ↓
direct induction fails
      ↓
find stronger theorem G
      ↓
prove G by induction
      ↓
T follows as specialization
```

So a failed proof can signal a **generalization discovery obligation**, not merely another tactic attempt.

## 4. Auxiliary lemma discovery

A 2024 IJCAR study integrated QuickSpec theory exploration with Vampire's automated induction.

QuickSpec proposes candidate equational lemmas from the theory; Vampire then uses useful candidates in the main proof and independently proves any lemma that is actually required.

Source:
- https://link.springer.com/chapter/10.1007/978-3-031-63498-7_13

Reported benchmark result versus Vampire's plain structural-induction baseline:

- lemma discovery alone: about 40% more proofs;
- induction-specialized strategy training alone: about 130% more proofs;
- combined: about 183% more proofs.

This is direct evidence for the architecture:

```
main proof stuck
      ↓
generate mathematical lemma candidates
      ↓
speculatively try subsets in main proof
      ↓
prove only lemmas that become necessary
      ↓
complete theorem
```

The system does not need to prove every conjectured lemma before knowing whether it is useful.

## 5. Candidate lemmas remain candidates

QuickSpec-generated equations are tested empirically over generated examples but are not automatically true.

The Vampire integration preserves the correct boundary:

```
conjectured lemma
      ↓
use speculatively as a route proposal
      ↓
if route depends on lemma:
    independently prove lemma
      ↓
only then final proof closes
```

This is exactly the project's constitutional model for model/generated mathematical candidates.

## 6. Goal-directed versus bottom-up theory exploration

Pure bottom-up exploration may discover enormous numbers of true but irrelevant facts.

The Vampire/QuickSpec work improves this by allowing candidate lemmas to compete according to whether they help the current main goal.

This suggests two modes:

### Background theory growth

```
explore structure
 -> conjecture general lemmas
 -> certify useful high-value laws
 -> promote to permanent theory
```

### Goal-directed lemma invention

```
current theorem blocked
 -> inspect proof gap
 -> conjecture only likely bridging lemmas/generalizations
 -> try speculatively
 -> certify those actually needed
```

Both should exist, but the Search Economy should strongly distinguish their value/cost.

## 7. Induction as one instance of well-founded reasoning

Natural-number induction is only one case.

The deeper structure is a well-founded relation/order:

```
≺
```

where every recursive/proof step moves to a smaller element.

Potential induction families include:

- structural induction;
- well-founded induction;
- lexicographic induction;
- multiset induction;
- ordinal induction in stronger settings;
- simultaneous/mutual induction;
- induction over derivations/proof trees.

The Theory Profile should record admissible well-founded structures and termination measures.

## 8. Induction and invariant synthesis connection

An induction hypothesis and an invariant are closely related proof objects:

```
inductive data:
    property preserved across constructor/recursive step

loop/state system:
    invariant preserved across transition step
```

Both require finding a stronger stable assertion that closes under an evolution rule.

This suggests a possible shared metaprimitive family:

```
SYNTHESIZE_INDUCTIVE_ASSERTION(
    transition/constructor relation,
    target property
)
```

with domain-specific realizations for:

- induction hypotheses;
- loop invariants;
- reachable-set invariants;
- barrier certificates;
- recursive summaries.

## 9. Induction and recursion duality

For an executable mathematical construction defined recursively, the project can potentially derive automatically:

```
termination measure
      ↓
well-founded recursion semantics
      ↓
corresponding induction principle
      ↓
proof templates for properties of construction
```

And conversely, a discovered induction/decomposition scheme may suggest an executable recursion/divide-and-conquer algorithm.

This is another bridge between proof and program.

## 10. Proof failure as structured information

If induction fails because the hypothesis is too weak, preserve the failure pattern:

```
induction scheme I
subgoal S
missing relation R
```

Then lemma/generalization synthesis can target `R` specifically.

A useful failed induction should generate:

- residual subgoal;
- variables not generalized enough;
- missing rewrite/lemma patterns;
- countermodels for candidate lemmas;
- dependencies of the blocked step.

This turns inductive proof failure into candidate-mathematics generation.

## 11. Promotion of discovered lemmas

A lemma invented to prove theorem `T` may later become broadly reusable.

Promotion path:

```
goal-local candidate lemma L
      ↓
formal proof
      ↓
record use/provenance
      ↓
transfer tests / relevance to other problems
      ↓
permanent theory lemma / rewrite rule / primitive law
```

If promoted as a rewrite rule, it may collapse future search spaces exactly as QuickSpec/IsaCoSy-style theory exploration does.

## 12. Mathematical Work Cell formation

An inductive proof campaign can split into heterogeneous cells:

```
- induction-scheme cells
- theorem-generalization cells
- lemma-conjecture cells
- counterexample cells
- rewriting/canonicalization cells
- direct ATP cells
- finite-model cells
- induction-hypothesis-strengthening cells
```

The root goal is an AND/OR proof graph:

```
T proven by scheme I
    AND
all generated subgoals
    AND
all auxiliary lemmas used
```

Candidate schemes/generalizations are OR alternatives.

This integrates directly with compositional proof progress.

## 13. Core law

> **When a universal recursive claim resists direct proof, search for the mathematical strengthening, auxiliary law, or well-founded structure that makes the induction close.**

A stronger theorem or new lemma can be the solution even when it was not part of the original problem statement.

## 14. Open research

1. Modern induction support in Vampire, cvc5, HipSpec, IsaPlanner, ACL2, Lean automation, and theorem-prover portfolios.
2. Automatic conjecture generalization/strengthening with correctness-preserving proof obligations.
3. Counterexample-guided induction-hypothesis synthesis.
4. Reusing recurrence/termination measures to derive induction schemas automatically.
5. Induction over semantic hypergraphs/relations rather than syntax trees.
6. Learned induction strategies that remain heuristic-only while final proof authority stays deterministic.
7. Integration of lemma discovery with equality saturation and theory exploration.
8. Minimal lemma sets/unsat-core-style proof dependency extraction after a successful inductive proof.
9. Incremental repair of inductive proofs when recursive definitions change.
10. Turning frequently discovered induction lemmas into compiled rewrite/solver primitives.
