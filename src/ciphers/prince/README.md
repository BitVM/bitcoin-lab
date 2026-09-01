# PRINCEv2

Bitcoin Script implementation of PRINCEv2 encryption for one 64-bit block with
a generation-time 128-bit key.

## Parameters

- Block size: fixed at 64 bits, represented by 16 nibbles.
- Key size: fixed at 128 bits and embedded in the generated locking fragment;
  no default key exists. The metric uses the all-zero key because key values
  can slightly affect push encodings.
- Encryption only; no public decryption script is currently exposed.

## Script metrics

The metric excludes the 16-nibble plaintext witness and output comparison.

| Fragment | Script size |
| --- | ---: |
| `prince_encrypt(0)` | `<!-- metric:prince_encrypt -->291044<!-- /metric:prince_encrypt -->` bytes |

Maximum depth is input/key-value dependent and is exercised by reference-vector
tests. The state itself occupies 16 stack items.

## Security

PRINCEv2 has a 128-bit key and 64-bit block. Exhaustive key search is nominally
128-bit, while generic block collisions occur around `2^32` blocks and codebook
coverage at `2^64`. This implementation makes no side-channel claim and should
not be treated as an authenticated-encryption mode.

## Script compatibility and standardness

The fragment uses legacy-compatible stack/arithmetic/flow opcodes, but its
generated size and opcode count make tapscript the practical research target.
P2SH, P2WSH, and bare standard use is generally unsuitable. A caller must
compare/consume all 16 output nibbles and leave a clean truthy stack.

## Witness and hints

No hints. The plaintext is 16 canonical nibbles with nibble 0 (the most
significant nibble) on top and nibble 15 deepest. The key is public in the
locking script.
