use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
struct Block {
    index: u32,
    timestamp: u128,
    previous_hash: String,
    hash: String,
    data: String,
}

impl Block {
    fn new(index: u32, previous_hash: String, data: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let hash = Self::calculate_hash(index, timestamp, &previous_hash, &data);
        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            data,
        }
    }

    fn calculate_hash(index: u32, timestamp: u128, previous_hash: &str, data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp.to_string());
        hasher.update(previous_hash);
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        Blockchain {
            chain: vec![Block::new(0, String::from("0"), String::from("Genesis Block"))],
        }
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            previous_block.hash.clone(),
            data,
        );
        self.chain.push(new_block);
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block(String::from("First Block after Genesis"));
    blockchain.add_block(String::from("Second Block after Genesis"));

    for block in blockchain.chain {
        println!("{:?}", block);
    }
}
