//! Hash primitives and their representation-specific implementations.

pub mod bithash;
pub mod blake3;
pub mod sha256;

// Compatibility aliases for the original flat hash module.
pub use blake3::utils as blake3_utils;
pub use sha256::sha2_u4 as sha256_u4;
pub use sha256::sha2_u4_stack as sha256_u4_stack;
