use crate::U256;
use sha256::digest;
use serde::Serialize;
use std::fmt;
use std::fmt::{Display, Formatter, write};

#[derive(Serialize, Clone, Copy)]
pub struct Hash(U256);

impl Hash {
    pub fn hash<T: serde::Serialize>(data: &T) -> Self {
        let mut serialized: Vec<u8> = Vec::new();

        if let Err(e) = ciborium::into_writer(
            data,
            &mut serialized
        ) {
            panic!(
                "Failed to serialize data: {:?}. \
                This should not happen",
                e
            );
        }

        let hash = digest(&serialized);
        let hash_bytes = hex::decode(hash).unwrap();
        let hash_slice = hash_bytes.as_slice();

        Hash(U256::from_big_endian(hash_slice))
    }

    pub fn matches_target(&self, target: U256) -> bool {
        self.0 <= target
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
