# Modular Number Theory Capability

Status: additive post-P12 capability candidate

Frozen predecessor authority:
P12 proof boundary 75e3cb55c9099200d1b0f36c09b35e23a7b0a482
BOOTSTRAP_TRUST_REDUCED remains unchanged.

This capability extends the P11 exact BigInt checker without rewriting frozen P11/P12 authority.

## Operations

Mod(value, modulus)
MulMod(lhs, rhs, modulus)
PowMod(base, exponent, modulus)
Gcd(lhs, rhs)
ExtendedGcd(lhs, rhs)
ModInverse(value, modulus)

## Semantics

For Mod, MulMod and PowMod, modulus must be strictly positive.
Results use mathematical floor-mod semantics and therefore lie in [0, modulus).

PowMod requires exponent >= 0.

Gcd is always non-negative, including gcd(0,0) = 0.

ExtendedGcd accepts any canonical-decimal triple (gcd, x, y) satisfying:
gcd = gcd(lhs, rhs)
lhs*x + rhs*y = gcd

ModInverse requires modulus > 1 and gcd(value, modulus) = 1.
The accepted inverse is normalized to [0, modulus).

## Trust boundary

External producers remain untrusted.
formula-check independently recomputes or verifies every operation.
Malformed decimal text, invalid modulus, negative exponent, incorrect result, and non-invertible inverse fail closed.

## Structural identity

Every operation binds its operator and exact BigInt operands into canonical Formula structural identity.
Evidence receipts bind the operation digest and producer result text.

## Explicit exclusions

This capability does not yet define:
RSA public-operation semantics
encoded-message or padding semantics
fixed-width byte representation
Montgomery implementation equivalence
device token formats
vendor authorization policy

Those are higher layers over exact modular arithmetic.
