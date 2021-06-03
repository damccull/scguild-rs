//! tests/discord_ed25519_signatures.rs

mod common;

#[actix_rt::test]
async fn api_sends_200_with_valid_data() {
    // Arrange test
    let address = common::spawn_app();

    // Create a reqwest client
    let client = reqwest::Client::new();

    // Do the actions
    let response = client
        .get(&format!("{}/api/discord/testing", address))
        .header("X-Signature-Ed25519", common::TEST_PUBLIC_KEY)
        .header("X-Signature-Timestamp", common::TEST_TIMESTAMP)
        .body(common::TEST_MESSAGE)
        .send()
        .await
        .expect("Failed to execute request.");

    //Run assertions against the data
    assert!(response.status().is_success());
}

#[actix_rt::test]
async fn api_sends_401_when_missing_ed25519_header() {
    // Arrange test
    let address = common::spawn_app();

    // Create a reqwest client
    let client = reqwest::Client::new();

    // Do the actions
    let response = client
        .get(&format!("{}/api/discord/testing", address))
        .header("X-Signature-Timestamp", common::TEST_TIMESTAMP)
        .body(common::TEST_MESSAGE)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 401)
}

#[actix_rt::test]
async fn api_sends_401_when_missing_timestamp_header() {
    // Arrange test
    let address = common::spawn_app();

    // Create a reqwest client
    let client = reqwest::Client::new();

    // Do the actions
    let response = client
        .get(&format!("{}/api/discord/testing", address))
        .header("X-Signature-Ed25519", common::TEST_PUBLIC_KEY)
        .body(common::TEST_MESSAGE)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 401)
}

#[actix_rt::test]
async fn api_sends_401_when_invalid_signature() {
    // Arrange test
    let address = common::spawn_app();

    // Create a reqwest client
    let client = reqwest::Client::new();

    // Do the actions
    let response = client
        .get(&format!("{}/api/discord/testing", address))
        .header("X-Signature-Ed25519", common::TEST_PUBLIC_KEY)
        .header("X-Signature-Timestamp", common::TEST_TIMESTAMP)
        .body("bad message")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 401)
}

#[actix_rt::test]
async fn api_sends_401_when_bad_hex() {
    // Arrange test
    let address = common::spawn_app();

    // Create a reqwest client
    let client = reqwest::Client::new();

    // Do the actions
    let response = client
        .get(&format!("{}/api/discord/testing", address))
        // Replacing all `f`s with `z`s will break the hex decoder
        .header("X-Signature-Ed25519", common::TEST_PUBLIC_KEY.replace("f", "z"))
        .header("X-Signature-Timestamp", common::TEST_TIMESTAMP)
        .body(common::TEST_MESSAGE)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 401)
}

#[actix_rt::test]
async fn api_sends_401_when_wrong_signature_length() {
    // Arrange test
    let address = common::spawn_app();

    // Create a reqwest client
    let client = reqwest::Client::new();

    // Do the actions
    let response = client
        .get(&format!("{}/api/discord/testing", address))
        // Replacing all `f`s with nothing will break the string length for the key
        .header("X-Signature-Ed25519", common::TEST_PUBLIC_KEY.replace("f", ""))
        .header("X-Signature-Timestamp", common::TEST_TIMESTAMP)
        .body(common::TEST_MESSAGE)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 401)
}
