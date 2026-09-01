# u32 arithmetic

Generic 32-bit operations represented as four 8-bit Script integers.

## Parameters

- Word width: fixed at 32 bits; byte width is fixed at 8 bits.
- Most binary operations take operand depths; XOR/AND additionally take the
  current table/stack layout. No universal default exists.

## Script metrics

The representative metric adds the top two words and drops the originals.
Unlocking size and maximum depth depend on the surrounding composition.

| Fragment | Script size |
| --- | ---: |
| `u32_add_drop(0, 1)` | <!-- metric:u32_add_drop -->78<!-- /metric:u32_add_drop --> bytes |

## Security

No independent security parameter. Values are byte limbs; callers are
responsible for canonical/range-checked inputs when adversarial witnesses are
accepted.

## Script compatibility and standardness

Opcode-compatible with legacy script and tapscript. Table-heavy logic and hash
compositions may exceed the limits of bare script, P2SH, or P2WSH. Cleanstack
is the responsibility of the complete script.

## Witness and hints

No hints are required. A word occupies four witness/stack items, most
significant byte first in the module's normal push representation.
