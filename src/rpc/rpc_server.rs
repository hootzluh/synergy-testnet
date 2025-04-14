use std::net::TcpListener;
use std::io::{Read, Write};
use crate::transaction::Transaction;
use serde_json::Value;

pub fn start_rpc_server() {
    let listener = TcpListener::bind("0.0.0.0:8545").expect("Failed to bind RPC server");
    println!("RPC server running on 0.0.0.0:8545");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 2048];
                match stream.read(&mut buffer) {
                    Ok(_) => {
                        let request_str = String::from_utf8_lossy(&buffer);
                        println!("📩 Received RPC request:\n{}", request_str);

                        if request_str.contains("POST") {
                            if let Some(json_start) = request_str.find("{") {
                                let json_str = &request_str[json_start..];
                                if let Ok(json): Result<Value, _> = serde_json::from_str(json_str) {
                                    if let Some(tx_hex) = json.get("tx") {
                                        if let Some(tx_str) = tx_hex.as_str() {
                                            match Transaction::from_json(tx_str) {
                                                Ok(tx) => {
                                                    println!("✅ Parsed Transaction: {:?}", tx);
                                                    // You could store the tx here if needed
                                                }
                                                Err(e) => {
                                                    eprintln!("❌ Failed to decode transaction: {}", e);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        let response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK";
                        let _ = stream.write(response.as_bytes());
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to read from stream: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Incoming connection failed: {}", e);
            }
        }
    }
}
