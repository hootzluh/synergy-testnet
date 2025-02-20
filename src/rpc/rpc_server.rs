use std::net::TcpListener;
use std::io::{Read, Write};

pub fn start_rpc_server() {
    let listener = TcpListener::bind("127.0.0.1:8545").expect("Failed to bind RPC server");
    println!("RPC server running on 127.0.0.1:8545");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
                println!("Received RPC request: {:?}", String::from_utf8_lossy(&buffer));
                stream.write(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK").unwrap();
            },
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}
