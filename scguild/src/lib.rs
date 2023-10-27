use error::ResponseError;
use http::StatusCode;
use poise::Command;

pub mod application;
pub mod authentication;
pub mod configuration;
pub mod error;
pub mod fleet;
pub mod routes;
pub mod session_state;
pub mod telemetry;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Context<'a> = poise::Context<'a, Data, anyhow::Error>;
pub type CommandList = Vec<Command<Data, anyhow::Error>>;

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Cause by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

pub fn e400<T>(e: T) -> ResponseError
where
    T: std::fmt::Debug,
    T: std::fmt::Display + 'static,
    T: Into<Box<dyn std::error::Error>>,
{
    ResponseError::from(e).set_status(StatusCode::BAD_REQUEST)
}

pub fn e500<T>(e: T) -> ResponseError
where
    T: std::fmt::Debug,
    T: std::fmt::Display + 'static,
    T: Into<Box<dyn std::error::Error>>,
{
    ResponseError::from(e)
}
