//! tests/api.rs

#[actix_rt::test]
async fn api_sends_404_with_missing_endpoint() {
    // Arrange test
    let address = common::spawn_app();
    // Create a reqwest client
    let client = reqwest::Client::new();

    // Do the actions
    let response = client
        .get(&format!("{}/api/missingendpoint", address.as_str()))
        .send()
        .await
        .expect("Failed to execute request.");

    //Run assertions against the data
    assert_eq!(response.status().as_u16(), 404);
}

#[actix_rt::test]
async fn api_sends_200_with_correct_endpoint() {
    // Arrange test
    let address = common::spawn_app();
    // Create a reqwest client
    let client = reqwest::Client::new();

    // Do the actions
    let response = client
        .get(&format!("{}/api/v1/testing", address.as_str()))
        .send()
        .await
        .expect("Failed to execute request.");

    //Run assertions against the data
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(
        response.text().await.unwrap(),
        "API requested path: testing"
    );
}
