//! tests/health_check.rs

mod common;

#[actix_rt::test]
async fn health_check_works() {
    // Arrange test
    let address = common::spawn_app();

    // Create a reqwest client
    let client = reqwest::Client::new();

    // Do the actions
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    //Run assertions against the data
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
