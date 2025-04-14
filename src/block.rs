use crate::transaction::Transaction;
use blake3::Hasher;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub previous_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
    pub validator: String,
    pub nonce: u64,
}

impl Block {
    /// Create a new block and compute its secure hash
    pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>, validator: String, nonce: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let hash = Self::calculate_hash(index, timestamp, &previous_hash, &transactions, &validator, nonce);

        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            transactions,
            validator,
            nonce,
        }
    }

    /// Calculate a BLAKE3 hash of the block's contents
    pub fn calculate_hash(
        index: u64,
        timestamp: u64,
        previous_hash: &str,
        transactions: &[Transaction],
        validator: &str,
        nonce: u64,
    ) -> String {
        let mut hasher = Hasher::new();
        hasher.update(index.to_le_bytes().as_ref());
        hasher.update(timestamp.to_le_bytes().as_ref());
        hasher.update(previous_hash.as_bytes());
        hasher.update(validator.as_bytes());
        hasher.update(nonce.to_le_bytes().as_ref());

        for tx in transactions {
            hasher.update(serde_json::to_string(tx).unwrap().as_bytes());
        }

        hasher.finalize().to_hex().to_string()
    }

    /// Verify block integrity (basic check)
    pub fn validate(&self) -> bool {
        self.hash == Self::calculate_hash(
            self.index,
            self.timestamp,
            &self.previous_hash,
            &self.transactions,
            &self.validator,
            self.nonce,
        )
    }
}
