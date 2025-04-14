use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::transaction::Transaction;
use sha3::{Digest, Sha3_256};
// Add Serde derives
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub validator_id: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        validator_id: String,
        nonce: u64,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut hasher = Sha3_256::new();
        hasher.update(index.to_be_bytes());
        hasher.update(timestamp.to_be_bytes());
        hasher.update(previous_hash.as_bytes());
        hasher.update(validator_id.as_bytes());
        hasher.update(nonce.to_be_bytes());

        for tx in &transactions {
            hasher.update(format!("{:?}", tx).as_bytes());
        }

        let hash = format!("{:x}", hasher.finalize());

        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
            validator_id,
            nonce,
        }
    }

    pub fn validate(&self) -> bool {
        // Basic validation: ensure block hash starts with '00'
        // Expand logic as needed for your real synergy chain
        self.hash.starts_with("00")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockChain {
    pub chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            chain: vec![],
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    pub fn latest_block(&self) -> Option<&Block> {
        self.chain.last()
    }
}
