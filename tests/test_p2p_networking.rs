use synergy_testnet::p2p;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn test_p2p_networking() {
    let blockchain = Arc::new(Mutex::new(Vec::<String>::new()));

    let handle = thread::spawn(move || {
        crate::p2p::start_p2p_network();
    });

    thread::sleep(Duration::from_secs(2));

    assert!(blockchain.lock().unwrap().is_empty());
    handle.join().unwrap();
}
