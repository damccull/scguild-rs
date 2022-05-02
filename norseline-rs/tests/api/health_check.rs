use crate::helpers::TestApp;

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let test_app = TestApp::spawn().await;

    // Act
    let response = reqwest::Client::new()
        .get(format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
