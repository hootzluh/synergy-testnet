use chrono::Utc;
use sha3::{Digest, Sha3_256};
use crate::transaction::Transaction;
use std::fmt::{self, Debug};
use std::sync::{Arc, Mutex};
use crate::rpc::rpc_server::TX_POOL;

#[derive(Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let block_string = format!("{index}{timestamp:?}{:?}{previous_hash}", transactions);
        let mut hasher = Sha3_256::new();
        hasher.update(block_string);
        let result = hasher.finalize();
        let hash = hex::encode(result);

        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Block #{}\n  Timestamp: {}\n  Previous Hash: {}\n  Hash: {}\n  Transactions: {:?}",
            self.index, self.timestamp, self.previous_hash, self.hash, self.transactions)
    }
}

/// Mines a new block from the TX_POOL.
pub fn mine_block() -> Block {
    let mut pool = TX_POOL.lock().unwrap();
    let transactions = pool.drain(..).collect::<Vec<Transaction>>();

    let previous_hash = "0".repeat(64); // Default hash for simplicity
    let block = Block::new(1, transactions, previous_hash);
    println!("⛏️  Mined new block: {:?}", block);
    block
}
