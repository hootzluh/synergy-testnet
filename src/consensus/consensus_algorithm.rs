use std::thread;
use std::time::Duration;
use crate::block::{Block, BlockChain};
use crate::transaction::Transaction;
use crate::rpc::rpc_server::TX_POOL;

#[derive(Debug)]
pub struct ProofOfSynergy {
    pub chain: BlockChain,
}

impl ProofOfSynergy {
    pub fn new() -> Self {
        ProofOfSynergy {
            chain: BlockChain::new(),
        }
    }

    pub fn initialize(&mut self) {
        println!("🔧 Initializing Proof of Synergy...");
        self.chain.genesis();
    }

    pub fn execute(&mut self) {
        println!("⚙️ Executing consensus engine...");

        let chain = &mut self.chain;

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(5));

            let mut pool = TX_POOL.lock().unwrap();
            if pool.is_empty() {
                println!("⏳ No transactions to include in block.");
                continue;
            }

            let transactions = pool.clone();
            let latest_block = chain.latest_block().unwrap();
            let new_block = Block::new(
                latest_block.index + 1,
                latest_block.hash.clone(),
                transactions.clone(),
            );

            chain.add_block(new_block.clone());
            pool.clear();

            println!("🧱 New Block Mined!");
            println!("   Block Height: {}", new_block.index);
            println!("   Tx Count: {}", new_block.transactions.len());
        });
    }
}
