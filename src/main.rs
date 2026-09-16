use anyhow::Context as _;
use serenity::all::{Client, GatewayIntents};
use tbe_companion::{load_config, ConfigError, DiscordHandler};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = load_config().map_err(|err| match err {
        ConfigError::MissingToken => anyhow::anyhow!("DISCORD_TOKEN must be set"),
        ConfigError::InvalidGuildId(value) => {
            anyhow::anyhow!("DISCORD_GUILD_ID must be a numeric Discord guild id, got `{value}`")
        }
    })?;

    let mut client = Client::builder(&config.discord_token, GatewayIntents::empty())
        .event_handler(DiscordHandler::new(config.guild_id))
        .await
        .context("failed to create Discord client")?;

    client
        .start()
        .await
        .context("Discord client stopped with an error")?;

    Ok(())
}
