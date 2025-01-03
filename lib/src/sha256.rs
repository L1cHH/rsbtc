use crate::U256;
use sha256::digest;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};


#[derive(Serialize, Clone, Copy, PartialEq, Eq, Debug, Deserialize, Hash)]
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

        Hash(U256::from_little_endian(hash_slice))
    }

    pub fn matches_target(&self, target: U256) -> bool {
        self.0 <= target
    }

    pub fn zero() -> Self {
        Hash(U256::zero())
    }

    pub fn as_bytes(&self) -> [u8; 32] {
        let mut bytes: Vec<u8> = vec![0; 32];
        self.0.to_little_endian()
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
