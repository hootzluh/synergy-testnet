use crate::transaction::Transaction;
use reqwest::blocking::Client;

pub fn broadcast_transaction(transaction: &Transaction, target_ip: &str, port: u16) -> Result<String, String> {
    let client = Client::new();
    let url = format!("http://{}:{}/broadcast", target_ip, port);

    match client.post(&url).json(&transaction).send() {
        Ok(resp) => match resp.text() {
            Ok(body) => Ok(format!("Broadcast response: {}", body)),
            Err(e) => Err(format!("Failed to read response body: {}", e)),
        },
        Err(e) => Err(format!("Failed to send transaction: {}", e)),
    }
}
