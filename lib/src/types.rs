use uuid::Uuid;
use crate::U256;

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

pub struct BlockHeader {
    ///Timestamp of the block
    pub timestamp: u64,
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
        timestamp: u64,
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

pub struct Transaction {
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>
}

pub struct TransactionInput {
    pub prev_transaction_output_hash: [u8; 32],
    pub signature: [u8; 64]
}
pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: [u8; 33]
}

