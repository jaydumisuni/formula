# Verifier Equivalence Capability

Status: additive capability candidate over the finally frozen RSA Public Representation capability.

Frozen RSA Public Representation predecessor:
9f222f0ed40d6c654d282f3d4a337c34d2c62f4a

## Montgomery product equivalence

For odd modulus N > 1 and radix R = 2^radix_bits with R > N:

MontMul(a, b) is equivalent to:

a * b * R^-1 mod N

for 0 <= a,b < N.

The checker accepts a producer result only when it is canonical decimal, lies in [0, 2N), and a single conditional subtraction of N yields the exact mathematical Montgomery product. This captures implementations that defer one final subtraction without making their limb algorithm mathematical authority.

## Type-1 payload block semantics

A width-byte block is valid for an exact expected payload when it is:

00 01 || FF^PS || 00 || payload

with:
- width >= 11 bytes;
- PS length >= 8;
- every PS byte exactly FF;
- payload may be empty;
- every byte is constrained;
- input and expected payload hex are canonical lowercase hexadecimal.

This is a structural block-type checker only. It does not define ASN.1, digest algorithms, PKCS#1 signing, or any private-key operation.

## Integrated RSA public verifier

Given public representative S, exponent e, modulus N, and exact expected payload:

1. compute M = S^e mod N;
2. encode M to the modulus byte width;
3. require the recovered block to satisfy the Type-1 payload structure;
4. require exact payload equality.

The operation checks public verification semantics only.

## Xiaomi ginkgo donor evidence

Recovered donor:
jaydumisuni/Xiaomi-bootloader
commit c41e2a2442240d866757ce923ea287c2f069ac8d

Recovered verify.c establishes:
- 64 32-bit limbs;
- 2048-bit modulus;
- public exponent 65537;
- key[1] is the Montgomery low-word inverse because n0 * n0_prime == -1 mod 2^32;
- the stored 64-limb helper constant is R^2 mod N for R = 2^2048;
- signature verification uses the Montgomery multiply primitive and a fixed 65537 exponentiation chain;
- recovered block parser is exactly 00 01 FF...FF 00 payload;
- token_len <= 245, therefore at least 8 FF bytes for width 256.

Offline donor simulations:
- 406 full public-operation cases, including 114 representatives >= N: zero mismatches versus exact PowMod;
- 305 direct Montgomery-product cases: zero mismatches versus a*b*R^-1 mod N after at most one subtraction;
- 52 of those 305 donor multiply outputs were >= N before the caller's single subtraction;
- payload lengths 0, 1, 48, and 245 accepted; first-FF and separator mutations rejected; length 246 rejected.

Full verifier simulation artifact SHA-256:
e265d19c23f37c48874ed98bbdd4dee1644188a78fdc4feb2704f7fa96bfab0a

## Trust boundary

The generic exact arithmetic and RSA public-operation layers remain mathematical authority.

A Montgomery/firmware realization is accepted only through equivalence checking.

A Type-1 block is accepted only through exact structural checking and exact payload equality.

No private-key generation, signing, server emulation, vendor policy, or unlock state transition is introduced.
