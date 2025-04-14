use synergy_testnet::{rpc, consensus, p2p};
use tokio::task;

#[tokio::main]
async fn main() {
    println!("Synergy Testnet Node is starting...");

    // Start the RPC server (can remain in a thread if it's sync)
    task::spawn_blocking(|| {
        rpc::start_rpc_server();
    });

    // Start the consensus engine (also assumed sync)
    task::spawn_blocking(|| {
        consensus::run_consensus();
    });

    // Start the async P2P listener
    task::spawn(async {
        if let Err(e) = p2p::start_p2p_network("0.0.0.0:30303").await {
            eprintln!("P2P Error: {}", e);
        }
    });

    // Keep the node running
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
