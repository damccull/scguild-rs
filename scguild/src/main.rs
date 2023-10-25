use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use scguild::{fleet, Context, Data};
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;
use tracing::instrument;

/// Responds with "world!"
#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), anyhow::Error> {
    ctx.say("world!").await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), anyhow::Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[shuttle_runtime::main]
#[instrument(name = "SCGuild", skip(secret_store))]
async fn poise(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttlePoise<Data, anyhow::Error> {
    // Set up tracing
    //let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    //telemetry::init_subscriber(subscriber);

    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: {
                let mut cmds = vec![hello(), age()];
                // Add fleet commands
                cmds.extend(fleet::add_commands());
                cmds
            },
            ..Default::default()
        })
        .token(discord_token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;
    tracing::info!("Startup successful.");
    Ok(framework.into())
}
