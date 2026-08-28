# Automatic Algorithm and Dynamic-Programming Synthesis Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Why this pass exists

The project aims to discover not only identities/formulas, but **new executable algorithms** that solve mathematical problems more efficiently than their original definitions or brute-force specifications.

Dynamic programming is one of the clearest examples of this transformation:

```
exponential recursive/enumerative definition
        ↓
discover reusable subproblem state
        ↓
prove safe pruning / optimal substructure
        ↓
memoize/tabulate
        ↓
polynomial or dramatically faster algorithm
```

Research shows that important portions of this process can already be synthesized automatically.

## 1. Algebraic Dynamic Programming

Algebraic Dynamic Programming (ADP) separates:

- a grammar/decomposition defining the search space;
- an evaluation algebra defining how candidate solutions are scored/combined;
- tabulation/memoization generated mechanically by the compiler.

Sources:
- https://pmc.ncbi.nlm.nih.gov/articles/PMC3582264/
- https://pmc.ncbi.nlm.nih.gov/articles/PMC1261154/
- https://doi.org/10.1016/j.tcs.2016.05.032

This means a declarative search-space definition can compile into an efficient DP implementation rather than explicitly enumerating every candidate tree.

## 2. Bellman's principle as an admissibility theorem

DP pruning is only valid when discarding non-optimal/intermediate candidates cannot remove something needed for the final optimum.

ADP formalizes this through Bellman's Principle at the evaluation-algebra level.

Conceptually:

```
reduce subproblem candidate set with h
      ↓
combine reduced sets
```

must produce the same final optimum/result as:

```
combine every candidate
      ↓
reduce only at end
```

When that law is established, exponential intermediate spaces can be thinned safely.

This is exactly the project's intended pattern:

> performance transformations require mathematical preservation conditions.

## 3. Search-space/evaluation separation

ADP allows the same combinatorial grammar/search space to be evaluated using different algebras:

```
minimize score
maximize score
count candidates
enumerate
sample
Pareto frontier
```

without rewriting the underlying decomposition.

This strongly reinforces the prior semiring-parametric/factorized-evaluation research.

The reusable object is often the **decomposition/search skeleton**, not one specific algorithm output.

## 4. MetHyl: synthesizing efficient DP by thinning

MetHyl and MetHyl+ automate the thinning theorem using program synthesis. They synthesize efficient dynamic-programming algorithms from inefficient specifications by discovering the information needed to safely discard dominated intermediate candidates.

Source:
- https://arxiv.org/abs/2202.12208

Reported evaluation:

- 37 tasks across 16 optimization problems;
- exponential speedups on 97.3% of tasks;
- generated algorithms as efficient as expert reference programs on 70.3% of tasks.

The important architectural pattern is:

```
brute-force solution generator
      ↓
identify dominance/thinning relation
      ↓
synthesize summaries needed to preserve correctness
      ↓
new DP algorithm
```

That is a direct narrow-domain instance of the project learning a new mathematical instruction from problem structure.

## 5. AutoLifter: synthesizing algorithmic paradigms

AutoLifter studies automatic application of divide-and-conquer-like algorithmic paradigms without depending on syntax-specific program transformations.

It decomposes the synthesis problem through component/variable elimination and synthesizes lifting functions that make efficient composition possible.

Source:
- https://arxiv.org/abs/2202.12193

Reported evaluation solved 82/96 tasks across six algorithmic paradigms.

The important abstraction is the **lifting problem**:

```
result for whole object
     ?=
combine(summary(left), summary(right))
```

When the original result lacks enough information for composition, synthesize an augmented summary:

```
summary(x) = (result(x), auxiliary_information(x))
```

such that:

```
summary(x ⊕ y)
    = combine(summary(x), summary(y))
```

This is extremely important to the project.

It means the machine can **invent the missing state representation that makes a fast algorithm possible**.

## 6. Bellmania: verified algorithm transformation

Bellmania uses deductive reasoning, refinement types, SMT, and synthesis to derive provably correct divide-and-conquer implementations of dynamic-programming algorithms from high-level specifications.

Source:
- https://cris.technion.ac.il/en/publications/deriving-divide-and-conquer-dynamic-programming-algorithms-using-

The generated implementations improve locality/parallelism while preserving semantic correctness.

This supports the project's two-stage optimization model:

```
mathematical algorithm derivation
       ↓
implementation transformation
       ↓
proof/refinement validation
```

## 7. AutoGen: discovering recursive access patterns

AutoGen automatically discovers cache-oblivious parallel recursive divide-and-conquer algorithms for classes of DP recurrences by analyzing the dependency/access pattern of a simple iterative description.

Source:
- https://researchconnect.stonybrook.edu/en/publications/autogen-automatic-discovery-of-efficient-recursive-divide-amp-con/

This is another representation-discovery result:

```
flat iteration order
       ↓
discover recursive geometric/dependency structure
       ↓
parallel cache-efficient algorithm
```

The mathematical recurrence remains the same while the execution decomposition changes radically.

## 8. Architecture-changing conclusion: synthesized sufficient state

One of the most valuable future metaprimitives may be:

```
SYNTHESIZE_SUMMARY(problem/composition)
```

Given a mathematical object family and composition operator, search for the smallest/cheapest state `S(x)` such that:

```
S(x ⊕ y) = C(S(x), S(y))
```

and the desired answer can be recovered from `S(x)`.

This can turn a global computation into incremental/divide-and-conquer/dynamic programming.

Examples of summaries might be:

- min/max values;
- boundary conditions;
- Pareto frontiers;
- finite automaton states;
- matrices/transfer operators;
- sufficient statistics;
- invariant signatures;
- partial polynomial coefficients;
- compact boundary relations.

The machine need not know this summary in advance—it can synthesize it.

## 9. Connection to blackboxing/open systems

This synthesized summary is closely related to blackboxing:

```
large subproblem
      ↓
small boundary summary
```

If the summary preserves exactly the information required for composition, internal details can be discarded from the upper-level algorithm.

So DP state synthesis, black-boxing, and interface extraction may be three manifestations of the same deeper operation:

> **find the smallest compositional interface sufficient for the requested observer/objective.**

This deserves future unification research.

## 10. Connection to semirings/factor graphs

If the decomposition graph is fixed and only the evaluation algebra changes, one synthesized structure can support:

```
existence
counting
minimum/maximum
probability
Pareto optimization
sampling
```

This makes the decomposition itself a highly reusable primitive.

## 11. Primitive-promotion opportunity

Suppose a campaign repeatedly solves a family by discovering the same summary/decomposition rule.

The system can promote:

```
ProblemFamily P
      ↓
Summary S
Composition C
Recovery R
Proof Bellman/composition law
      ↓
compiled DP primitive
```

Future `P` instances bypass the original exponential search entirely.

This is one of the strongest concrete forms of self-expansion found so far.

## 12. Proof obligations

A synthesized DP algorithm must establish at least:

```
coverage:
    decomposition generates all relevant candidates

summary sufficiency:
    composition using summaries preserves requested result

dominance/thinning safety:
    discarded candidates cannot affect final answer

base cases:
    correct

termination/index bounds:
    where required

recovery:
    final requested witness/value reconstructible
```

Performance improvement alone is never enough for promotion.

## 13. Core law

> **When a brute-force mathematical definition repeats structurally equivalent subproblems, search for the smallest certified state that makes those subproblems compositional.**

That state may itself be new mathematics.

## 14. Open research

1. General synthesis of sufficient compositional summaries beyond sequences/standard DP tasks.
2. Minimal-summary discovery relative to a declared grammar/cost model.
3. Connection between synthesized summaries and categorical/open-system blackboxing.
4. Proof-producing synthesis of Bellman/thinning conditions.
5. Automatic detection of overlapping subproblems from relational/hypergraph semantics.
6. Combining DP synthesis with treewidth/decomposition discovery.
7. Incrementalizing a synthesized DP primitive automatically after promotion.
8. Semiring/algebra product generation for multiobjective and counting variants.
9. Native parallel scheduling/work-span certification for generated DP algorithms.
10. Use of models only as optional proposal engines for summaries/paradigms, with deterministic synthesis/checking authority.
