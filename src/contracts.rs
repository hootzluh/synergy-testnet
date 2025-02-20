use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Contract {
    pub address: String,
    pub code: Vec<u8>, // WASM bytecode
}

#[derive(Debug)]
pub struct ContractExecutor {
    pub contracts: HashMap<String, Contract>,
}

impl ContractExecutor {
    pub fn new() -> Self {
        ContractExecutor {
            contracts: HashMap::new(),
        }
    }

    pub fn deploy_contract(&mut self, address: String, code: Vec<u8>) {
        let contract = Contract { address: address.clone(), code };
        self.contracts.insert(address, contract);
        println!("Smart Contract deployed successfully!");
    }

    pub fn execute_contract(&self, address: &str) {
    match self.contracts.get(address) {
        Some(contract) => {
            println!("Executing contract at address: {}", address);
            println!("Contract Code: {:?}", contract.code); // Use the contract variable
        }
        None => println!("Contract not found."),
    }
}
}
