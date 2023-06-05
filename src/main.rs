mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut bc = Blockchain::new_blockchain();

    bc.add_block("Send 1 ETH to Bob".to_string());
    bc.add_block("Send 1 BTC to Alice".to_string());

    for block in &bc.blocks {
        println!("Prev. hash: {:?}", block.prev_block_hash);
        println!("Data: {:?}", block.data);
        println!("Hash: {:?}\n", block.hash);
        println!("Timestamp: {:?}\n", block.timestamp);
    }
}
