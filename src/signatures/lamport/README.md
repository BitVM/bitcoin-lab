# Lamport 2-bit commitment

A one-time four-way HASH160 commitment that either commits to and returns a
2-bit value or proves membership without returning the value.

## Parameters

- Four secret preimages and their four HASH160 public commitments.
- Value range: `0..=3`; out-of-range commit values are clamped to 3 by the
  current script. There is no key-generation default.
- The metric uses preimages `secret0` through `secret3` and value 1.

## Script metrics

Serialized witness size includes the witness element count and per-item length
prefixes.

| Configuration | Locking script | Unlocking witness |
| --- | ---: | ---: |
| 2-bit commit | `<!-- metric:lamport_lock -->96<!-- /metric:lamport_lock -->` bytes | `<!-- metric:lamport_witness -->11<!-- /metric:lamport_witness -->` bytes |

Maximum depth is dominated by the two witness items plus four embedded hashes;
executable tests cover all four values and malformed witnesses.

## Security

Strictly one-time. Security is bounded by HASH160: at most 80-bit collision and
160-bit preimage/second-preimage resistance. Revealing a preimage makes that
branch public; key reuse can enable unintended alternate openings.

## Script compatibility and standardness

The small script is opcode-compatible with bare, P2SH, P2WSH, and tapscript,
subject to the enclosing output's policy. Arbitrary bare scripts are normally
non-standard. The commit variant intentionally leaves the selected value; the
caller decides whether to consume it or enforce cleanstack.

## Witness and hints

No hints. Commit witness order is `<value>` deepest, `<preimage>` on top. Reveal
mode supplies only `<preimage>`.
