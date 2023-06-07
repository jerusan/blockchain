use sha2::{Digest, Sha256};

pub fn get_sha256_hash(input: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();

    hex::encode(result)
}

pub const TARGET_BITS: u128 = 0b10000000000000000000000000000000000000000000000000000000000;