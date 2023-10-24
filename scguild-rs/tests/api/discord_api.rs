use crate::helpers::TestApp;
use actix_http::StatusCode;
use chrono::Utc;
use ed25519_dalek::Signer;
use serde::Deserialize;
use twilight_model::{
    application::interaction::{self, InteractionType},
    id::{
        marker::{ApplicationMarker, InteractionMarker},
        Id,
    },
};

#[actix_rt::test]
async fn discord_api_responds_with_pong_when_given_ping() {
    // Arrange
    let test_app = TestApp::spawn().await;
    let keypair = test_app.discord_keypair;

    let body = interaction::Ping {
        application_id: Id::<ApplicationMarker>::new(12351532),
        id: Id::<InteractionMarker>::new(684641),
        kind: InteractionType::Ping,
        token: "blart".to_string(),
    };

    let x = serde_json::to_string(&body).unwrap();

    let timestamp = Utc::now().to_string();
    let signme = format!("{}{}", timestamp, x);
    let signature = keypair.sign(signme.as_bytes());

    // Act
    let response = reqwest::Client::new()
        .post(format!("{}/discord", &test_app.address))
        .header("X-Signature-Ed25519", signature.to_string())
        .header("X-Signature-Timestamp", timestamp)
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    let status = response.status();

    #[derive(Debug, Deserialize)]
    struct Pong {
        #[serde(rename = "type")]
        pub interaction_type: u8,
    }

    let json = response.json::<Pong>().await;

    // Assert
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json.unwrap().interaction_type, 1_u8);
}

// #[actix_rt::test]
// async fn discord_api_responds_with_debug_when_debug_interaction_sent() {
//     // Arrange
//     let test_app = TestApp::spawn().await;
//     let keypair = test_app.discord_keypair;

//     let body = interaction::ApplicationCommand{

//     }

//     let x = serde_json::to_string(&body).unwrap();

//     let timestamp = Utc::now().to_string();
//     let signme = format!("{}{}", timestamp.to_string(), x);
//     let signature = keypair.sign(signme.as_bytes());

//     // Act
//     let response = reqwest::Client::new()
//         .post(format!("{}/api/discord", &test_app.address))
//         .header("X-Signature-Ed25519", signature.to_string())
//         .header("X-Signature-Timestamp", timestamp)
//         .json(&body)
//         .send()
//         .await
//         .expect("Failed to execute request.");

//     let status = response.status();

//     #[derive(Debug, Deserialize)]
//     struct Pong {
//         #[serde(rename = "type")]
//         pub interaction_type: u8,
//     }

//     let json = response.json::<Pong>().await;

//     // Assert
//     assert_eq!(status, StatusCode::OK);
//     assert_eq!(json.unwrap().interaction_type, 1_u8);
// }
