#![crate_name = "jardb"]


mod util;
mod commands;

use clap::Parser;
use poise::serenity_prelude as serenity;

use crate::commands::{
    info::{age::*}
};
use crate::util::io::*;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// User data, which is stored and accessible in all command invocations
pub struct Data {}



/// Registers slash commands with discord.
///
/// # Arguments
/// * `ctx` - Context passed from the invoking message.
#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Path to config file.
   #[arg(short, long)]
   config: Option<String>
}

use serde_derive::Deserialize;
struct Config {
    logging_level: String,
    token_file: String,
    listen_timeout: u64,
    prefix: String
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config_file = args.config.unwrap_or("$HOME/.config/jardb/config.yaml".to_string());
    // Get config
    let config_result = get_config_from_yaml(&config_file, None);
    let config = match config_result {
        Ok(yaml) => yaml,
        Err(err) => panic!("Problem opening the file: {:?}", err),
    };
    // Get token
    let token = get_token(config["token_file"].as_str().unwrap());

    // Set intents
    let intents = serenity::GatewayIntents::GUILDS
                  | serenity::GatewayIntents::GUILD_MEMBERS
                  | serenity::GatewayIntents::GUILD_BANS
                  | serenity::GatewayIntents::GUILD_EMOJIS_AND_STICKERS
                  | serenity::GatewayIntents::GUILD_INVITES
                  | serenity::GatewayIntents::GUILD_PRESENCES
                  | serenity::GatewayIntents::GUILD_MESSAGES
                  | serenity::GatewayIntents::GUILD_MESSAGE_REACTIONS
                  | serenity::GatewayIntents::GUILD_MESSAGE_TYPING
                  | serenity::GatewayIntents::MESSAGE_CONTENT;

    let command_list = vec![age(), register()];
    // Initiate discord bot.
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: config["prefix"].clone().into_string(),
                edit_tracker: Some(poise::EditTracker::for_timespan(std::time::Duration::from_secs(config["listen_timeout"].as_i64().unwrap() as u64))),
                case_insensitive_commands: true,
                ..Default::default()
            },
            commands: command_list,
            ..Default::default()
        })
        .token(token)
        .intents(intents)
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}
