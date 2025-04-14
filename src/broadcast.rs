use crate::transaction::Transaction;
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

/// Broadcast a transaction to a remote node using an HTTP POST request.
/// Attempts retry logic on failure.
pub async fn broadcast_transaction(transaction: &Transaction, target_ip: &str, port: u16) -> Result<String, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| format!("Client init error: {}", e))?;

    let url = format!("http://{}:{}/broadcast", target_ip, port);

    let mut attempts = 0;
    let max_attempts = 3;

    while attempts < max_attempts {
        match client.post(&url).json(&transaction).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    let body = resp.text().await.unwrap_or_else(|_| "<no body>".to_string());
                    return Ok(format!("✅ Broadcast success: {}", body));
                } else {
                    return Err(format!("❌ Broadcast failed with HTTP {}", resp.status()));
                }
            }
            Err(e) => {
                attempts += 1;
                println!("⚠️ Broadcast attempt {}/{} failed: {}", attempts, max_attempts, e);
                sleep(Duration::from_millis(500)).await;
            }
        }
    }

    Err("❌ Max retries reached. Transaction broadcast failed.".to_string())
}
