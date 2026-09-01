use bitcoin::hashes::{sha256, ripemd160, Hash};
use crate::treepp::{script, Script};

/// Compute BitHash128 in Rust (reference implementation).
/// bits[0] = bit0 (processed first), bits[127] = bit127 (processed last).
pub fn bithash_compute(bits: &[u8; 128]) -> [u8; 20] {
    let mut state: Vec<u8> = vec![]; // empty initial value (OP_0)
    for i in 0..128 {
        state = if bits[i] != 0 {
            ripemd160::Hash::hash(&state).to_byte_array().to_vec()
        } else {
            sha256::Hash::hash(&state).to_byte_array().to_vec()
        };
    }
    ripemd160::Hash::hash(&state).to_byte_array()
}

/// Verify BitHash(bits) == expected_hash and consume all bits.
/// Stack before: bit0 (top), bit1, ..., bit127 (deepest).
/// The initial OP_0 state is pushed by the script itself.
pub fn bithash_verify(expected_hash: [u8; 20]) -> Script {
    script! {
        OP_0    // initial state (empty bytes)
        for _ in 0..128 {
            OP_SWAP    // bring next bit to top
            OP_IF
                OP_RIPEMD160
            OP_ELSE
                OP_SHA256
            OP_ENDIF
        }
        OP_RIPEMD160   // final hash → 20-byte output
        { expected_hash.to_vec() }
        OP_EQUALVERIFY
        OP_1
    }
}

/// Altstack variant: saves each bit to altstack during hashing so bits can be retrieved afterward.
/// After script: hash verified, bits are on altstack (bit0 on top of altstack, bit127 deepest).
pub fn bithash_verify_save_to_altstack(expected_hash: [u8; 20]) -> Script {
    script! {
        OP_0    // initial state
        for _ in 0..128 {
            OP_SWAP
            OP_DUP
            OP_TOALTSTACK   // save bit for reuse
            OP_IF
                OP_RIPEMD160
            OP_ELSE
                OP_SHA256
            OP_ENDIF
        }
        OP_RIPEMD160
        { expected_hash.to_vec() }
        OP_EQUALVERIFY
        OP_1
    }
}

/// Just compute BitHash and leave the 20-byte hash on the stack (no verification).
/// Stack before: bit0 (top), bit1, ..., bit127 (deepest).
pub fn bithash_compute_script() -> Script {
    script! {
        OP_0
        for _ in 0..128 {
            OP_SWAP
            OP_IF
                OP_RIPEMD160
            OP_ELSE
                OP_SHA256
            OP_ENDIF
        }
        OP_RIPEMD160
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execute_script_with_inputs;

    /// Build witness: witness[0] pushed first (deepest) = bit127, witness[127] last (top) = bit0.
    fn bits_to_witness(bits: &[u8; 128]) -> Vec<Vec<u8>> {
        (0..128).rev().map(|i| {
            if bits[i] != 0 { vec![1u8] } else { vec![] }
        }).collect()
    }

    #[test]
    fn test_bithash_all_zeros() {
        let bits = [0u8; 128];
        let expected = bithash_compute(&bits);
        let witness = bits_to_witness(&bits);
        let s = bithash_verify(expected);
        let result = execute_script_with_inputs(s, witness);
        assert!(result.success, "all-zeros failed: {:?}", result);
    }

    #[test]
    fn test_bithash_all_ones() {
        let bits = [1u8; 128];
        let expected = bithash_compute(&bits);
        let witness = bits_to_witness(&bits);
        let s = bithash_verify(expected);
        let result = execute_script_with_inputs(s, witness);
        assert!(result.success, "all-ones failed: {:?}", result);
    }

    #[test]
    fn test_bithash_alternating() {
        let mut bits = [0u8; 128];
        for i in (0..128).step_by(2) { bits[i] = 1; }
        let expected = bithash_compute(&bits);
        let witness = bits_to_witness(&bits);
        let s = bithash_verify(expected);
        let result = execute_script_with_inputs(s, witness);
        assert!(result.success, "alternating failed: {:?}", result);
    }

    #[test]
    fn test_bithash_wrong_hash_fails() {
        let bits = [0u8; 128];
        let wrong_hash = [0xffu8; 20];
        let witness = bits_to_witness(&bits);
        let s = bithash_verify(wrong_hash);
        let result = execute_script_with_inputs(s, witness);
        assert!(!result.success, "wrong hash should fail");
    }

    #[test]
    fn test_bithash_save_to_altstack() {
        let bits = [1u8; 128];
        let expected = bithash_compute(&bits);
        let witness = bits_to_witness(&bits);
        let s = bithash_verify_save_to_altstack(expected);
        let result = execute_script_with_inputs(s, witness);
        assert!(result.success, "altstack variant failed: {:?}", result);
    }

    #[test]
    fn test_bithash_compute_deterministic() {
        let bits = [0u8; 128];
        assert_eq!(bithash_compute(&bits), bithash_compute(&bits));
        let bits2 = [1u8; 128];
        assert_ne!(bithash_compute(&bits), bithash_compute(&bits2));
    }
}
