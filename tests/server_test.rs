use roverfy::{BaseSettings, Roverfy};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

#[test]
fn test_server_starts_and_accepts_connection() {
    // Step 1: Initialize settings for the test environment.
    // This uses the default configuration which points to the project root.
    let settings = BaseSettings::default();
    let app = Roverfy::new(vec!["test".to_string(), "serve".to_string()], settings);

    // Step 2: Run the server in a separate thread so it doesn't block the test.
    let _server_handle = thread::spawn(move || {
        if let Err(e) = app.run() {
            panic!("Server failed to run: {}", e);
        }
    });

    // Give the server a moment to start up.
    thread::sleep(Duration::from_millis(500));

    // Step 3: Attempt to connect to the server.
    // The server address is hardcoded for this test, assuming default settings.
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(_) => {
            // Connection successful, the server is up and running.
            println!("Successfully connected to the server.");
        }
        Err(e) => {
            // The test fails if we cannot connect.
            panic!("Failed to connect to server: {}", e);
        }
    }

    // Note: We don't have a clean server shutdown mechanism yet, so the test
    // will complete and the server thread will be forcefully terminated.
    // This is acceptable for a simple "does it start?" test.
    // To properly stop the server, `server_handle.join()` would be needed
    // after implementing a server stop signal.
} 