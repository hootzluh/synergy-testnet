use crate::block::Block;
use crate::transaction::Transaction;
use crate::contracts::ContractExecutor;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: VecDeque<Transaction>,
    pub contract_executor: ContractExecutor,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "0".to_string(), vec![]);
        Blockchain {
            chain: vec![genesis_block],
            pending_transactions: VecDeque::new(),
            contract_executor: ContractExecutor::new(),
        }
    }

    /// Adds a new transaction to the pending transaction queue.
    pub fn add_transaction(&mut self, sender: String, receiver: String, amount: u64) {
        let transaction = Transaction::new(sender, receiver, amount);
        self.pending_transactions.push_back(transaction);
        println!("Transaction added: {:?}", self.pending_transactions.back().unwrap());
    }

    /// Mines a new block with the pending transactions.
    pub fn mine_block(&mut self) {
    if self.pending_transactions.is_empty() {
        println!("No pending transactions to mine.");
        return;
    }

    let previous_block = self.chain.last().unwrap().clone(); // Clone the last block

    let transactions: Vec<Transaction> = self.pending_transactions.drain(..).collect();
    let new_block = Block::new(
        previous_block.index + 1,
        previous_block.hash.clone(),
        transactions,
    );

    self.chain.push(new_block);
    println!("Block {} mined successfully!", previous_block.index + 1);
}

    /// Deploys a smart contract onto the blockchain.
    pub fn deploy_smart_contract(&mut self, address: String, code: Vec<u8>) {
        self.contract_executor.deploy_contract(address, code);
    }

    /// Executes a smart contract.
    pub fn execute_smart_contract(&self, address: &str) {
        self.contract_executor.execute_contract(address);
    }
}

/// Starts the Synergy Testnet Node.
pub fn start_node() {
    println!("Synergy node is starting...");

    let mut blockchain = Blockchain::new();
    println!("Genesis Block: {:?}", blockchain.chain[0]);

    blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 100);
    blockchain.mine_block();

    let contract_address = "0xABC123".to_string();
    let contract_code = vec![0x00, 0x61, 0x73, 0x6D]; // Sample WASM header
    blockchain.deploy_smart_contract(contract_address.clone(), contract_code);
    blockchain.execute_smart_contract(&contract_address);

    println!("Synergy Testnet Node Initialized!");
}
