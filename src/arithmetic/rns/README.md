# Residue number system arithmetic

Lookup-table addition, subtraction, and multiplication modulo the product of a
fixed pairwise-coprime modulus set.

## Parameters

- Moduli/default: `[4, 9, 25, 7, 11]`.
- Combined modulus: `69,300`; representation width: five residues.
- Addition/subtraction table size: 107 items each.
- Multiplication table size: 892 items.
- Both add/sub operands use ordinary residues. Multiplication requires the left
  operand in indexed-row form and the right operand in ordinary form.

## Script metrics

Each size includes table setup, the operation, table cleanup, and moving the
five results back from the altstack. Operand pushes and final verification are
excluded. Both operands together occupy ten witness items when witness-supplied.

| Operation | Locking fragment | Maximum stack items |
| --- | ---: | ---: |
| Add | <!-- metric:rns_add -->219<!-- /metric:rns_add --> bytes | 118 |
| Subtract | <!-- metric:rns_sub -->221<!-- /metric:rns_sub --> bytes | 118 |
| Multiply | <!-- metric:rns_mul -->1564<!-- /metric:rns_mul --> bytes | 903 |

## Security

No cryptographic security parameter. The encoding is not a commitment and is
unique only modulo 69,300. Operand range/canonicality must be enforced by the
caller when needed.

## Script compatibility and standardness

The opcode set is compatible with legacy script and tapscript. P2SH is not
suitable for the larger table scripts; P2WSH/tapscript feasibility depends on
the complete serialized script and policy. Multiplication stays below the
1,000 combined stack-item limit at 903 in the measured composition. Bare
arbitrary scripts are non-standard and cleanstack must be supplied by the
caller.

## Witness and hints

No hints are required. Ordinary residues are ordered with modulus 4 on top and
modulus 11 deepest. `rns_push_indexed_value` is required only for the left
multiplication operand.
