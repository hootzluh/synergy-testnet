use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod p2p;
mod consensus;
mod rpc;

fn main() {
    println!("Starting Synergy Network Node...");

    let blockchain = Arc::new(Mutex::new(Vec::new()));

    let consensus_handle = {
        let blockchain = Arc::clone(&blockchain);
        thread::spawn(move || {
            consensus::run_consensus(blockchain);
        })
    };

    let p2p_handle = {
        let blockchain = Arc::clone(&blockchain);
        thread::spawn(move || {
            p2p::start_p2p_network(blockchain);
        })
    };

    let rpc_handle = thread::spawn(move || {
        rpc::start_rpc_server();
    });

    consensus_handle.join().unwrap();
    p2p_handle.join().unwrap();
    rpc_handle.join().unwrap();
}
