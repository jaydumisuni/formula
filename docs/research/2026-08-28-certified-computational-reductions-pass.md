# Research Pass — Certified Computational Reductions and Whole-Capability Transfer

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates computational reductions as first-class mathematical artifacts: instead of solving every new problem class directly, certify a map into a problem class already supported by strong solvers/certificates, then lift the result back.

The central finding is:

> **A certified reduction can transfer an entire body of solver capability from one problem class to another. But “reduction” is not one semantic claim: decision, witness, counting, optimization, approximation, and parameterized reductions preserve different information. The reduction artifact must declare exactly what survives and how results/certificates are reconstructed.**

---

## 1. A reduction can make one solver serve an entirely different problem class

A many-one/Karp-style decision reduction maps instances of problem `A` to instances of `B` so that the yes/no answer is preserved.

Conceptually:

```text
instance x of A
    -> encode f(x) as B
    -> solve B
    -> recover answer to A
```

### Architectural implication

The project should register reductions alongside ordinary primitives:

```text
CertifiedReduction
    source_problem_class
    target_problem_class
    encode
    preserved_result_semantics
    result/witness_lift
    assumptions
    complexity_overhead
    proof/certificate route
```

Once admitted, every matching future source problem can route through the target capability automatically.

---

## 2. Decision preservation is weaker than witness preservation

A decision reduction may establish only:

```text
A(x) is satisfiable <-> B(f(x)) is satisfiable
```

but a useful solver integration often also needs a way to convert a target witness back into a source witness.

### Architectural implication

The reduction contract should distinguish:

```text
decision_preserving
witness_forward_map
witness_backward_map
bijection_of_solution_sets
partial witness reconstruction
```

A client asking only feasibility may accept a weaker reduction than a client asking for all actual solutions.

---

## 3. Parsimonious reductions preserve exact solution counts

A parsimonious reduction preserves the number of solutions, not merely whether a solution exists.

Sources:

https://reductions.network/parsimonious

https://www.sciencedirect.com/science/article/pii/S030439750400115X

### Architectural implication

The same encoding may or may not be valid for different query algebras.

For example:

```text
equisatisfiable reduction
    -> valid for existence
    -> NOT automatically valid for counting

parsimonious reduction
    -> can transfer exact counting
```

This connects directly to semiring-parametric evaluation: the reduction must declare which evaluation semantics it preserves.

---

## 4. Optimization reductions need richer preservation contracts

Approximation-preserving reductions map both instances and solutions while controlling how solution quality/approximation guarantees transfer. The literature distinguishes AP-reductions, L-reductions, strict reductions, PTAS reductions, and other variants.

Sources:

https://onlinelibrary.wiley.com/doi/10.1002/9781118600207.ch12

https://www.sciencedirect.com/science/article/pii/S0377221705005011

### Architectural implication

For optimization, the reduction artifact should expose a mathematical relation between objectives:

```text
source_objective
    <-> target_objective
```

plus, where applicable:

```text
optimum preservation
approximation-ratio transformation
error/gap transformation
solution reconstruction
```

The compiler can then decide whether a target solver satisfies the source client's requested guarantee.

---

## 5. Parameterized reductions preserve structural tractability information

Parameterized complexity studies reductions/kernels that preserve a chosen parameter so a generally hard problem may become fixed-parameter tractable after structural reduction.

Kernelization reduces an instance to an equivalent smaller instance whose size depends primarily on the parameter rather than raw input size.

Sources:

https://www.sciopen.com/article/10.1109/TST.2014.6867516

https://www.sciencedirect.com/science/article/pii/S1574013726001437

### Architectural implication

A reduction can carry parameter semantics:

```text
source parameter k
    -> target parameter g(k)
```

and kernelization can be represented as a particularly valuable self-reduction:

```text
P(x,k)
    -> smaller equivalent P(x',k')
```

with a certified size bound.

This ties reduction search to the earlier “find where the difficulty lives” research.

---

## 6. Verified bit-blasting is a concrete proof-preserving reduction pipeline

Lean's current bitvector decision tactic performs an explicit verified reduction chain:

```text
BitVec/Bool theorem
    -> reflected expression
    -> verified bit-blasting to AIG
    -> verified AIG-to-CNF translation
    -> external SAT solver
    -> LRAT UNSAT certificate
    -> verified LRAT checker
    -> theorem about original BitVec goal
```

Sources:

https://github.com/leanprover/leansat/blob/main/README.md

https://lean-lang.org/doc/api/Lean/Elab/Tactic/BVDecide.html

The former LeanSAT repository was merged into Lean core as `Std.Tactic.BVDecide`.

### Architectural implication

This is nearly the ideal model for `CertifiedReduction`:

> The target solver is untrusted, but the reduction and certificate-lifting chain are proved/checked, so a target-domain proof establishes the source-domain result.

A source problem can therefore borrow a highly optimized target solver without surrendering source semantics.

---

## 7. Translation/encoding correctness can itself be certified

Certified CNF-translation work proves correctness of encodings from higher-level pseudo-Boolean/cardinality constraints to CNF rather than trusting encoding software.

Source:

https://drops.dagstuhl.de/storage/00lipics/lipics-vol236-sat2022/LIPIcs.SAT.2022.16/LIPIcs.SAT.2022.16.pdf

### Architectural implication

For reduction-heavy execution, the encoder is part of the mathematical proof boundary.

A reduction should be admitted through either:

```text
proved encoder theorem
```

or:

```text
per-instance translation certificate + checker
```

rather than trusting arbitrary preprocessing code.

---

## 8. Reduction composition creates a capability-routing graph

Suppose certified reductions exist:

```text
A -> B
B -> C
```

Then the project can potentially derive:

```text
A -> C
```

by composition, including composed witness/certificate lifts and accumulated complexity overhead.

### Architectural implication

The capability registry becomes a graph whose edges are reductions/transforms and whose nodes are problem classes/theories.

Problem solving can search this graph for a route to a class with mature primitives:

```text
source class
    -> reduction chain
    -> solver-rich target class
```

This is computationally analogous to the previously researched theory-morphism graph.

---

## 9. The shortest reduction chain is not necessarily the best one

A direct encoding may produce a huge target instance, weak propagation, expensive reconstruction, or poor certificates.

Another longer chain may expose structure and solve much faster.

### Architectural implication

Reduction routing needs a cost vector:

```text
encoding cost
target size/width
solver expected cost
witness reconstruction cost
certificate size/check cost
approximation/error degradation
parameter blow-up
future reuse value
```

The Search Economy chooses a route without changing mathematical validity.

---

## 10. Reduction discovery is a new mathematical synthesis target

Instead of manually registering every reduction, the project can eventually search for:

```text
encode : A -> B
lift : Result_B -> Result_A
```

such that a declared preservation theorem holds.

This can involve:

- representation changes;
- gadget constructions;
- quotient/embedding maps;
- conjugacies;
- factorization;
- theory interpretations;
- program synthesis.

### Architectural implication

`discover_reduction(A,B)` may become one of the highest-value metaprimitives because one successful general reduction can unlock *all* target-class mathematics for every future source instance.

The reduction must go through adversarial falsification and proof/certification before promotion.

---

## 11. Reductions can transfer certificates as well as answers

The strongest reduction route maps a target witness/proof into a compact source certificate or chains a source theorem to a checked target certificate.

Lean's bitvector pipeline demonstrates exactly this for UNSAT proofs.

### Architectural implication

The universal certificate envelope should support nested/composed evidence:

```text
source claim
    -> reduction certificate/theorem
    -> target instance digest
    -> target certificate
    -> target checker
    -> lift theorem
```

Downstream systems can independently replay the whole path without trusting the source solver orchestration.

---

## 12. Query semantics must choose reduction strength

A source problem may be queried for:

```text
exists solution?
find one solution
find all solutions
count solutions
optimize objective
approximate optimum
prove uniqueness
sample solutions
```

### Architectural implication

The compiler should only route through reductions whose preservation contract is strong enough for the requested result.

For example:

```text
existence query
    -> equisatisfiable encoding may suffice

count query
    -> require count-preserving/reconstructable reduction

unique-solution query
    -> require uniqueness preservation
```

This prevents accidental semantic weakening during aggressive solver reuse.

---

## 13. Reduction edges can become permanent capability multipliers

If class `B` later gains a new primitive/decision procedure, every source class with a compatible certified reduction into `B` may gain access automatically.

### Architectural implication

Capability growth is no longer only:

```text
new primitive -> one domain becomes stronger
```

It can be:

```text
new primitive in B
    -> traverse compatible incoming reduction edges
    -> many domains become stronger immediately
```

Likewise, one newly discovered reduction can unlock an already huge target capability library.

This is multiplicative self-expansion.

---

## 14. Current certified-reduction hypothesis

```text
NEW PROBLEM CLASS / INSTANCE A
    -> classify requested result semantics
    -> search capability/reduction graph
    -> choose compatible target B with strong solvers
    -> verified/certified encoding A -> B
    -> solve B using replaceable solver
    -> verify B result/certificate
    -> certified witness/result lift B -> A
    -> compose source certificate
    -> return A result

IF reduction is new/generalizable
    -> prove preservation contract
    -> promote edge
    -> future A problems inherit B capability
```

This is one of the strongest mechanisms yet found for making the entire existing body of mathematical algorithms composable.

---

## 15. New research obligations

1. Define a preservation lattice/schema: decision, witness, bijection, count, uniqueness, optimum, approximation, sampling/distribution, parameter, proof.
2. Study witness-preserving and proof-preserving reductions beyond decision complexity theory.
3. Investigate automatic reduction/gadget synthesis with independent equivalence checking.
4. Study reduction composition and canonical simplification of long reduction chains.
5. Investigate cost-aware routing over the capability/reduction graph.
6. Study reductions between continuous, algebraic, graph, optimization, SAT/SMT, and automata domains.
7. Investigate certificate translation/lifting for SAT/SMT/MILP/graph/algebra reductions.
8. Connect theory morphisms and computational reductions: identify when one artifact can serve both roles.
9. Study negative knowledge: certify that no reduction exists in a declared restricted transformation class where decidable.
10. Build First-Light rediscovery tasks where the system must discover or select a reduction into a known solver class rather than solve the source problem directly.
