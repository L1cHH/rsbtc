mod sha256;
mod types;
mod util;
mod crypto;
mod error;

use serde::{Deserialize, Serialize};
use uint::construct_uint;

construct_uint! {
    #[derive(Serialize, Deserialize)]
    pub struct U256(4);
}

// initial reward in bitcoin - multiply by 10^8 to get satoshis
pub const INITIAL_REWARD: u64 = 50;

// halving interval in blocks
pub const HALVING_INTERVAL: u64 = 210;

pub const IDEAL_BLOCK_TIME: u64 = 10;

// minimum target
pub const MIN_TARGET: U256 = U256([
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0x0000_FFFF_FFFF_FFFF,
]);

pub const DIFFICULTY_UPDATE_INTERVAL: u64 = 50;