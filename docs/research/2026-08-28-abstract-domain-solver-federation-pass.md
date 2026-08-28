# Abstract-Domain Solver Federation Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Why this pass exists

The runtime-theory-combination pass found a safe narrow federation pattern from SMT: specialist theory solvers can cooperate through carefully proved combination protocols, often exchanging equalities/disequalities over shared terms.

That interface is too narrow for the broader project.

A mathematical engine may know facts such as:

```
x ∈ [2, 3]
x ≡ 1 (mod 2)
x + y <= 7
x ∈ {1,4,9}
rank(A) >= 4
objective >= L
probability(event) <= p
```

and these partial facts can be highly valuable to another specialist before either side has a complete solution.

Abstract interpretation and cooperative constraint solving provide a general theory for this style of information exchange.

## 1. Solver cooperation through abstract domains

Talbot, Monfroy, and Truchet model constraint solvers and cooperation schemes using abstract interpretation. Constraint languages become ordered abstract domains, and cooperation is expressed through combinations/reductions of those domains.

Source:
- https://www.cambridge.org/core/journals/theory-and-practice-of-logic-programming/article/abs/modular-constraint-solver-cooperation-via-abstract-interpretation/0AD7487361E60FCCFAD458E7128E483C
- https://arxiv.org/abs/2008.01415

Their framework includes:

- interval-propagator completion, allowing domains to exchange bound constraints;
- delayed products, exchanging over-approximations between domains;
- shared products for modularly composing solvers/cooperation schemes.

This is fundamentally broader than equality-only coordination.

## 2. Reduced product as information exchange

Cousot, Cousot, and Mauborgne study reduced products of abstract domains and decision-procedure combination through **iterated pairwise reduction**.

Source:
- https://www.di.ens.fr/~cousot/COUSOTpapers/FoSSaCS-11.shtml

The key modularity result is architectural gold for this project:

> to add a new abstract component, define how observations are exchanged between the new component and each existing component; iterated reduction then propagates information through the combined product.

Conceptually:

```
Domain A: intervals
Domain B: congruences
Domain C: polyhedra

A learns: x ∈ [0,10]
B learns: x ≡ 7 mod 8

reduction:
    x ∈ {7}

then C receives x=7
and may derive further linear constraints
```

No solver needs to implement the others' algorithms.

## 3. Cooperative solver history

Earlier heterogeneous constraint-solving architectures explicitly observed that value-only propagation is too weak. For example, an integer finite-domain solver and real-linear solver can cooperate by exchanging variable bounds.

Source:
- https://research.google/pubs/combinatorial-problem-solving-in-constraint-logic-programming-with-cooperating-solvers/

Multi-domain constraint systems have combined:

- symbolic equality/disequality;
- real arithmetic;
- finite-domain integer constraints;

under sound/complete coordination models.

Source:
- https://www.cambridge.org/core/journals/theory-and-practice-of-logic-programming/article/on-the-cooperation-of-the-constraint-domains-and-in-cflp/A1D82C5C14F1B7BCB6E267728756C83E

This confirms that richer shared facts are an established need, not merely an optimization idea.

## 4. Shared bound propagation in parallel optimization

Distributed-domain-propagation research in parallel MIP shares global variable-bound tightenings between portfolio solver instances. Exchanged bounds trigger additional propagation in other solvers and improve performance.

Source:
- https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.SEA.2017.6

This gives a concrete scalable cooperation pattern:

```
solver A tightens x <= 17
      ↓
shared certified/global bound store
      ↓
solvers B,C,D propagate consequences
      ↓
new bounds/conflicts return to shared store
```

The system can benefit from solver diversity while sharing mathematically valid discoveries.

## 5. Architecture-changing conclusion: Shared Fact Algebra

The project likely needs something richer than a global clause/equality database.

Current hypothesis:

```
Shared Mathematical Fact Fabric
        |
        |- exact facts
        |- sound abstractions / over-approximations
        |- lower/upper bounds
        |- congruences
        |- finite-domain restrictions
        |- linear/polyhedral constraints
        |- equality/disequality
        |- theory-specific exported summaries
```

Each fact must carry:

```
semantic domain
precision/order relation
soundness direction
certificate/provenance
scope/world
freshness
producer
```

Specialist Work Cells subscribe only to fact classes they can soundly consume.

## 6. Exact facts versus abstract facts

This distinction is essential.

An interval such as:

```
x ∈ [2,3]
```

may be a **sound over-approximation** of all valid values, not an exact characterization.

Therefore the shared fabric must represent semantic polarity such as:

```
OVER_APPROXIMATION
UNDER_APPROXIMATION
EXACT_SET
LOWER_BOUND
UPPER_BOUND
NECESSARY_CONDITION
SUFFICIENT_CONDITION
```

Two sound over-approximations may safely intersect to get a stronger over-approximation.

But an over-approximation cannot automatically become an existence witness.

The fact algebra must know what inference direction each abstraction supports.

## 7. Reduction operators

For abstract domains `A` and `B`, a reduction/cooperation map might look like:

```
ρ_A→B : A -> Information(B)
ρ_B→A : B -> Information(A)
```

with soundness obligations relating concretizations.

The runtime repeatedly applies reductions until:

- fixed point;
- budget limit;
- contradiction;
- target obligation solved.

This is analogous to relational propagator fixed-point execution but now spans entire specialist solvers/domains.

## 8. Pairwise cooperation contracts

A new solver/domain should not gain unrestricted access to every other domain.

Instead define certified pairwise bridges:

```
Bridge(A,B) {
    projection_A_to_B,
    projection_B_to_A,
    soundness_proofs,
    precision/order semantics,
    termination/fairness metadata,
    supported fact classes
}
```

This extends the earlier `CombinationContract(T1,T2)` concept.

Some bridges may provide only safe partial information without a complete combined decision procedure.

So distinguish:

```
COMPLETE_COMBINATION
SOUND_COOPERATIVE_REDUCTION
HEURISTIC_PROPOSAL_ONLY
UNSUPPORTED
```

## 9. Shared facts can be proof-producing

A specialist should ideally export:

```
fact + certificate
```

Example:

```
x <= 12
certificate: LP dual/Farkas derivation
```

or:

```
x ∈ interval I
certificate: rigorous interval propagation
```

Another solver need not trust the producer if the fact is independently checked before entering the durable shared fact fabric.

For speed, candidate/unchecked facts may exist in a speculative layer but cannot become authority input until validated according to policy.

## 10. Monotone narrowing within a campaign world

Many shared facts naturally form an information order:

```
Top (no information)
   ⊒
[0,100]
   ⊒
[20,30]
   ⊒
{23}
   ⊒
Bottom (contradiction)
```

Within a fixed assumption world, cooperative propagation can often be monotone toward greater information/narrower concretization.

This connects directly to:

- domain-theoretic certified progress;
- constraint propagation;
- fixed-point recursion;
- search economy.

The root scheduler can measure actual lattice refinement where the domain provides it.

## 11. Non-monotone/retractable information

Learned facts under assumptions may later be invalidated when:

- assumption world changes;
- source package generation changes;
- speculative branch closes;
- non-conservative theory update occurs.

Therefore the permanent mathematical ledger remains immutable/generation-based, while active fact stores are context/version views with provenance allowing retraction/rebuild.

This is consistent with the earlier substrate-integration warning about contextual equality and monotone Datalog.

## 12. Conflict extraction

If combined reductions reach contradiction:

```
⊥
```

the system should recover the smallest/most useful supporting fact set and derive:

- conflict core;
- interpolant;
- nogood;
- theory lemma;
- domain-native infeasibility witness.

Thus cooperative propagation feeds the compact-witness/failure-compilation loop.

## 13. Federation architecture hypothesis

Potential runtime:

```
                Shared Fact Fabric
                /      |       \
               /       |        \
        Interval     SMT       MIP
        engine       engine    engine
            \          |         /
             \         |        /
              certified bridges
```

A fact may propagate through several domains:

```
SMT equality
  -> interval substitution
  -> tighter numeric bound
  -> MIP relaxation
  -> dual bound
  -> global contradiction
  -> SAT/SMT conflict lemma
```

Each edge is a certified semantic bridge.

This is far more powerful than one solver calling another as a black-box subroutine.

## 14. Search-economy implication

Not every possible cross-domain projection is worth computing.

The scheduler should estimate:

```
expected information refinement
expected downstream unlocks
bridge cost
checker cost
fanout
```

and schedule high-value fact exchanges first, while preserving fairness for completeness-critical bridges.

## 15. Relation to heterogeneous theory composition

There are now two distinct cooperation modes:

### Full logical combination

A theorem guarantees a complete/refutation-complete joint procedure under conditions.

Example:
- Nelson-Oppen / polite combination.

### Sound abstract cooperation

Domains exchange sound partial facts, improving each other's search, without claiming the combined procedure is complete.

Example:
- reduced products / bound propagation.

The project must preserve this distinction.

A very productive cooperative solver network may still return `UNKNOWN` if no complete route closes the obligation.

## 16. Core law

> **Specialist mathematical engines should exchange the strongest compact facts they can soundly project into one another's languages, through certified bridges, without surrendering their independent semantics.**

## 17. Open research

1. A general typed schema for facts, approximation polarity, and information order.
2. Certificate composition for facts derived through multiple abstract-domain bridges.
3. Automatic synthesis of reduction/bridge operators from theory morphisms or Galois connections.
4. Conditions guaranteeing termination/convergence of large cyclic cooperation networks.
5. Distributed/asynchronous sound propagation across Work Cells.
6. Exact handling of disjunctions/non-convex facts without exponential blowup.
7. Integration with versioned assumption worlds and contextual equality.
8. Fact-store indexing so millions of learned facts remain cheap to route.
9. Bridge minimization/interface extraction: communicate only observer-relevant information.
10. Automatic detection that a set of cooperative domains admits a stronger complete combination theorem.
11. Proof-producing reduced products and formally verified bridge checkers.
12. Extension beyond constraints to probability bounds, algebraic ideals, spectral bounds, and other domain-specific summaries.
