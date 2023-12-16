use std::net::TcpListener;
use zero2prod::run;

#[tokio::test]
async fn test_health_check() {
    // Arrange
    let address = spawn_app();

    // Bring in `reqwest` to perform HTTP requests against the application
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch the application in tha background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");

    // Retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("Failed to bind address.");

    // `tokio::spawn` runs the server concurrently so it won't block the rest of the test
    tokio::spawn(server);

    // return the address
    format!("http://127.0.0.1:{}", port)
}
