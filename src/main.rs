use std::thread;
use synergy_testnet::{rpc, consensus, p2p};

fn main() {
    println!("Synergy Testnet Node is starting...");

    // Start the RPC server in a separate thread
    thread::spawn(|| {
        rpc::start_rpc_server();
    });

    // Start other components
    thread::spawn(|| {
        consensus::run_consensus();
    });

    thread::spawn(|| {
        p2p::start_p2p_network();
    });

    // Keep the main thread alive
    loop {}
}
