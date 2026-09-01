pub mod hors;
pub mod lamport;
pub mod winternitz;

pub use winternitz::signing as signing_winternitz;
pub use winternitz::utils;
pub use winternitz::{
    CompactWots, GenericWinternitzPublicKey, WinternitzSecret, WinternitzSigningInputs, Wots,
    Wots16, Wots32, Wots4, Wots64, Wots80, LOG2_BASE,
};

pub const HASH_LEN: usize = 16;
