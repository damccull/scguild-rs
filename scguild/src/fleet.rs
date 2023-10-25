use crate::{CommandList, Context};

#[tracing::instrument(name = "Fleet Commands")]
pub fn add_commands() -> CommandList {
    tracing::info!("Adding fleet commands");
    let commands = vec![fleet()];
    commands
}

#[poise::command(slash_command, subcommands("list", "add", "decommission", "set"))]
#[tracing::instrument(name = "Command: `/fleet`", skip(_ctx))]
pub async fn fleet(_ctx: Context<'_>) -> Result<(), anyhow::Error> {
    // This is a parent /command, and since it's not also a 'prefix' command
    // it should never be able to be called by itself.
    tracing::error!("fleet called - this should be impossible without prefix commands");
    anyhow::bail!("Illegal action")
}

#[poise::command(slash_command)]
#[tracing::instrument(name = "Command: `/fleet list`", skip(ctx))]
pub async fn list(ctx: Context<'_>) -> Result<(), anyhow::Error> {
    tracing::debug!("Listing the fleet");
    ctx.say("Fleet list".to_string()).await?;
    Ok(())
}

#[poise::command(slash_command)]
#[tracing::instrument(name = "Command: `/fleet add`", skip(ctx))]
pub async fn add(ctx: Context<'_>) -> Result<(), anyhow::Error> {
    tracing::debug!("Adding a ship to the fleet");
    ctx.say("Fleet add".to_string()).await?;
    Ok(())
}
#[poise::command(slash_command)]
#[tracing::instrument(name = "Command: `/fleet decommission`", skip(ctx))]
pub async fn decommission(ctx: Context<'_>) -> Result<(), anyhow::Error> {
    tracing::debug!("Decommissioning a ship");
    ctx.say("Fleet decommission".to_string()).await?;
    Ok(())
}
#[poise::command(slash_command)]
#[tracing::instrument(name = "Command: `/fleet set`", skip(ctx))]
pub async fn set(
    ctx: Context<'_>,
    #[description = "Ship name"] ship_name: Option<String>,
    #[description = "SC registration number"] sc_registration_number: Option<String>,
) -> Result<(), anyhow::Error> {
    tracing::debug!("Setting ship values");
    let mut response: Vec<String> = vec!["Setting options:".to_string()];
    if let Some(s_name) = ship_name {
        response.push(format!("`ship_name={s_name}`"));
    };
    if let Some(regnum) = sc_registration_number {
        response.push(format!("`sc_registration_number={regnum}`"));
    }
    ctx.say(response.join("\n")).await?;
    Ok(())
}
