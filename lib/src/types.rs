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


pub struct Block;
pub struct BlockHeader;
pub struct Transaction;

