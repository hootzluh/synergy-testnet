//! Synergy Network Consensus Module
//!
//! This module handles initialization and coordination of the
//! consensus mechanism used to secure the Synergy Testnet blockchain.

pub mod consensus_algorithm;

use crate::block::{Block, BlockChain};
use crate::transaction::Transaction;

use self::consensus_algorithm::ProofOfSynergy;

/// Starts the consensus mechanism using Proof of Synergy.
pub fn start_consensus(chain: &mut BlockChain, pending: Vec<Transaction>) {
    let mut engine = ProofOfSynergy::new();
    engine.initialize();
    let block = engine.mine_block(chain, pending);
    chain.add_block(block);
    println!("✅ New block mined and added to chain.");
}
