# bitcoin-lab

Experimental Bitcoin Script arithmetic and cryptographic primitives.

## Layout

```text
src/
├── arithmetic/       # u4, u32, fixed-width bigint, and RNS
├── hashes/           # SHA-256, BLAKE3, and BitHash128
├── signatures/       # Lamport, HORS, and Winternitz OTS
├── ciphers/          # PRINCEv2
├── curves/           # BN254 fields, groups, and pairing
└── support/          # Script execution and shared pseudo-op helpers
```

Every primitive directory has a README covering parameters, measured script
and witness sizes, stack behavior, security assumptions, script-type
compatibility, standardness, and witness hints. Shared interpretation notes are
in [`docs/script-types.md`](docs/script-types.md) and
[`docs/standardness.md`](docs/standardness.md).

The cleaner domain paths are the canonical organization. Existing paths such
as `bitcoin_lab::u4`, `bitcoin_lab::hash`, and `bitcoin_lab::bn254` remain
available as compatibility re-exports.

## Metric snapshots

`tests/primitive_metrics.rs` computes documented script/witness sizes. Normal
tests fail if a snapshot is stale. After an intentional script change, update
the numeric README markers with:

```sh
UPDATE_PRIMITIVE_METRICS=1 cargo test --test primitive_metrics
```
