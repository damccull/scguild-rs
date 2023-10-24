//! This module handles settings for the server.
//!
//! The root [`Settings`] struct holds subcategories that contain individual
//! settings. It may also contain individual settings that don't fit into an existing
//! category and don't warrant an entirely new subcategory.

use config::Config;
use ed25519_dalek::PublicKey;
use std::convert::{TryFrom, TryInto};

use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    ConnectOptions,
};
use tracing::log::LevelFilter;

use crate::serde_helpers::deserialize_discord_public_key_from_string;

/// Initializes the server's settings from configuration files and environment variables
/// and returns a [`Settings`] struct.
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Get the application's base path
    let base_path = std::env::current_dir().expect("Failed to determine the current directory.");

    // Join the configuration directory to the app's base path
    let configuration_directory = base_path.join("configuration");

    // Instantiate a builder and set the base settings file
    // TODO: Consider whether to set these base options using hard-coded defaults or not
    let mut builder = Config::builder()
        .add_source(config::File::from(configuration_directory.join("base")).required(true));

    // Detect the running environment; default to 'local' if unspecified
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    builder = builder.add_source(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    );

    // Layer on any settings from environment variables
    // Environment variables prefixed with 'APP' and using '__' as a separator
    // E.g. 'APP_APPLICATION__PORT=5001' will set 'Settings.application.port' to 5001
    builder = builder.add_source(
        config::Environment::with_prefix("app")
            //.prefix_separator("_") // TODO: Uncomment this when released to fix the prefix separator
            .separator("__"),
    );

    builder.build()?.try_deserialize()
}

/// The root settings struct.
///
/// See `get_configuration` for the preferred way to instantiate this.
#[derive(Clone, serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub discord: DiscordSettings,
}

/// Represents database specific settings.
///
/// See `get_configuration` for the preferred way to instantiate this.
#[derive(Clone, serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}
impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
            .log_statements(LevelFilter::Trace)
            .to_owned()
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}

/// Represents application specific settings.
///
/// See `get_configuration` for the preferred way to instantiate this.
#[derive(Clone, serde::Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
}

#[derive(Clone, serde::Deserialize)]
pub struct DiscordSettings {
    pub application_id: u64,
    #[serde(deserialize_with = "deserialize_discord_public_key_from_string")]
    pub public_key: PublicKey,
    pub client_id: u64,
    pub client_secret: String,
    pub guild_id: u64,
}

/// Represents the environment in which the server is running.
pub enum Environment {
    Local,
    Production,
    CI,
}
impl Environment {
    /// Returns the environment as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
            Environment::CI => "ci",
        }
    }
}
impl TryFrom<String> for Environment {
    type Error = String;

    /// Attempts to parse an [`Environment`] from a given string.
    /// Returns an error if the given environment string is invalid.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            "ci" => Ok(Self::CI),
            other => Err(format!(
                "{} is not a supported environment. Use either 'local' or 'production'.",
                other
            )),
        }
    }
}
