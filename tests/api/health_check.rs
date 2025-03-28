use crate::helpers::spawn_app;

#[tokio::test]
async fn test_health_check() {
    // Arrange
    let app = spawn_app().await;

    // Bring in `reqwest` to perform HTTP requests against the application
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
