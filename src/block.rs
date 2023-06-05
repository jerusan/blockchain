use chrono::Utc;
use sha2::{Digest, Sha256};

pub struct Block {
    pub timestamp: i64,
    pub data: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
}

impl Block {
    pub fn new_block(data: String, prev_block_hash: Vec<u8>) -> Block {
        let timestamp = Utc::now().timestamp();
        let timestamp_bytes = timestamp.to_be_bytes();
        let data_byes = data.as_bytes();
        let headers_len = prev_block_hash.len() + data_byes.len() + timestamp_bytes.len();
        let mut headers = Vec::with_capacity(headers_len);
        headers.extend_from_slice(&prev_block_hash);
        headers.extend_from_slice(&data_byes);
        headers.extend_from_slice(&timestamp_bytes);
    
        let hash = Sha256::digest(&headers);
    
        Block {
            timestamp,
            data: data.into_bytes(),
            prev_block_hash,
            hash: hash.to_vec(),
        }
    }

    pub fn new_genesis_block() -> Block {
        let data = "Genesis Block".to_string();
        let prev_block_hash = Vec::new();
        Self::new_block(data, prev_block_hash)
    }
}

