mod block;
mod blockchain;
mod proof_of_work;
mod util;

use blockchain::Blockchain;
use proof_of_work::ProofOfWork;

fn main() {
    let mut bc = Blockchain::new();

    bc.add_block("Send 1 ETH to Alice".to_string());
    bc.add_block("Send 1 ETH to Bob".to_string());

    for block in bc.blocks {
        let hex_prefix = if !block.prev_block_hash.is_empty() { "0x"} else {""};
        println!("Previous hash: {}{}", hex_prefix, hex::encode(&block.prev_block_hash));
        println!("Data: {:?}", &block.data);
        println!("Hash: 0x{}", hex::encode(&block.hash));
        println!("Timestamp: {:?}\n", block.timestamp);

        let pow = ProofOfWork::new(block.clone()); 
        println!("Validation for block data: {:?}: {:?}\n", &block.data, pow.validate());

    }
}
