use std::net::TcpListener;
use std::io::{Read, Write};
use synergy_testnet::transaction::Transaction; // adjust path if needed
use serde_json::Result as SerdeResult;

pub fn start_rpc_server() {
    let listener = TcpListener::bind("0.0.0.0:8545").expect("Failed to bind RPC server");
    println!("RPC server running on 0.0.0.0:8545");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 2048];
                let bytes_read = stream.read(&mut buffer).unwrap();
                let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);

                println!("\n--- Incoming RPC Request ---\n{}\n", request_str);

                if request_str.contains("POST /broadcast") {
                    if let Some(json_body) = extract_json(&request_str) {
                        match serde_json::from_str::<Transaction>(&json_body) {
                            Ok(tx) => {
                                println!("✅ Received transaction: {:?}", tx);
                                if tx.validate() {
                                    let response = format!(
                                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 26\r\n\r\nTransaction broadcasted."
                                    );
                                    stream.write_all(response.as_bytes()).unwrap();
                                } else {
                                    let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 20\r\n\r\nInvalid transaction";
                                    stream.write_all(response.as_bytes()).unwrap();
                                }
                            }
                            Err(e) => {
                                println!("❌ Failed to parse transaction: {:?}", e);
                                let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 24\r\n\r\nInvalid transaction data";
                                stream.write_all(response.as_bytes()).unwrap();
                            }
                        }
                    }
                } else {
                    let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\nNot Found";
                    stream.write_all(response.as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

// --- Helper to extract JSON from raw HTTP POST ---
fn extract_json(request: &str) -> Option<String> {
    let parts: Vec<&str> = request.split("\r\n\r\n").collect();
    if parts.len() > 1 {
        Some(parts[1].trim().to_string())
    } else {
        None
    }
}
