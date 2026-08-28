# Research Pass — Cross-Domain Infeasibility Witnesses

**Date:** 2026-08-28  
**Status:** RESEARCH supplement  

This note extends the compact-witness research beyond SAT/SMT and tests whether the same principle—large failed mathematical world -> small independently checkable contradiction—appears in optimization, polynomial algebra, real algebraic geometry, and combinatorial encodings.

The answer is strongly yes, but the witness algebra is domain-specific.

---

## 1. Linear optimization has native dual certificates of impossibility

Farkas' lemma provides a classic alternative theorem for linear systems: either the primal constraints are feasible, or there exists a vector of multipliers establishing an explicit contradictory inequality.

CGAL's exact linear/quadratic solver exposes and demonstrates direct verification of such infeasibility certificates.

Sources:

https://doc.cgal.org/latest/QP_solver/index.html

https://www.cs.tau.ac.il/~efif/doc_output10/QP_solver/QP_solver_2infeasibility_certificate_8cpp-example.html

MOSEK likewise exposes Farkas-type certificates for infeasible linear optimization models.

Source:

https://docs.mosek.com/latest/pythonapi/tutorial-pinfeas-shared.html

### Architectural implication

In some domains an impossibility witness is not fundamentally a proof trace.

It is a compact mathematical object such as a multiplier vector `y` satisfying exact relations whose check directly establishes contradiction.

Possible envelope body:

```text
certificate_family: farkas_infeasibility
multipliers: ...
claimed_support: ...
exact_checker: ...
```

This is a strong example of a domain-native certificate that should not be translated into a generic logical proof before it can be useful.

---

## 2. Certificate support can itself identify a conflicting subsystem

For linear inequality systems, irreducible infeasible subsystem (IIS) algorithms identify a subset of constraints/bounds that is infeasible but becomes feasible if any one member is removed.

Gurobi documents an IIS method based on dual Farkas proofs: support of an appropriate extreme Farkas certificate yields an IIS.

Source:

https://support.gurobi.com/hc/en-us/articles/360041448572-How-does-Gurobi-compute-the-IIS-for-infeasible-models

### Architectural implication

The project should investigate a general pattern:

```text
certificate
    -> algebraic support/dependency set
    -> compact conflict core
```

rather than always running an independent combinatorial minimizer after proof.

In a domain where certificate coefficients or support have mathematical meaning, proof generation and explanation extraction can reinforce each other.

---

## 3. Polynomial equations have Nullstellensatz certificates

Hilbert's Nullstellensatz yields an algebraic certificate that a polynomial system over an algebraically closed field has no common zero.

A certificate can express `1` as a polynomial combination of the input equations, so any common zero would imply the contradiction `1 = 0`.

Nullstellensatz certificates have been used computationally to prove infeasibility of combinatorial encodings such as graph colorability problems.

Source:

https://www.sciencedirect.com/science/article/pii/S0747717111001192

Isabelle contains formalized Gröbner/Nullstellensatz machinery capable of producing certificate data for polynomial ideal/refutation reasoning.

Sources:

https://isabelle.in.tum.de/website-Isabelle2023/dist/library/HOL/HOL/ISABELLE_HOME/src/HOL/Tools/groebner.ML.html

https://devel.isa-afp.org/entries/Nullstellensatz.html

### Architectural implication

For polynomial equality systems, the compact witness language can be **algebraic combination itself**:

```text
p1 * f1 + p2 * f2 + ... + pn * fn = 1
```

where the `fi` are original constraints and the `pi` are certificate polynomials.

The support `{fi | pi != 0}` provides an immediate dependency/core candidate.

This suggests that the project's generic provenance layer should preserve domain-native coefficient/support structure rather than reducing everything to opaque evidence references.

---

## 4. Polynomial inequalities have Positivstellensatz / sum-of-squares certificates

Real semialgebraic infeasibility can be certified using Positivstellensatz identities combining:

- polynomial equality constraints through ideals;
- inequality constraints through sums of squares / preorder or quadratic-module structure;
- an explicit contradiction such as `-1` belonging to the generated cone/module.

Isabelle includes tooling to serialize and read Positivstellensatz certificates and formal real-arithmetic procedures using these objects.

Sources:

https://isabelle.in.tum.de/website-Isabelle2025-1/dist/library/HOL/HOL-Analysis/ISABELLE_HOME/src/HOL/Library/Sum_of_Squares/positivstellensatz_tools.ML.html

https://isabelle.in.tum.de/website-Isabelle2023/dist/library/HOL/HOL-Analysis/ISABELLE_HOME/src/HOL/Library/Sum_of_Squares/positivstellensatz.ML.html

Lean's current sum-of-squares tooling similarly searches externally and checks exact SOS/Putinar-style certificates inside Lean; infeasibility is represented by a target `-1` certificate.

Source:

https://github.com/leanprover/sos

### Architectural implication

A future mathematical work cell might use an untrusted SDP/SOS engine to search for a certificate, but the admitted result can be exact algebraic data checked by the project's certification layer or a proof assistant.

Again:

```text
search may be numerical / heuristic
certificate admission is exact
```

---

## 5. Sparse certificate structure can preserve representation locality

Sparse Positivstellensatz results show that, under structural assumptions such as running-intersection properties, positivity/infeasibility certificates can preserve the sparsity/local-variable structure of the original problem.

Representative source:

https://pubsonline.informs.org/doi/abs/10.1287/moor.2022.1284

A 2025/2026 Mathematical Programming result similarly discusses sparse Positivstellensatz structure for sparse polynomial optimization.

Source:

https://link.springer.com/article/10.1007/s10107-025-02223-2

### Architectural implication

Compact witness search should exploit Theory Profile structure such as sparsity and decomposition rather than only minimize certificate byte size after the fact.

A structurally local certificate can be more valuable than an equally small but globally entangled one because it gives:

- better conflict localization;
- cheaper replay;
- stronger decomposition information;
- better candidate-space pruning.

---

## 6. CAD/nonlinear decision procedures also support sceptical external computation

Formal work on univariate real polynomial decision procedures uses efficient untrusted external computation to generate certificate material while Isabelle/HOL performs verified checking.

Source:

https://doi.org/10.1007/s10817-017-9424-6

The authors explicitly compare certificate styles by both:

- how much mathematics/computation is needed to construct the certificate;
- how much trusted/verified computation is required to check it.

### Architectural implication

The project's certificate catalogue should include **certificate economics**:

```text
search cost
certificate size
checking cost
formalization/trust cost
pruning/explanation value
```

The mathematically strongest certificate is not automatically the best operational artifact if it is impossibly expensive to construct or verify.

---

## 7. The same abstract pattern spans very different witness algebras

The domain-native forms differ:

```text
SAT/SMT
    proof trace / core / interpolant

linear optimization
    Farkas multiplier vector

polynomial equalities
    Nullstellensatz polynomial combination

polynomial inequalities
    Positivstellensatz / SOS identity

graph canonization
    canonical-label proof certificate

rigorous numerics
    interval/root/integration certificate
```

But each can fit the same higher-level role:

```text
Mathematical Impossibility Witness
    claim/world identity
    assumption scope
    native witness family
    exact dependencies/support
    checker
    verdict
    optional compact-core/minimality claim
```

This strengthens the universal-certificate-envelope hypothesis while arguing strongly against a universal certificate body.

---

## 8. A new possible metaprimitive: certificate-support extraction

The project should investigate an operation roughly like:

```text
support(certificate)
```

where domain semantics determine which original assumptions/constraints materially participate in the certified claim.

Possible outputs:

- clause dependency core;
- nonzero Farkas multiplier support;
- nonzero Nullstellensatz multiplier support;
- Positivstellensatz constraint/SOS block support;
- e-graph explanation dependencies;
- proof-assistant theorem dependencies.

This support is not automatically minimal.

A second operation may attempt:

```text
minimize_support(certificate_or_claim)
```

with its own certificate/minimality obligations.

---

## 9. Current cross-domain synthesis

The compact-witness design should probably separate four concepts:

```text
NATIVE CERTIFICATE
    proves/refutes the mathematical claim

CERTIFICATE SUPPORT
    identifies contributing assumptions/dependencies

IRREDUCIBLE / MINIMAL CORE
    claims no listed dependency can be removed

GENERALIZED BOUNDARY / NOGOOD
    derives a reusable exclusion/invariant from the failure
```

A single solver output may produce only the first.

Further work cells can derive the others when their expected future value justifies the cost.

---

## 10. New research obligations

1. Investigate whether certificate-support extraction has common algebraic abstractions across linear, polynomial, logical, and equality-saturation certificates.
2. Determine when support of an extreme/vertex certificate guarantees irreducibility and when it does not.
3. Investigate exact rational reconstruction for certificates produced initially by floating-point LP/SDP/SOS solvers.
4. Study certificate-size/degree lower bounds as predictors of search difficulty and scheduler allocation.
5. Determine how sparse/domain-decomposed certificates should feed the AND/OR campaign graph.
6. Investigate whether algebraic certificates can directly generate interpolant-like learned boundaries between assumption worlds.
7. Study minimal infeasible subsystems for nonlinear/semialgebraic constraint systems.
8. Determine how certificate support should interact with provenance when the same claim has several independent proofs with different supports.
