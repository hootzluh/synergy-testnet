use crate::block::{Block, BlockChain};
use crate::transaction::Transaction;

pub struct ProofOfSynergy;

impl ProofOfSynergy {
    pub fn new() -> Self {
        ProofOfSynergy
    }

    pub fn initialize(&mut self) {
        println!("🔧 Initializing Proof of Synergy...");
    }

    pub fn execute(&mut self) {
        println!("⚙️ Executing consensus engine...");
    }

    pub fn mine_block(&mut self, chain: &BlockChain, transactions: Vec<Transaction>) -> Block {
        let prev = chain.latest_block().expect("No previous block exists.");

        let new_block = Block::new(
            prev.index + 1,
            transactions,
            prev.hash.clone(),
            "validator-0001".to_string(),
            prev.nonce + 1,
        );

        if new_block.validate() {
            println!("✅ Valid block mined: {}", new_block.hash);
            new_block
        } else {
            panic!("❌ Invalid block.");
        }
    }
}
