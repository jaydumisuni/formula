# Research Pass — Probabilistic/Stochastic Semantics, Exact Inference, and Certified Quantitative Claims

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates how the unnamed mathematical project should represent and solve genuinely probabilistic/stochastic problems without weakening the core rule that mathematical truth does not depend on models, heuristics, or informal confidence.

The strongest conclusion is:

> **Random execution does not imply probabilistic truth authority. A probability distribution, Markov kernel, expectation, reachability probability, or stochastic-process property can itself be an exact mathematical object with exact or certified quantitative claims.**

The system must distinguish stochastic mathematics from empirical/statistical uncertainty and from randomized search algorithms.

---

## 1. Probability kernels provide a compositional mathematical semantics

Measure-theoretic probability represents a stochastic transition from space `A` to space `B` as a **Markov kernel**:

```text
κ : A -> ProbabilityMeasure(B)
```

Mathlib formalizes kernels as measurable maps from inputs to measures, with Markov-kernel structure asserting that each resulting measure is a probability measure.

Sources:

https://leanprover-community.github.io/mathlib4_docs/Mathlib/Probability/Kernel/Defs.html

https://leanprover-community.github.io/mathlib_docs/probability/kernel/basic.html

### Architectural implication

The relational semantic core can naturally include stochastic relations:

```text
Deterministic relation:
    R(x,y)

Stochastic relation/kernel:
    K(x, distribution_over_y)
```

Composition remains mathematical rather than operationally random.

The same semantic kernel can have many sampling/inference realizations.

---

## 2. Conditional probability is itself another kernel/interface

Mathlib formalizes regular conditional distributions as Markov kernels under appropriate measurable-space conditions.

Sources:

https://leanprover-community.github.io/mathlib4_docs/Mathlib/Probability/Kernel/CondDistrib.html

https://leanprover-community.github.io/blog/posts/basic-probability-in-mathlib/

### Architectural implication

Conditioning should be treated as a mathematically constrained transformation of probabilistic structure, not merely an inference-engine side effect.

The Theory Profile may need properties such as:

```text
measurable_space
standard_borel
finite/s-finite kernel
conditional_distribution_exists
```

because certain constructions exist only under specific structural assumptions.

---

## 3. Exact probabilistic inference is possible for substantial discrete programs

**Dice** compiles discrete probabilistic programs into weighted model counting (WMC). The compilation is compositional and is proved correct with respect to a denotational semantics.

The method exploits logical/program structure and can perform exact inference on programs with very large numbers of random variables when the compiled representation remains compact.

Sources:

https://arxiv.org/abs/2005.09089

https://github.com/SHoltzen/dice

### Architectural implication

A probabilistic program need not imply Monte-Carlo sampling.

Possible route:

```text
probabilistic semantic program
    -> compile discrete structure
    -> BDD / weighted Boolean representation
    -> exact weighted model count
    -> exact probability
```

This directly connects stochastic mathematics to the project's existing compact symbolic-space research.

---

## 4. Weighted model counting separates logical structure from numeric weights

Dice's WMC reduction separates the Boolean/logical structure of a probabilistic program from its probability parameters.

Source:

https://arxiv.org/abs/2005.09089

Recent 2026 imprecise-probabilistic-programming work further shows that the same BDD/WMC pipeline can be parameterized by different semirings to support exact, differentiable, and interval/imprecise inference.

Source:

https://arxiv.org/abs/2607.20801

### Architectural implication

This is another instance of a project-wide law:

```text
semantic structure
    !=
numeric realization/algebra used to evaluate it
```

A compiled probabilistic structure may support multiple evaluations:

```text
exact rational probability
interval probability
symbolic/parametric probability
derivative/sensitivity
credal/imprecise bounds
```

without recompiling the logical structure from scratch.

---

## 5. Imprecise probability should represent epistemic uncertainty explicitly

Standard probability gives one distribution. Imprecise probability can represent a **set of admissible distributions** (credal set), separating lack of knowledge from randomness inside a known distribution.

Recent 2026 work gives an executable probabilistic-programming semantics where named epistemic uncertainties are tracked and the same BDD/WMC pipeline computes interval/credal results.

Source:

https://arxiv.org/abs/2607.20801

### Architectural implication

The project should distinguish at least:

```text
ALEATORY UNCERTAINTY
    randomness specified by a probability law

EPISTEMIC UNCERTAINTY
    law/parameters not uniquely known

EMPIRICAL ESTIMATION UNCERTAINTY
    finite-data evidence about an unknown process
```

Collapsing all three into one probability number would be mathematically dishonest.

---

## 6. Probabilistic model checking produces quantitative mathematical claims

PRISM supports formal analysis of stochastic models including:

- DTMCs;
- CTMCs;
- MDPs;
- reachability probabilities;
- expected rewards/costs;
- long-run/steady-state behavior.

Sources:

https://www.prismmodelchecker.org/manual/PropertySpecification/AllOnOnePage

https://www.prismmodelchecker.org/bibitem.php?key=KNP09a

### Architectural implication

The result class can include claims such as:

```text
Probability(reach Target) >= 0.999
ExpectedCostUntil(Target) <= 12
```

These are mathematical properties of the declared stochastic model, not empirical confidence statements.

---

## 7. Exact rational probabilistic model checking is practical in supported fragments

Storm supports exact rational arithmetic for supported model/property combinations via exact mode, avoiding floating-point approximation.

Source:

https://www.stormchecker.org/documentation/usage/troubleshooting.html

Storm's DRN representation explicitly supports numeric value types such as:

```text
double
interval-double
rational
rational-interval
parametric
```

Source:

https://www.stormchecker.org/documentation/background/drn.html

### Architectural implication

Probabilistic result status needs an exactness axis:

```text
EXACT_RATIONAL
EXACT_SYMBOLIC/PARAMETRIC
RIGOROUS_INTERVAL
SOUND_NUMERICAL_BOUND
FLOATING_APPROXIMATION
STATISTICAL_ESTIMATE
```

A probability being between 0 and 1 does not itself indicate how trustworthy/precise the numeric result is.

---

## 8. Parametric stochastic models can yield exact symbolic formulas

Storm can compute closed-form rational functions for reachability probabilities of parametric Markov models.

Source:

https://www.stormchecker.org/documentation/usage/running-storm-on-parametric-models

PRISM also supports parametric model checking with rational-function probabilities/rates in supported model classes.

Source:

https://www.prismmodelchecker.org/manual/RunningPRISM/ParametricModelChecking

### Architectural implication

The system may transform:

```text
stochastic state machine with unknown parameters
```

into:

```text
exact executable/symbolic probability formula
```

This is directly aligned with the broader project vision: a complex probabilistic process can collapse into a reusable mathematical construction.

---

## 9. Probabilistic verification can be certificate-bearing

2025 work develops **fixed-point certificates** for quantitative reachability and expected-reward properties in finite MDPs. The certificates are designed to be lightweight/easy to check, their soundness is formalized in Isabelle/HOL, and Storm was extended to emit them.

Source:

https://arxiv.org/abs/2501.11467

### Architectural implication

The project does not need to trust a large probabilistic model checker.

Possible envelope:

```text
claim:
    Pmax(reach Target) <= q

producer:
    Storm / other engine

certificate:
    fixed-point witness

checker:
    independently/formally verified checker
```

This extends the universal certificate-envelope pattern into stochastic mathematics.

---

## 10. Supermartingales are compact certificates for stochastic behavior

Probabilistic program verification uses ranking supermartingales and related certificates for properties such as:

- almost-sure termination;
- finite expected termination;
- quantitative termination bounds;
- safety/stability;
- omega-regular properties;
- cost bounds.

Sources:

https://link.springer.com/chapter/10.1007/978-3-031-13185-1_4

https://link.springer.com/chapter/10.1007/978-3-031-98679-6_2

### Architectural implication

A stochastic/infinite execution need not be verified by enumerating all random paths.

A compact mathematical function/witness can summarize the required global property.

This mirrors ranking functions for deterministic termination and strengthens the project's compact-witness doctrine.

---

## 11. Randomized algorithms and probabilistic semantic claims are different

Earlier fast-exact research distinguished:

```text
LAS VEGAS ALGORITHM
    random runtime/path
    output correctness exact

MONTE CARLO ALGORITHM
    nonzero probability of wrong verdict
```

This pass adds a third concept:

```text
STOCHASTIC MATHEMATICAL OBJECT
    output itself is a probability law/distribution/property
```

### Architectural implication

The project must track these independently.

Example:

```text
A randomized exact factorization algorithm
```

may be Las Vegas and return a deterministically checkable factorization.

A model saying:

```text
failure_probability = 1/1000
```

is a deterministic mathematical claim **about a stochastic system**.

A Monte Carlo estimator reporting:

```text
estimated failure probability = 0.001 ± ...
```

is an empirical/statistical result.

These three must never share one truth status.

---

## 12. Sampling is a realization, not the semantic distribution

A probability distribution/kernel defines mathematical mass/measure. A sampler is one algorithm for generating observations according to that law.

### Architectural implication

Separate:

```text
Distribution semantic identity
    exact law / kernel

Sampling realization
    pseudorandom generator
    seed policy
    algorithm
    hardware implementation
```

A faster sampler can replace a slower one without changing the mathematical distribution **if distributional equivalence/correctness is established**.

The random seed belongs to an execution/replay record, not to the distribution's mathematical identity.

---

## 13. Reproducible random search and stochastic mathematics can coexist

The search-economy research already identified counter-based/replayable randomness as a way to make adaptive exploration reproducible.

### Architectural implication

The project should separate:

```text
RANDOMNESS USED TO SEARCH FOR MATHEMATICS
    operational/replay concern

RANDOMNESS INSIDE THE MATHEMATICAL OBJECT
    probability/measure semantics
```

The former can be replayed by seed/counter and has no truth authority.

The latter is part of the mathematical statement and requires probability-theoretic certification.

---

## 14. Almost-everywhere equality introduces another semantic equality class

Measure-theoretic mathematics frequently identifies functions/properties up to sets of measure zero. Mathlib's probability library uses almost-everywhere equality extensively in conditional expectation/distribution theory.

Sources:

https://leanprover-community.github.io/mathlib_docs/probability/notation.html

https://leanprover-community.github.io/mathlib4_docs/Mathlib/Probability/Kernel/CondDistrib.html

### Architectural implication

The semantic equality system must eventually support more than ordinary pointwise equality:

```text
exact structural equality
pointwise mathematical equality
almost-everywhere equality
equality in distribution
bisimulation/equivalence
approximate/error-bounded equivalence
```

These are not interchangeable.

This further supports typed/domain-specific equivalence relations rather than one universal `=` implementation.

---

## 15. Probabilistic composition has structure requirements

Mathlib distinguishes finite, s-finite, Markov, and related kernel classes; some closure/composition properties require stronger boundedness/measure conditions.

Source:

https://leanprover-community.github.io/mathlib4_docs/Mathlib/Probability/Kernel/Defs.html

### Architectural implication

Probability structures should participate in the structure-witness/type system:

```text
Kernel(A,B)
IsMarkovKernel(K)
IsFiniteKernel(K)
...
```

and composition should generate/check appropriate measurability/finiteness obligations before being admitted.

---

## 16. Current stochastic-semantic hypothesis

```text
STOCHASTIC SEMANTIC OBJECT
    probability measure / distribution
    Markov kernel
    stochastic transition system
    stochastic process
        |
        v
QUERY / PROPERTY
    probability
    expectation
    reachability
    reward/cost
    almost-sure property
    conditional law
    optimization over policy
        |
        v
SOLVER REALIZATIONS
    exact WMC/BDD
    rational linear algebra
    parametric symbolic solver
    rigorous interval solver
    numerical approximation
    statistical sampler
        |
        v
CERTIFICATE / RESULT CLASS
    exact
    fixed-point certificate
    supermartingale certificate
    rigorous interval
    probabilistic algorithmic guarantee
    empirical estimate
```

The semantic object and the inference algorithm remain separate.

---

## 17. New research obligations

1. Study semiring/algebraic model counting as a generic evaluation substrate beyond ordinary probabilities.
2. Investigate proof/certificate formats for weighted model counting and exact discrete probabilistic inference.
3. Study formal semantics and certification for continuous probabilistic programs, not only finite/discrete models.
4. Investigate probability-kernel composition and categorical/Giry-monad structure as a possible reusable semantic layer without forcing every client to use category terminology.
5. Define precise truth/result classes separating exact probability, rigorous bounds, Monte Carlo confidence, and empirical statistical inference.
6. Study imprecise probabilities/credal sets as a native representation of epistemic uncertainty.
7. Investigate verified/sound probabilistic model checking for infinite/countable state spaces.
8. Study automatic synthesis of supermartingale/barrier certificates as mathematical Work Cell families.
9. Investigate probabilistic termination/productivity Theory Profile properties.
10. Study certified distributional equivalence between alternative samplers/implementations.
11. Define equality-in-distribution and almost-everywhere equality in the semantic equivalence system.
12. Investigate exact parametric probability formula discovery as a primitive-promotion path.
13. Study probabilistic programs with nondeterministic schedulers/MDPs and how `min/max` policy semantics fit the relation model.
14. Determine how stochastic semantic objects interact with theory morphisms, package composition, and semantic interfaces.
15. Investigate how empirical datasets should be translated into stochastic assumptions without promoting statistical patterns to mathematical truth.
