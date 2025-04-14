use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::block::Block;

/// Starts the Proof of Synergy (PoSy) consensus engine.
/// This simulates block proposal, validation, and addition to the chain.
pub fn run_consensus(blockchain: Arc<Mutex<Vec<Block>>>) {
    println!("🌀 Starting Proof of Synergy (PoSy) consensus engine...");

    thread::spawn(move || {
        loop {
            {
                let mut chain = blockchain.lock().unwrap();
                let last_block = chain.last().cloned();

                // Simulate proposing a new block
                if let Some(prev) = last_block {
                    let new_block = Block::new(
                        prev.index + 1,
                        prev.hash.clone(),
                        vec![], // normally includes pending transactions
                        "validator-0001".to_string(),
                        prev.nonce + 1,
                    );

                    if new_block.validate() {
                        println!("✅ New block validated and added to chain: #{}", new_block.index);
                        chain.push(new_block);
                    } else {
                        eprintln!("❌ Invalid block rejected.");
                    }
                }
            }

            thread::sleep(Duration::from_secs(10));
        }
    });
}
