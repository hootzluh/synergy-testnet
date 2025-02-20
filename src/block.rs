use crate::transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub previous_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let hash = format!("{:x}", md5::compute(format!("{:?}{:?}{:?}", index, previous_hash, transactions)));

        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            transactions,
        }
    }
}
