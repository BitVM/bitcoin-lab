# BN254 pairing

Affine G2 preparation and a hinted four-pair Miller-loop/final-verification
path specialized for Groth16-style verification.

## Parameters

- Exactly four pairing terms in the current main entry point.
- Three G2 inputs are fixed/prepared constants; the fourth is witness supplied.
- `c`, `c_inv`, `wi`, four G1 evaluation points, Q4, and prepared line
  coefficients are generation-time or witness parameters as documented by
  `Pairing::hinted_quad_miller_loop_with_c_wi`.
- Ate loop count, twist type, and extension-field constants are fixed by BN254.
  There is no safe generic default instance.

## Script metrics

There is deliberately no single default byte count: the entry point is
instance-specific and returns both a script and its hints. Pairing tests print
and execute concrete-instance sizes; adding a protocol default should add a
metric marker to `tests/primitive_metrics.rs`. Maximum depth is likewise
instance-specific and current end-to-end tests use execution without the
default stack limit.

## Security

The pairing construction inherits BN254's roughly 100-bit security and the
soundness assumptions of the enclosing proof protocol. Correct prepared
coefficients, subgroup checks, non-degenerate points, and binding every hint are
essential.

## Script compatibility and standardness

This is tapscript-oriented research code. Full pairing scripts are not expected
to satisfy bare/P2SH/P2WSH standardness and may exceed ordinary stack/execution
limits. The enclosing verifier must supply the terminal comparison and
cleanstack behavior.

## Witness and hints

Hints are mandatory. They include intermediate base/extension-field values and
line-operation witnesses in the exact returned order. Q4 and protocol-specific
field/group inputs are also witness data; the three prepared constant G2 terms
are embedded by the generator.
