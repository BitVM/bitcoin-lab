# BitHash128

A custom 128-bit-input hash walk selecting SHA-256 or RIPEMD-160 at each bit,
followed by RIPEMD-160.

## Parameters

- Input length: fixed at 128 bits, one Script boolean per stack item.
- Output: fixed at 160 bits.
- Verification can either consume the input or save all bits to the altstack.
  There is no configurable default.

## Script metrics

| Fragment | Script size | Input witness |
| --- | ---: | ---: |
| `bithash_verify([0; 20])` | <!-- metric:bithash_verify -->793<!-- /metric:bithash_verify --> bytes | 128 boolean items |

Maximum combined depth is at least the 128 input items and is larger for the
altstack-preserving variant. The executable tests cover both branches.

## Security

This is a custom construction, not a standard hash function. Its 160-bit final
output caps generic collision resistance at 80 bits and preimage/second-preimage
resistance at 160 bits; no stronger standalone proof is claimed. “128” denotes
input bits, not 128-bit collision security.

## Script compatibility and standardness

The used hash/flow opcodes exist in legacy script and tapscript, but the large
branch count can violate legacy opcode and standardness limits. Tapscript is
the intended experimental target. Cleanstack is satisfied by the verify helper
when invoked with exactly the documented witness.

## Witness and hints

No hints are required. The witness supplies bit 127 deepest and bit 0 on top;
each item must be a canonical false/true Script value if malleability matters.
