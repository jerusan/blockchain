use std::cmp::Ordering;
use crate::{block::Block, util::{TARGET_BITS, get_sha256_hash}};

#[derive(Debug, Clone)]
pub struct ProofOfWork {
    block: Block,
    target: u128,
}

impl ProofOfWork {
    pub fn new(block: Block) -> ProofOfWork {
        ProofOfWork { block, target: TARGET_BITS }
    }

    pub fn validate(self) -> bool {
        let input: String = hex::encode(&self.block.prev_block_hash)
        + &self.block.data
        + &self.block.timestamp.to_string()
        + &self.block.nonce.to_string();
        let mut is_valid: bool = false;
        let sha256_string = get_sha256_hash(input);
        if hex::encode(sha256_string).cmp(&hex::encode(format!("{:x}", self.target))) == Ordering::Less {
            is_valid = true;
        }
        is_valid
    }
}
