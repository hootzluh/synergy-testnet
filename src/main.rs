use synergy_testnet::{rpc, consensus, p2p};
use std::sync::Arc;
use tokio::task;

#[tokio::main]
async fn main() {
    println!("✅ Synergy Testnet Node is starting...");

    // NOTE: Replace with actual config loading when the config module is wired up correctly.
    let p2p_address = "0.0.0.0:30303";

    // Start the RPC server (no arguments required)
    task::spawn_blocking(|| {
        rpc::start_rpc_server(); // Now called with no args
    });

    // Start the consensus engine (no arguments required)
    task::spawn_blocking(|| {
        consensus::run_consensus(); // Now called with no args
    });

    // Start the async P2P listener
    task::spawn(async move {
        if let Err(e) = p2p::start_p2p_network(p2p_address).await {
            eprintln!("P2P Error: {}", e);
        }
    });

    // Keep the node running
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
