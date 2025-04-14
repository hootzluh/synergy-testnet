use crate::block::{Block, BlockChain};
use crate::transaction::Transaction;
use std::fs;
use std::path::PathBuf;

/// Initializes and returns the blockchain with a genesis block.
pub fn initialize_blockchain() -> BlockChain {
    let mut chain = BlockChain::new();

    let genesis_block = Block::new(
        0,
        vec![],
        "0".to_string(),
        "genesis-validator".to_string(),
        0,
    );

    chain.add_block(genesis_block);
    chain
}

/// Simulates creating and adding a new block to the chain.
pub fn generate_new_block(
    chain: &mut BlockChain,
    transactions: Vec<Transaction>,
) -> Option<Block> {
    let previous_block = chain.latest_block()?;

    let new_block = Block::new(
        previous_block.index + 1,
        transactions,
        previous_block.hash.clone(),
        "validator-0001".to_string(),
        previous_block.nonce + 1,
    );

    if new_block.validate() {
        chain.add_block(new_block.clone());
        Some(new_block)
    } else {
        None
    }
}

/// Saves the blockchain state to a file (basic JSON serialization).
pub fn save_blockchain(chain: &BlockChain, path: &str) {
    let serialized = serde_json::to_string_pretty(&chain.chain).expect("Failed to serialize chain");

    fs::write(PathBuf::from(path), serialized).expect("Unable to write file");
}

/// Loads blockchain state from a file (basic JSON deserialization).
pub fn load_blockchain(path: &str) -> Option<BlockChain> {
    if let Ok(data) = fs::read_to_string(PathBuf::from(path)) {
        if let Ok(blocks) = serde_json::from_str::<Vec<Block>>(&data) {
            let mut chain = BlockChain::new();
            for block in blocks {
                chain.add_block(block);
            }
            return Some(chain);
        }
    }
    None
}
