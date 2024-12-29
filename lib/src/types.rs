use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::U256;
use crate::crypto::{Signature, PublicKey};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: Vec::new()
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block)
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

    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {
    ///Timestamp of the block
    pub timestamp: DateTime<Utc>,
    ///Nonce used to mine the block
    pub nonce: u64,
    ///Hash of the previous block
    pub prev_block_hash: [u8; 32],
    ///Merkle root of the block's transaction
    pub merkle_root: [u8; 32],
    ///target (number that need to be higher than hash)
    pub target: U256
}

impl BlockHeader {
    pub fn new(
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: [u8; 32],
        merkle_root: [u8; 32],
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

    pub fn hash(&self) -> ! {
        unimplemented!()
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

    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
///What exact output should be spent
pub struct TransactionInput {
    ///Points to exact UTXO (unspent output) {Tx hash and index of exact output}
    pub prev_transaction_output_hash: [u8; 32],
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

