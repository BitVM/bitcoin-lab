# Winternitz one-time signatures

Base-16 HASH160 Winternitz signatures with list-pick, brute-force, and binary-
search verifiers plus typed APIs for fixed message lengths.

## Parameters

- Base: fixed at 16 (`LOG2_BASE = 4`).
- Typed message sizes: 4, 16, 32, 64, or 80 bytes. The documented default is
  `Wots32`.
- Verifier: list-pick for standard signatures, brute force for compact
  signatures; the lower-level API can select another verifier/converter.
- Public keys, signature digit counts, and checksum digits derive from the
  message length and base.

## Script metrics

The locking metric verifies a standard (non-compact) `Wots32` signature. The
witness metric uses a deterministic 20-byte secret and all-zero message and
includes witness serialization overhead.

| Configuration | Locking script | Unlocking witness |
| --- | ---: | ---: |
| `Wots32`, list-pick | <!-- metric:wots32_lock -->4908<!-- /metric:wots32_lock --> bytes | <!-- metric:wots32_witness -->1477<!-- /metric:wots32_witness --> bytes |

Maximum stack depth and size vary with verifier, message length, and compact
mode. The module's executable vectors cover all typed message lengths.

## Security

One-time only. HASH160 limits each chain to at most 80-bit collision and
160-bit preimage resistance; concrete multi-target security also depends on the
number of chains and key reuse. The checksum prevents monotone digit changes
under the Winternitz construction assumptions.

## Script compatibility and standardness

Opcode-compatible with legacy script and tapscript, but realistic keys produce
large scripts and witnesses. P2SH and bare script are unsuitable; P2WSH and
tapscript standardness must be checked for the selected message/verifier mode.
Verification can leave the recovered message or explicitly clear it; choose
the latter or add a consumer to meet cleanstack.

## Witness and hints

No auxiliary hints. A standard signature has a 20-byte chain value and digit
item per message/checksum digit. Compact signatures omit digit items and trade
witness size for brute-force verification work.
