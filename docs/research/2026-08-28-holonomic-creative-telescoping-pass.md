# Holonomic Systems and Creative Telescoping Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Why this pass exists

The project is ultimately intended not merely to evaluate known formulas, but to **discover new executable mathematics**: compact recurrences, transforms, operators, algorithms, and relations derived from larger mathematical structures.

Holonomic systems and creative telescoping are one of the strongest existing examples of exactly that process.

## 1. Functions represented by annihilating operators

A holonomic/D-finite function or sequence can be represented through linear differential or recurrence operators that annihilate it.

Examples:

```
L(f) = 0
```

where `L` may be:

- a differential operator;
- a shift/recurrence operator;
- a q-shift operator;
- a multivariate Ore operator.

`HolonomicFunctions` and `ore_algebra` manipulate these operators and annihilating ideals directly.

Sources:
- https://risc.jku.at/sw/holonomicfunctions/
- https://github.com/mkauers/ore_algebra
- https://www.algebra.uni-linz.ac.at/people/mkauers/ore_algebra/generated/ore_algebra.ore_algebra.html

This is a major representation lesson:

> a mathematical object does not need to be represented by its values or explicit closed form; it can be represented by the operators/relations it satisfies.

## 2. Closure properties execute mathematics at the relation level

Holonomic systems are closed under many operations. Given annihilating systems for `f` and `g`, algorithms can derive annihilating systems for constructions such as:

```
f + g
f * g
substitutions
sequences ↔ generating functions
definite sums
definite integrals
```

Sources:
- https://risc.jku.at/sw/holonomicfunctions/
- https://www.dkcm.jku.at/publications/refereed/2013-01-02

This means the system can manipulate **descriptions of entire infinite objects** without expanding their values individually.

It is another strong example of finite symbolic representation of infinite mathematics.

## 3. Creative telescoping

Creative telescoping takes an expression involving a summation/integration variable and searches for an operator in the external/free variable such that:

```
P(F) = Δ_k(G)
```

for sums, or analogously:

```
P(F) = dG/dx
```

for integrals.

After summing/integrating over the internal variable, the telescoping/certificate term collapses or reduces to boundary terms, leaving a recurrence/differential equation for the whole definite sum/integral.

Sources:
- https://www.sciencedirect.com/science/article/pii/S0747717108800442
- https://www.sciencedirect.com/science/article/pii/037704279090042X
- https://www3.risc.jku.at/research/combinat/software/ergosum/RISC/HolonomicFunctions.html

Conceptually:

```
large object:
S(n) = Σ_k F(n,k)

      ↓ creative telescoping

compact operator:
P(n,S_n) S(n) = boundary/0

      + certificate relating P(F) to a telescoping difference
```

A potentially enormous computation has become a compact executable recurrence.

That is extremely close to the project's target pattern.

## 4. Certificate structure

Creative telescoping often yields a **telescoper** plus a certificate operator/function.

For example, in the summation setting an operator can be decomposed as:

```
P = P_telescope + Σ_i (E_i - 1) R_i
```

where the `(E_i - 1) R_i` terms telescope when summed.

Once the difficult search has found the operators, checking the relation can reduce to a finite algebraic identity.

Source:
- https://link.springer.com/article/10.1007/s00208-020-02028-y

Concrete supplementary material for creative-telescoping research even provides certificates that are checked by applying the Ore operators and simplifying the residual to zero.

Source:
- https://www.algebra.uni-linz.ac.at/people/mkauers/risc-stuff/residues/

This is exactly the producer/checker split:

```
expensive telescoper search
        ↓
operator + certificate
        ↓
cheap algebraic checker
```

## 5. Guessing versus proof

`ore_algebra` includes guessing tools that take finite sequence/function data and search for small recurrence/differential operators matching the samples.

Source:
- https://www.algebra.uni-linz.ac.at/people/mkauers/ore_algebra/generated/ore_algebra.guessing.html

For a sample sequence:

```
a_0, a_1, ..., a_N
```

it can search for an operator:

```
L(a) = 0
```

within bounded order/degree classes.

The documentation is explicit that this is **guessing**: a relation matching finite data is a candidate, not a theorem about the entire infinite sequence.

This gives the project a perfect staged architecture:

```
observations/data
      ↓
GUESS annihilating relation
      ↓
candidate operator
      ↓
derive/prove from certified semantics
      ↓
CERTIFIED operator
```

The machine can therefore use cheap data-driven relation discovery without poisoning the truth layer.

## 6. Relation-space discovery rather than one formula

An annihilating ideal can represent an entire family of differential/recurrence relations satisfied by an object.

This aligns with earlier Gröbner-basis research:

```
infinite relation family
       ↓
finite generating basis
```

So the machine's discovery target should not always be:

```
find formula F
```

It may be:

```
find the operator ideal / relation module that characterizes F
```

From that relation space the system can derive:

- recurrences;
- differential equations;
- asymptotics;
- evaluation algorithms;
- identities;
- transformations;
- boundary-value representations.

## 7. Compact relation -> executable primitive

Once a recurrence/differential operator is certified, it can become executable mathematics.

For a recurrence:

```
p_r(n) a_{n+r} + ... + p_0(n) a_n = 0
```

plus enough initial conditions, the system has a program for generating the sequence.

That primitive may later be specialized using:

- fast recurrence evaluation;
- binary splitting;
- matrix powering;
- asymptotic methods;
- modular images/reconstruction;
- SIMD/batched evaluation.

Thus creative telescoping can literally turn a mathematical derivation into a new executable instruction.

## 8. Identity proving

The holonomic systems approach shows that broad classes of identities involving sums/integrals/products of holonomic special functions can be verified in finitely many steps.

Source:
- https://www.sciencedirect.com/science/article/pii/037704279090042X

Typical pattern:

```
left side L
right side R
      ↓
derive annihilating operator(s) for both
      ↓
show same operator/system
      ↓
check enough initial/boundary conditions
      ↓
identity established
```

This is another representation change:

```
equality of complicated functions
        ↓
equality of finite operator descriptions + initial data
```

## 9. Closure graph as capability graph

A holonomic Theory Profile could expose rules such as:

```
D_FINITE(f)
D_FINITE(g)
    => D_FINITE(f+g)
    => D_FINITE(f*g)

HOLONOMIC(F(n,k))
    => maybe creative_telescoping over k
```

Then proving that an object lies in a structure class automatically unlocks operator algorithms, just as GAP structure inference unlocks specialized group algorithms.

This fits the project's structure-inference/capability-closure architecture perfectly.

## 10. Important limitations

Holonomic/D-finite methods are powerful precisely because they operate in a restricted structural class.

Not every mathematical function/sequence is holonomic.

Creative telescoping may fail, be extremely expensive, or return high-order/large certificates.

Recent reduction-based methods sometimes deliberately compute the telescoper without the larger certificate for speed, which is useful operationally but means certificate recovery/checking policy needs explicit treatment.

Source:
- https://www.sciencedirect.com/science/article/abs/pii/S0747717124000336

Therefore:

```
HOLONOMIC
```

must be a Theory Profile property, not a universal assumption.

## 11. Strong architectural law

> **Search for a compact operator/relation that generates or constrains an object, not only for a closed-form expression of the object itself.**

This substantially broadens what can count as a discovered mathematical formula.

A new mathematical primitive may naturally be:

```
- recurrence operator
- differential operator
- Ore operator ideal
- telescoper + certificate
- generating-function equation
```

rather than a human-readable equation.

## 12. Connection to the original vision

This is one of the clearest existing examples of:

```
large mathematical structure
      ↓
algorithmic mathematical discovery
      ↓
new compact relation
      ↓
finite certificate
      ↓
executable recurrence/operator
```

The project aims to generalize this behavior across many mathematical structures and then make every certified discovered relation available to future problem-solving campaigns.

## 13. Open research

1. Creative telescoping beyond holonomic/D-finite classes (differentially algebraic, difference fields, Sigma methods, etc.).
2. Certificate formats and independently verified checkers for Ore-algebra/telescoping results.
3. Automatic choice between guessing, symbolic derivation, telescoping, and direct proof.
4. Minimal/cost-optimal annihilating operator selection under declared cost models.
5. Relation-space indexing so a newly encountered function can be matched to known annihilating ideals/operators.
6. Incremental maintenance of recurrences when summands/integrands change.
7. Native compilation of certified recurrences/differential operators into high-performance CPU kernels.
8. Cross-domain transport: identify when an engineering/data problem can be expressed as a holonomic sequence/function problem.
9. Combining operator guessing from observed data with exact derivation from source semantics.
10. Generalizing `annihilator discovery` as a project-wide metaprimitive: search for operators/transforms under which an object maps to zero, fixed points, or simpler canonical structure.
