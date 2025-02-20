use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn run_consensus(blockchain: Arc<Mutex<Vec<String>>>) {
    println!("Running Proof of Synergy consensus...");

    loop {
        thread::sleep(Duration::from_secs(10));
        println!("Consensus validating new blocks...");
    }
}
