#![allow(unused)]

use std::net::TcpListener;
pub fn spawn_app() -> String {
    // Create a TcpListener so we can get info off the port and return it
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    // Create the server, bind port 0 to allow the OS to choose an unused one at random.
    let server = norseline::run(listener).expect("Failed to bind address.");
    // Run it in a spawn
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

pub const TEST_PUBLIC_KEY: &str = "172676b110ea0e14d6f41c50cf6b82dbf789cfeb5057eafc4ed58b2d49d98c75edd6964ba8ce8ab6d945581056b553a8f3dcef978a2bcfa8879ea7747384c10f";
pub const TEST_TIMESTAMP: &str = "123";
pub const TEST_MESSAGE: &str = "Test message";
