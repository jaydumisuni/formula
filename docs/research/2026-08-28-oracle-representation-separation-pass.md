# Oracle Representation and Separation Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Question

How should the project represent mathematical objects or feasible sets that are exponentially large or expensive to materialize, but admit efficient structured queries?

One answer is an **oracle representation**: the mathematical object is accessed through certified query operations rather than full enumeration.

## Separation-oracle evidence

For a convex set `K`, a strong separation oracle receives a point `s` and either:

- certifies `s ∈ K`, or
- returns a separating hyperplane proving that `s` lies outside `K`.

Classical Grötschel-Lovász-Schrijver results connect separation and optimization for convex sets under appropriate representation/bit-complexity assumptions. Ellipsoid/cutting-plane algorithms can optimize over sets described only through such an oracle.

Source:
- https://www.cs.cmu.edu/afs/cs.cmu.edu/academic/class/15859-f11/www/notes/lecture09.pdf

## Exponentially many constraints without materialization

Cutting-plane algorithms repeatedly:

```
solve current finite relaxation
       ↓
query separation oracle
       ↓
violated constraint found?
   yes -> add only that constraint
   no  -> current point satisfies full implicit family
```

There are concrete optimization problems where polynomial-time separation algorithms operate over exponentially large families of valid inequalities.

Sources:
- https://optimization-online.org/2021/05/8383/
- https://optimization-online.org/2001/07/360/

This establishes:

```
size(full mathematical relation)
        >>
size(active materialized representation)
```

without sacrificing exact semantics.

## Column generation / variable oracles

The dual pattern exists for enormous variable spaces: column generation keeps only a small active subset of variables and calls a pricing oracle to discover whether another variable/column can improve the solution.

Source:
- https://optimization-online.org/2003/02/602/

So both dimensions can be implicit:

```
exponentially many constraints
    -> separation oracle

exponentially many variables/constructions
    -> pricing/generation oracle
```

## Architecture-changing conclusion

The mathematical universe should support multiple representation classes:

```
EXTENSIONAL
    explicit objects/elements/constraints

SYMBOLIC
    BDD/automaton/polyhedron/generating function/etc.

GENERATIVE
    grammar / construction rules

ORACLE
    membership/separation/optimization/pricing queries

HYBRID
    materialized working set + oracle for remainder
```

A huge mathematical object does not need to be expanded simply because the logical set it denotes is huge.

## Oracle contract

An oracle is not just a callback.

It needs semantic metadata such as:

```
query_type
input_domain
output_semantics
completeness
soundness
certificate_family
bit/size bounds
complexity bounds
failure/unknown conditions
```

Example:

```
separate_K(s)
  -> Member(certificate)
   | SeparatingHyperplane(a, certificate)
   | Unknown(reason)
```

A heuristic violated-constraint finder may still be useful, but it cannot establish full membership unless completeness is certified.

## Relationship to search cells

A mathematical Work Cell can therefore represent an enormous family implicitly:

```
cell owns candidate family F
but stores only:
    symbolic/oracle representation
    active constraints
    learned cuts/nogoods
    query index
```

The campaign does not need RAM proportional to `|F|`.

## Query-interface primitive

A promoted mathematical package may expose capability through an oracle interface rather than direct evaluation:

```
membership(x)
separate(x)
optimize(c)
count(...)
find_witness(...)
price(...)
```

Different oracle methods can be related by certified reductions when theory permits.

## Search-economy implication

Oracle calls themselves become schedulable mathematical work:

```
possible query q
    ↓
expected search-space reduction / bound improvement
    ÷
oracle cost
```

This connects directly to the search-economy and symbolic-query-learning research.

## Black-boxing connection

A certified black-box component may naturally expose its behavior as an oracle rather than an explicit relation.

For example:

```
huge internal component
      ↓
black-box boundary
      ↓
separation/membership oracle for boundary relation
```

The full boundary relation can remain implicit if explicit elimination is too expensive.

## Core law

> **Materialize only what the current mathematical obligation requires; represent the rest by certified symbolic or oracle access whenever closure/query guarantees permit.**

## Open research

1. Certificate-producing separation/pricing oracles.
2. Automatic detection that an implicit family admits efficient separation or optimization.
3. Converting between oracle, symbolic, and explicit representations.
4. Cache/learning policies for oracle answers without invalidating semantic freshness.
5. Oracle composition across theory morphisms and reductions.
6. How to prove completeness of a separation oracle independently of its implementation.
7. Query-planning methods that minimize oracle calls while maximizing root-level mathematical progress.
