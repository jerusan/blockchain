mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut bc = Blockchain::new_blockchain();

    bc.add_block("Send 1 ETH to Bob".to_string());
    bc.add_block("Send 1 BTC to Alice".to_string());

    for block in &bc.blocks {
        let hex_prefix = if !block.prev_block_hash.is_empty() { "0x"} else {""};
        println!("Previous hash: {}{}", hex_prefix, hex::encode(&block.prev_block_hash));
        println!("Data: {:?}", String::from_utf8_lossy(&block.data).to_string());
        println!("Hash: 0x{}", hex::encode(&block.hash));
        println!("Timestamp: {:?}\n", block.timestamp);
    }
}
