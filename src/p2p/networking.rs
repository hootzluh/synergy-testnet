use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_p2p_network(blockchain: Arc<Mutex<Vec<String>>>) {
    println!("Initializing P2P network...");

    loop {
        thread::sleep(Duration::from_secs(5));
        println!("P2P network running, syncing blocks...");
    }
}
