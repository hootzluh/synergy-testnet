use std::net::TcpListener;
use std::io::{Read, Write};
use crate::transaction::Transaction;
use crate::config::NodeConfig;

/// Start a minimal JSON-over-HTTP RPC server for broadcasting transactions.
pub fn start_rpc_server(config: &NodeConfig) {
    let addr = config.rpc.listen_address.as_str();
    let listener = TcpListener::bind(addr).expect("Failed to bind RPC server");
    println!("🌐 RPC server running on {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 2048];
                if let Ok(bytes_read) = stream.read(&mut buffer) {
                    let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);

                    println!("🔁 Incoming RPC request:\n{}", request_str);

                    if request_str.contains("POST /broadcast") {
                        if let Some(json_body) = extract_json(&request_str) {
                            match serde_json::from_str::<Transaction>(&json_body) {
                                Ok(tx) if tx.validate() => {
                                    println!("✅ Valid transaction: {:?}", tx);
                                    let response = http_response(200, r#"{"status":"broadcasted"}"#);
                                    let _ = stream.write_all(response.as_bytes());
                                }
                                Ok(_) => {
                                    let response = http_response(400, r#"{"error":"invalid transaction"}"#);
                                    let _ = stream.write_all(response.as_bytes());
                                }
                                Err(_) => {
                                    let response = http_response(400, r#"{"error":"malformed json"}"#);
                                    let _ = stream.write_all(response.as_bytes());
                                }
                            }
                        }
                    } else {
                        let response = http_response(404, r#"{"error":"not found"}"#);
                        let _ = stream.write_all(response.as_bytes());
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ RPC connection error: {}", e);
            }
        }
    }
}

/// Extract JSON body from raw HTTP POST string
fn extract_json(request: &str) -> Option<String> {
    request.split("\r\n\r\n").nth(1).map(str::trim).map(str::to_string)
}

/// Helper to build HTTP/1.1 responses
fn http_response(status: u16, body: &str) -> String {
    format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        status,
        http_status_msg(status),
        body.len(),
        body
    )
}

/// Maps status codes to message strings
fn http_status_msg(code: u16) -> &'static str {
    match code {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        _ => "Unknown",
    }
}
