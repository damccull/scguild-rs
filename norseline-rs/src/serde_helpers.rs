use ed25519_dalek::{PublicKey, PUBLIC_KEY_LENGTH};
use hex::FromHex;
use serde::{Deserialize, Deserializer};

pub fn deserialize_discord_public_key_from_string<'de, D>(
    deserializer: D,
) -> Result<PublicKey, D::Error>
where
    D: Deserializer<'de>,
{
    let x = String::deserialize(deserializer)?;

    let bytes = match <[u8; PUBLIC_KEY_LENGTH] as FromHex>::from_hex(x) {
        Ok(x) => x,
        Err(e) => {
            return Err(serde::de::Error::custom(format!(
                "Failed to read public key from config file: {}",
                e
            )))
        }
    };
    let pubkey = PublicKey::from_bytes(&bytes);
    match pubkey {
        Ok(x) => Ok(x),
        Err(e) => Err(serde::de::Error::custom(format!(
            "Failed to parse public key from bytes: {}",
            e
        ))),
    }
}
