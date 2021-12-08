use std::num::NonZeroU64;

use crate::helpers::TestApp;
use actix_http::StatusCode;
use chrono::Utc;
use ed25519_dalek::{Keypair, Signature, Signer};
use serde::Deserialize;
use twilight_model::{
    application::interaction::{self, InteractionType},
    id::{ApplicationId, InteractionId},
};

#[actix_rt::test]
async fn discord_api_responds_with_pong_when_given_ping() {
    // Arrange
    let test_app = TestApp::spawn().await;
    let keypair = test_app.discord_keypair;

    let body = interaction::Ping {
        application_id: ApplicationId {
            0: NonZeroU64::new(12351532).unwrap(),
        },
        id: InteractionId {
            0: NonZeroU64::new(684641).unwrap(),
        },
        kind: InteractionType::Ping,
        token: "blart".to_string(),
    };

    let x = serde_json::to_string(&body).unwrap();

    let timestamp = Utc::now().to_string();
    let signme = format!("{}{}", timestamp.to_string(), x);
    let signature = keypair.sign(signme.as_bytes());

    // Act
    let response = reqwest::Client::new()
        .post(format!("{}/api/discord/", &test_app.address))
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
    dbg!(&json);

    // Assert
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json.unwrap().interaction_type, 1_u8);
}
