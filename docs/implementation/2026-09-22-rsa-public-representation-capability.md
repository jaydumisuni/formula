# Fixed-Width Representation and RSA Public Operation Capability

Status: additive capability candidate over the finally frozen Modular Number Theory capability.

Frozen modular predecessor:
79755993f0e2a7c5d369d90c3a369433ee22c4b9

## Fixed-width unsigned big-endian representation

EncodeUnsignedBe(value, width_bytes):
- width_bytes must be >= 1;
- value must be non-negative;
- value must fit into exactly width_bytes;
- output is exactly 2 * width_bytes lowercase hexadecimal characters;
- leading zero bytes are preserved explicitly.

DecodeUnsignedBe(hex, width_bytes):
- width_bytes must be >= 1;
- input must be exactly 2 * width_bytes lowercase hexadecimal characters;
- result is the exact non-negative integer represented by those bytes.

## Generic RSA public operation

For:
S = representative
e = public exponent
N = modulus

the exact mathematical operation is:

M = S^e mod N

with:
- N > 1;
- e > 0;
- 0 <= S < N.

The output byte representation is the fixed-width unsigned big-endian encoding of M using the byte width of N:

k = ceil(bit_length(N) / 8)

The checker independently validates both the canonical decimal representative M and the exact k-byte lowercase-hex representation.

## Trust boundary

External producers remain untrusted.

formula-check independently computes the public operation and fixed-width representation.

Structural identity binds S, e, and N. Evidence identity binds the operation digest plus the producer decimal and fixed-width hexadecimal outputs.

## Explicit exclusions

This capability does not define:
- private-key operations;
- signature generation;
- PKCS#1 v1.5 padding;
- PSS;
- OAEP;
- encoded-message parsing;
- vendor token formats;
- bootloader policy;
- Montgomery implementation equivalence.

Those are separate layers. This capability proves only exact public modular arithmetic and representation semantics.
