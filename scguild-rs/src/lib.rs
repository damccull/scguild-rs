pub mod application;
pub mod configuration;
pub mod database;
pub mod discord;
pub mod es_datastore;
pub mod middleware;
pub mod serde_helpers;
pub mod telemetry;
pub mod webapp;

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
