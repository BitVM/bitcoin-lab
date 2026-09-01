# Script type compatibility

The primitive READMEs distinguish opcode compatibility from deployability.
A fragment may use opcodes understood by several script versions and still be
too large, execute too many operations, or use too many stack items for a
particular spend type.

- **Bare script:** arbitrary bare scripts are generally non-standard even when
  consensus-valid. Treat this as regtest/miner-policy territory.
- **P2SH:** the redeem script must also fit the legacy pushed-element limit, so
  it is unsuitable for most table-heavy primitives.
- **P2WSH:** legacy script rules still apply, together with P2WSH policy limits.
- **Tapscript:** preferred for large experimental fragments, but transaction
  weight, script size, stack-item, and execution-budget rules still apply.
- **Any:** means the fragment does not rely on a version-specific opcode. It
  does not waive the limits of the enclosing script type.

Compatibility claims assume the caller supplies a valid terminal stack result
and any required signature checks for the actual output being spent.
