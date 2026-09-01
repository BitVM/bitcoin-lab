# BLAKE3

BLAKE3 hashing for messages up to one 1,024-byte chunk, implemented with the
tracked-stack u4/bigint machinery.

## Parameters

- Message length: `0..=1024` bytes, fixed at generation time.
- Limb width: `4..32`; default `29` in `blake3_compute_script`.
- Table mode is currently full tables. The documented metric uses a 64-byte
  message and 29-bit limbs.

## Script metrics

The metric covers the compute fragment only; message encoding and output
verification are separate helpers.

| Configuration | Compute script |
| --- | ---: |
| 64 bytes, 29-bit limbs | <!-- metric:blake3_64_limb29 -->77777<!-- /metric:blake3_64_limb29 --> bytes |

Maximum depth is parameter-dependent. Use
`maximum_number_of_altstack_elements_using_blake3` when composing other
altstack users; execution tests deliberately disable the default stack limit
for configurations that exceed it.

## Security

The 256-bit BLAKE3 output has generic 128-bit collision resistance and 256-bit
preimage/second-preimage resistance. Only the single-chunk-tree range supported
by this implementation is in scope.

## Script compatibility and standardness

Generated scripts are substantially beyond ordinary standard output templates
and can exceed normal stack/policy limits. Treat them as tapscript research
fragments; P2SH, P2WSH, and bare deployment is generally unsuitable. A complete
caller must verify the digest and leave a clean truthy result.

## Witness and hints

No cryptographic hints are required. The witness/input must already use the
limb layout selected at generation time; `blake3_push_message_script_with_limb`
documents and generates that encoding.
