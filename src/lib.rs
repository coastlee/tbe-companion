use std::collections::HashMap;

use serenity::all::{
    Command, CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage, EventHandler, GuildId, Interaction, Ready,
};
use serenity::async_trait;
use tracing::{error, info};

const ABOUT_RESPONSE: &str = "TBE Companion is online and ready to help.";
type SerenityResult<T> = Result<T, Box<serenity::Error>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub discord_token: String,
    pub guild_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    MissingToken,
    InvalidGuildId(String),
}

pub fn load_config_from_env(env: &HashMap<String, String>) -> Result<AppConfig, ConfigError> {
    let discord_token = env
        .get("DISCORD_TOKEN")
        .map(|token| token.trim())
        .filter(|token| !token.is_empty())
        .ok_or(ConfigError::MissingToken)?
        .to_owned();

    let guild_id = env
        .get("DISCORD_GUILD_ID")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(|value| {
            value
                .parse::<u64>()
                .map_err(|_| ConfigError::InvalidGuildId(value.to_owned()))
        })
        .transpose()?;

    Ok(AppConfig {
        discord_token,
        guild_id,
    })
}

pub fn command_response(command_name: &str) -> &'static str {
    match command_name {
        "ping" => "Pong!",
        "about" => ABOUT_RESPONSE,
        _ => "Unknown command.",
    }
}

pub fn application_commands() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new("ping").description("Check whether TBE Companion is running."),
        CreateCommand::new("about").description("Show a short status message for TBE Companion."),
    ]
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    load_config_from_env(&std::env::vars().collect())
}

pub struct DiscordHandler {
    guild_id: Option<u64>,
}

impl DiscordHandler {
    pub fn new(guild_id: Option<u64>) -> Self {
        Self { guild_id }
    }

    async fn register_commands(&self, ctx: &Context) -> SerenityResult<()> {
        let commands = application_commands();

        match self.guild_id {
            Some(guild_id) => {
                GuildId::new(guild_id)
                    .set_commands(&ctx.http, commands)
                    .await
                    .map_err(Box::new)?;
                info!(guild_id, "registered guild application commands");
            }
            None => {
                Command::set_global_commands(&ctx.http, commands)
                    .await
                    .map_err(Box::new)?;
                info!("registered global application commands");
            }
        }

        Ok(())
    }

    async fn handle_command(&self, ctx: &Context, command: &CommandInteraction) {
        let content = command_response(command.data.name.as_str());
        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(content),
        );

        if let Err(err) = command.create_response(&ctx.http, response).await {
            error!(
                command = command.data.name.as_str(),
                error = %err,
                "failed to respond to interaction"
            );
        }
    }
}

#[async_trait]
impl EventHandler for DiscordHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!(user = %ready.user.name, "connected to Discord");

        if let Err(err) = self.register_commands(&ctx).await {
            error!(error = %err, "failed to register application commands");
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            self.handle_command(&ctx, &command).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    fn env(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect()
    }

    #[test]
    fn load_config_requires_a_discord_token() {
        assert_matches!(
            load_config_from_env(&env(&[])),
            Err(ConfigError::MissingToken)
        );
    }

    #[test]
    fn load_config_accepts_token_without_guild_id() {
        let config = load_config_from_env(&env(&[("DISCORD_TOKEN", "abc123")])).unwrap();

        assert_eq!(config.discord_token, "abc123");
        assert_eq!(config.guild_id, None);
    }

    #[test]
    fn load_config_parses_optional_guild_id() {
        let config = load_config_from_env(&env(&[
            ("DISCORD_TOKEN", "abc123"),
            ("DISCORD_GUILD_ID", "1234567890"),
        ]))
        .unwrap();

        assert_eq!(config.guild_id, Some(1_234_567_890));
    }

    #[test]
    fn load_config_treats_blank_guild_id_as_absent() {
        let config = load_config_from_env(&env(&[
            ("DISCORD_TOKEN", "abc123"),
            ("DISCORD_GUILD_ID", "   "),
        ]))
        .unwrap();

        assert_eq!(config.guild_id, None);
    }

    #[test]
    fn load_config_rejects_invalid_guild_id() {
        assert_matches!(
            load_config_from_env(&env(&[
                ("DISCORD_TOKEN", "abc123"),
                ("DISCORD_GUILD_ID", "not-a-number"),
            ])),
            Err(ConfigError::InvalidGuildId(value)) if value == "not-a-number"
        );
    }

    #[test]
    fn command_response_handles_supported_commands() {
        assert_eq!(command_response("ping"), "Pong!");
        assert_eq!(
            command_response("about"),
            "TBE Companion is online and ready to help."
        );
    }
}
