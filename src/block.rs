use chrono::Utc;
use std::{cmp::Ordering};

use crate::util::{TARGET_BITS, get_sha256_hash};

#[derive(Debug, Clone)]
pub struct Block {
    pub timestamp: i64,
    pub data: String,
    pub prev_block_hash: String,
    pub hash: String,
    pub nonce: u128,
}

impl Block {
    pub fn new(data: String, prev_block_hash: String) -> Block {
        let timestamp = Utc::now().timestamp();

        let input: String = hex::encode(&prev_block_hash) + &data + &timestamp.to_string();
        let hash_string = get_sha256_hash(input);

        let mut new_block = Block {
            timestamp,
            data: data,
            prev_block_hash,
            hash: hash_string,
            nonce: 1,
        };
        Self::run_pow(&mut new_block);
        return new_block;
    }

    pub fn new_genesis_block() -> Block {
        let data = "Genesis Block".to_string();
        let prev_block_hash = "".to_string();
        Self::new(data, prev_block_hash)
    }

    fn run_pow(block: &mut Block) {
        let mut nonce: u128 = 0;
  
        println!(
            "Mining the block containing {:?}",
            &block.data
        );

        while nonce < 5000 {
            let input: String = hex::encode(&block.prev_block_hash)
                + &block.data
                + &block.timestamp.to_string()
                // + &hex::encode(&block.hash)
                // + &TARGET_BITS.to_string()
                + &nonce.to_string();

            let hash_string = get_sha256_hash(input);
           
            let target_string = format!("{:x}", TARGET_BITS);
            
            if (&hash_string).cmp(&target_string) == Ordering::Less {
                println!("Nonce: {} Found Hash: {:?}", nonce, hash_string);

                block.nonce = nonce;
                block.hash = hash_string;
                println!();
                break;
            } else {
                nonce += 1;
            }
        }
    }
}
