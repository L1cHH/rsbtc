use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::U256;
use crate::crypto::{Signature, PublicKey};
use crate::sha256::Hash;
use crate::util::MerkleRoot;
use crate::error::{BtcError, Result};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    pub utxos: HashMap<Hash, TransactionOutput>,
    pub blocks: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            utxos: HashMap::new(),
            blocks: Vec::new()
        }
    }

    pub fn add_block(&mut self, block: Block) -> Result<()> {

        if self.blocks.is_empty() {

            ///Check if the previous block hash eq to zero
            if block.header.prev_block_hash != Hash::zero() {
                println!("zero hash");
                return Err(BtcError::InvalidBlock)
            }

        } else {

            let last_block = self.blocks.last().unwrap();

            ///Check if the prev block hash is equal to curr_block.header.prev_block_hash
            if block.header.prev_block_hash != last_block.hash() {
                println!("prev hash is wrong");
                return Err(BtcError::InvalidBlock)
            }

            ///Check if the block's hash does not match the target (need to be hash<target)
            if !block
                .header
                .hash()
                .matches_target(block.header.target) {

                println!("does not match target");
                return Err(BtcError::InvalidBlock)
            }

            ///Check if the Merkle root hash is correct
            let calculated_merkle_root_hash = MerkleRoot::calculate(&block.transactions);
            if calculated_merkle_root_hash != block.header.merkle_root {
                println!("invalid merkle root hash");
                return Err(BtcError::InvalidMerkleRoot)
            }

            ///Check if the block's timestamp is after last block's timestamp
            if block.header.timestamp <= last_block.header.timestamp {
                return Err(BtcError::InvalidBlock)
            }

            /// Verify all transactions in the block


        }

        self.blocks.push(block);
        Ok(())
    }

    pub fn rebuild_utxos(&mut self) {
        for block in &self.blocks {
            for transaction in &block.transactions {
                for input in &transaction.inputs {
                    self.utxos.remove(&input.prev_transaction_output_hash);
                }

                for output in &transaction.outputs {
                    self.utxos.insert(
                        transaction.hash(),
                        output.clone()
                    );
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>
}

impl Block {
    pub fn new(
        header: BlockHeader,
        transactions: Vec<Transaction>
    ) -> Self {
        Block {
            header,
            transactions
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {
    ///Timestamp of the block
    pub timestamp: DateTime<Utc>,
    ///Nonce used to mine the block
    pub nonce: u64,
    ///Hash of the previous block
    pub prev_block_hash: Hash,
    ///Merkle root of the block's transaction
    pub merkle_root: MerkleRoot,
    ///target (number that need to be higher than hash)
    pub target: U256
}

impl BlockHeader {
    pub fn new(
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: Hash,
        merkle_root: MerkleRoot,
        target: U256
    ) -> Self {
        BlockHeader {
            timestamp,
            nonce,
            prev_block_hash,
            merkle_root,
            target
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>
}

impl Transaction {
    pub fn new(
        inputs: Vec<TransactionInput>,
        outputs: Vec<TransactionOutput>
    ) -> Transaction {
        Transaction {
            inputs,
            outputs
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
///What exact output should be spent
pub struct TransactionInput {
    ///Points to exact UTXO (unspent output) {Tx hash and index of exact output}
    pub prev_transaction_output_hash: Hash,
    ///Signature is used for verifying accessory to specific output (ability to spend)
    pub signature: Signature
}

#[derive(Serialize, Deserialize, Clone, Debug)]
///How much and for whom exact output should be spent
pub struct TransactionOutput {
    ///How much currency
    pub value: u64,
    ///ID of specific output (index)
    pub unique_id: Uuid,
    ///Pubkey of recipient
    pub pubkey: PublicKey
}

impl TransactionOutput {
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

