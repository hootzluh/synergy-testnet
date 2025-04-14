use serde::{Deserialize, Serialize};
use crate::transaction::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub validator_id: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        validator_id: String,
        nonce: u64,
    ) -> Self {
        let data = format!("{:?}{:?}{}{}{}", index, transactions, previous_hash, validator_id, nonce);
        let hash = blake3::hash(data.as_bytes()).to_hex().to_string();
        Block {
            index,
            transactions,
            previous_hash,
            validator_id,
            nonce,
            hash,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockChain {
    pub chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain { chain: vec![] }
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    pub fn last(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn genesis(&mut self) {
        let genesis_block = Block::new(
            0,
            vec![],
            "0".to_string(),
            "genesis".to_string(),
            0,
        );
        self.chain.push(genesis_block);
    }
}
