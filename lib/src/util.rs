use serde::{Deserialize, Serialize};
use crate::sha256::Hash;
use crate::types::Transaction;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct MerkleRoot(Hash);

impl MerkleRoot {
    pub fn calculate(transactions: &[Transaction]) -> Self {
        let mut layer: Vec<Hash> = Vec::new();

        for tx in transactions {
            layer.push(Hash::hash(&tx))
        };

        while layer.len() > 1 {
            let mut new_layer: Vec<Hash> = Vec::new();

            for pair_hash_of_tx in layer.chunks(2) {
                let left_tx_hash = pair_hash_of_tx[0];
                ///if there is no right, use the left hash again
                let right_tx_hash = pair_hash_of_tx.get(1).unwrap_or(&pair_hash_of_tx[0]);

                new_layer.push(Hash::hash(&[left_tx_hash, *right_tx_hash]))
            }

            layer = new_layer;
        }
        MerkleRoot(layer[0])
    }
}

