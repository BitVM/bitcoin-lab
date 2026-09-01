# BN254 fields

Prime fields `Fq` and `Fr` plus extension fields `Fq2`, `Fq6`, and `Fq12`.

## Parameters

- Field moduli: fixed by BN254; `Fq` and `Fr` expose their own constants.
- Representation: `BigIntImpl<254,29>`, nine little-endian limbs with a shorter
  head limb.
- Extension tower and non-residues: fixed by BN254/arkworks.
- Operand depths are generation-time parameters. There is no universal default;
  metrics use operands at depths 1 and 0.

## Script metrics

These are operation fragments and exclude operand pushes, witness hints, and
result checks.

| Fragment | Script size |
| --- | ---: |
| `Fq::add(1, 0)` | <!-- metric:fq_add -->415<!-- /metric:fq_add --> bytes |
| `Fr::add(1, 0)` | <!-- metric:fr_add -->415<!-- /metric:fr_add --> bytes |
| `Fq2::add(2, 0)` | <!-- metric:fq2_add -->846<!-- /metric:fq2_add --> bytes |

Maximum depth is operation-specific; hinted multiplication/inversion is much
larger than the representative additions.

## Security

Field operations provide correctness, not cryptographic security in isolation.
Canonical range checks are required at trust boundaries. The enclosing BN254
protocol has roughly 100-bit pairing security.

## Script compatibility and standardness

Basic additions may fit multiple script types, while hinted extension-field
operations can exceed legacy/P2WSH policy or execution limits. Tapscript is the
intended research target. Callers must arrange final cleanstack behavior.

## Witness and hints

Addition, subtraction, negation, and stack utilities need no hints. Hinted
multiplication, square, inversion, and Frobenius helpers return a script plus an
ordered `Vec<Hint>` that must be serialized into the witness before operands.
