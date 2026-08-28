# Research Pass — Theory Profiling, Completion, and Search Strategy Selection

**Date:** 2026-08-28  
**Status:** RESEARCH supplement  

This note records evidence for a new architectural hypothesis: before expensive mathematical search begins, the system should inspect the mathematical theory/problem region and determine which structural algorithms are valid, finite, complete, or likely to be useful.

---

## 1. Theory-property profiling is not speculative

Existing rewriting/formal systems already automate several properties that determine whether a mathematical/equational theory is executable in a strong sense.

Maude's tool ecosystem includes:

- Church-Rosser / confluence checking;
- termination checking;
- Knuth-Bendix-style completion;
- coherence checking/completion;
- sufficient-completeness checking;
- inductive theorem proving.

Sources:

https://maude.cs.illinois.edu/maude1/tools/

https://maude.cs.illinois.edu/tools

AProVE provides automated termination and complexity analysis for term rewriting and several programming-language formalisms.

Source:

https://github.com/aprove-developers/aprove-open-source

### Architectural implication

A future problem region can potentially be assigned a **Theory Profile** before search:

```text
termination:
    proven / disproven / unknown

confluence:
    proven / disproven / unknown

coherence:
    proven / unknown

canonicalization:
    complete / conditional / unavailable

finite variants:
    yes / no / unknown

complete invariant basis:
    available for declared class / unavailable / unknown

symmetry quotient:
    exact / partial / unavailable

abstraction:
    exact / sound-lossy / unavailable
```

Search strategy can then depend on established mathematical properties rather than fixed routing rules.

---

## 2. Canonical operational semantics requires structural proof

Maude's functional semantics relies on reducing terms to canonical forms. The desired coincidence between mathematical and operational semantics depends on properties such as termination and Church-Rosser/confluence (possibly modulo algebraic axioms).

Source:

https://maude.lcc.uma.es/maude31-manual-html/maude-manualch4.html

This is directly relevant to the unnamed project:

> A canonical-form engine should not be admitted simply because it appears to normalize examples. Its theory conditions need to be established or its limitations declared.

---

## 3. Completion can be an active repair operation

The Maude toolset contains not only checkers but also completion tools.

This supports a possible metaprimitive family:

```text
analyze_theory(T)
complete_theory(T)
canonicalize_under(T)
```

where completion attempts to transform an incomplete/non-confluent equational presentation into a usable canonical rewrite presentation when mathematically justified.

This should not be assumed always possible.

Failure or nontermination of completion is itself useful theory knowledge.

---

## 4. Automatic profiling should include complexity, not only correctness

AProVE demonstrates that automated analysis can derive termination and worst-case complexity properties for rewrite systems/programs.

This suggests Theory Profiles may eventually include:

```text
known asymptotic complexity
known termination measure
known branching behavior
known finite/infinite search property
```

Those fields can influence whether the system chooses:

- direct normalization;
- bounded equality saturation;
- narrowing;
- exhaustive enumeration;
- abstraction/refinement;
- solver reduction;
- distributed search.

---

## 5. Search routing should be structural

The strongest current routing hypothesis is:

```text
problem region
    -> infer mathematical/theory structure
    -> prove/check useful properties
    -> select strongest admissible structural method
    -> only then launch generic search
```

Examples:

```text
confluent + terminating rewrite theory
    -> canonical normalization

finite variant property
    -> complete variant-based inversion/unification

polynomial relation family
    -> ideal / Grobner-basis route

known exact symmetry
    -> quotient/canonicalize first

sound abstract domain only
    -> abstract -> solve -> refine

none of the above
    -> bounded/general search campaign
```

This may be one of the central reasons the final system can outperform naive brute-force combination of mathematical primitives.

---

## 6. Portable proof certificates already exist for theory properties

A major finding is that automated theory profilers do not have to be trusted directly.

The rewriting community uses the **Certification Problem Format (CPF)** as a machine-readable exchange format for proofs of properties including:

- termination / nontermination;
- confluence / nonconfluence;
- complexity;
- completion;
- safety and related properties.

CPF was created specifically so independent automated tools and certifiers could interoperate.

Sources:

https://isafor-ceta.uibk.ac.at/cpf3.html

https://doi.org/10.4204/EPTCS.167.8

**CeTA** is a certifier generated from the Isabelle/HOL formalization IsaFoR. It checks CPF certificates for rewriting properties.

Sources:

https://isafor-ceta.uibk.ac.at/

https://devel.isa-afp.org/entries/First_Order_Rewriting.html

CeTA's supported certificate boundary includes termination, confluence, complexity, completion, and related rewrite-system properties.

### Architectural implication

A Theory Profile should not contain only:

```text
termination = PROVEN
```

It should bind the claim to evidence such as:

```text
property: termination
status: PROVEN
producer: AProVE / other analyzer
certificate_format: CPF3
certificate_digest: ...
checker: CeTA
checker_lineage: IsaFoR / Isabelle-HOL
checker_result: CERTIFIED
```

The large analyzer remains replaceable. The portable certificate and independent checker establish the trusted result.

This is directly aligned with the project's broader solver-versus-verifier law.

---

## 7. The profile schema must separate property from proof method

Different domains may prove the same profile property using unrelated mathematics.

For example:

```text
termination = PROVEN
```

might be supported by:

- dependency-pair techniques and CPF/CeTA in a rewrite theory;
- a ranking-function certificate in another transition system;
- a direct algebraic argument in a specialized mathematical domain.

Therefore the schema should expose the mathematical property while preserving:

```text
proof method
certificate type
producer identity
independent checker identity
assumptions
scope
freshness
```

The profile is a semantic interface, not a universal proof language.

---

## 8. New research obligation

The next stage should investigate a common **Theory Profile schema** capable of representing properties obtained from unrelated mathematical engines without pretending they use the same internal proof method.

A useful initial direction is to study CPF as a donor for:

- portable proof-object design;
- extensible property families;
- separation of prover and certifier;
- graceful handling of proof techniques not yet supported by the certifier.

The project should not necessarily adopt CPF itself outside rewriting. The architectural lesson is more important: **standardize evidence interchange at the property boundary, not solver internals.**