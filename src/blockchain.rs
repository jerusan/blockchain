use super::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn add_block(&mut self, data: String) {
        let prev_block = self.blocks.last().unwrap();
        let new_block = Block::new(data, prev_block.hash.clone());
        self.blocks.push(new_block);
    }

    pub fn new() -> Blockchain {
        let genesis_block = Block::new_genesis_block();
        Blockchain {
            blocks: vec![genesis_block],
        }
    }
}
