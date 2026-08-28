# Research Pass — Proof by Reflection, Verified Decision Procedures, and Native Mathematical Computation

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates how the unnamed mathematical project can combine theorem-level trust with high-performance executable computation without constructing enormous explicit proof terms for every routine calculation.

The central pattern is:

> **Prove/check the computational procedure once, then use computation as a proof mechanism for many concrete obligations.**

---

## 1. Proof by reflection connects propositions to executable decision procedures

Small-scale reflection/MathComp systematically relates logical propositions to computable Boolean predicates using reflection lemmas. Once the relation is proved, concrete instances can be established by computation.

Sources:

https://math-comp.github.io/mcb/snippets/ch5.html

https://www-sop.inria.fr/teams/marelle/advanced-coq-17/lesson1.html

### Architectural implication

A mathematical primitive can have two levels:

```text
semantic theorem:
    checker/decision procedure D is correct for theory fragment F

concrete execution:
    D(instance) = true/false
```

The expensive general soundness proof is reused across arbitrarily many concrete instances.

This can be much cheaper than producing a long bespoke proof trace for each instance.

---

## 2. Computation itself can be a stable proof mechanism in decidable domains

MathComp explicitly exploits the idea that when a concept is decidable, it can be represented as a program and closed propositions can be established by evaluating that program.

Source:

https://www-sop.inria.fr/teams/marelle/advanced-coq-17/lesson1.html

### Architectural implication

For a promoted deterministic primitive with a formally established decision contract, a query may use:

```text
execute certified decision procedure
```

instead of:

```text
invoke general theorem search
```

This is another route by which discovered/certified mathematics can become extremely cheap to reuse.

---

## 3. Proof assistants provide accelerated reduction engines for reflection

Rocq/Coq provides `vm_compute` and `native_compute`; native computation compiles evaluation to OCaml and is typically faster for large computational proofs.

Source:

https://rocq-prover.org/doc/v8.12/refman/proof-engine/tactics.html

### Architectural implication

The project should distinguish:

```text
logical correctness of the reflective procedure
```

from:

```text
execution mechanism used to evaluate it
```

A slow reference evaluator, bytecode evaluator, native CPU realization, or GPU realization can all implement the same established decision semantics if realization equivalence is verified.

---

## 4. Native evaluation can enlarge the trusted computing base if used naively

Lean's `native_decide` evaluates a `Decidable` instance using compiled native execution and is significantly faster on large computations, but current Lean documentation explicitly notes that this route introduces a native-computation axiom/trust dependency rather than being checked purely by kernel reduction.

Sources:

https://lean-lang.org/doc/reference/latest/Tactic-Proofs/Tactic-Reference/

https://lean-lang.org/doc/reference/latest/Axioms/

### Architectural implication

The project should not assume:

```text
native execution inside a proof assistant
==
small trusted kernel proof
```

For the highest-assurance route, prefer:

```text
fast untrusted/native computation
    -> certificate/result artifact
    -> independently verified checker/reflection path
```

rather than making the entire optimizing compiler/runtime part of mathematical truth authority.

---

## 5. Bitvector reflection shows domain-specific reflected checkers can be efficient

Lean's bitvector decision tooling contains a reflected verification layer with correctness theorems connecting certificate/checking computations to logical unsatisfiability/results.

Source:

https://lean-lang.org/doc/api/Std/Tactic/BVDecide/Reflect.html

### Architectural implication

The universal certificate envelope can coexist with **reflected domain checkers**:

```text
certificate/native result
    -> reflected checker
    -> theorem in formal kernel
```

A domain promoted to frequent use may justify building a reflected checker to reduce certificate replay overhead.

---

## 6. Reflection and certificate checking are complementary

Two trust patterns emerge:

### Reflective procedure

```text
prove D correct once
execute D(instance)
```

Best when:

- the decision procedure is compact enough to formalize;
- the fragment is decidable;
- concrete execution can happen efficiently.

### External producer + certificate checker

```text
complex solver S
    -> certificate C
small checked verifier V(C)
```

Best when:

- search/optimization is enormous;
- solver internals change frequently;
- compact witnesses exist;
- formalizing the entire producer would be costly.

### Architectural implication

Theory Profile / certificate catalogue should choose between these patterns per domain rather than require one global trust architecture.

---

## 7. Reflective procedures can become mathematical primitives themselves

Suppose the system repeatedly proves properties in fragment `F` using a generic solver and discovers a deterministic decision procedure `D`.

After proving:

```text
forall x in F:
    D(x) = true <-> Property(x)
```

`D` becomes an unusually powerful promoted primitive:

```text
future query in F
    -> execute D
    -> obtain proof/refutation cheaply
```

This is stronger than caching individual theorem results.

The machine has acquired a **decision algorithm** for an entire class of problems.

---

## 8. Reflection fits self-specialization

A generic certified relation can potentially be specialized to a fragment and then proved/reflected as a dedicated decision procedure:

```text
generic relation R
    -> identify recurring decidable fragment F
    -> specialize/supercompile R_F
    -> prove R_F decision theorem
    -> optimize native realization
    -> validate implementation
    -> promote reflective primitive
```

This is one possible end-state of primitive distillation.

---

## 9. Trust roots should be explicit and comparable

A concrete result may have different assurance routes:

```text
kernel reduction only

reflected checker proved in kernel

formally verified external checker

native computation with compiler/runtime trust

cross-implementation exact recomputation
```

### Architectural implication

Certificate envelopes should expose the trust-root class so downstream clients can demand an assurance threshold appropriate to the use case.

A trading analysis may accept one level while a new promoted mathematical theorem may require a stronger route before entering the permanent instruction set.

---

## 10. Current reflective-computation hypothesis

The project's fastest trusted path for mature mathematical fragments may eventually look like:

```text
MATHEMATICAL THEORY / FRAGMENT
    -> certified/reflected decision procedure
    -> specialized native implementation
    -> realization validation

CONCRETE QUERY
    -> native evaluate
    -> compact output/certificate
    -> reflective/independent check
    -> result
```

The proof search cost is paid during procedure discovery/certification, not on every invocation.

---

## 11. New research obligations

1. Study proof-by-reflection patterns for algebra, Presburger arithmetic, polynomial identities, graph properties, and rigorous numerics.
2. Determine when a reflective checker is cheaper than domain-native certificate replay.
3. Investigate formally verified compilation/runtime routes that allow native computation without enlarging the trust base significantly.
4. Study proof-producing JIT/AOT specialization of reflected decision procedures.
5. Define trust-tier policies for primitive promotion and client invocation.
6. Investigate how reflected procedures expose counterexamples/witnesses rather than only Boolean verdicts.
7. Determine how a reflected decision procedure is invalidated/versioned when its underlying theory/axioms change.
8. Study automatic synthesis of decision procedures followed by formal correctness proofs as a long-term primitive-discovery target.
9. Investigate extraction of certified checkers to Rust/C/native code with equivalence validation against the formal version.
10. Measure the crossover point among kernel reduction, bytecode evaluation, native reflection, and external certificate checking for representative mathematical domains.
