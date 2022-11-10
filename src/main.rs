#![crate_name = "jardb"]

mod util;
mod commands;

use clap::Parser;
use poise::serenity_prelude as serenity;
use crate::commands::{
    admin::{register::*},
    info::{age::*}
};
use crate::util::{
    data::*,
    io::*
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Discord bot that has no purpose.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Path to config file.
   #[arg(short, long)]
   config: Option<String>
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config_file = args.config.unwrap_or("$HOME/.config/jardb/config.toml".to_string());

    // Get config
    let config: Config = get_config_from_toml(&config_file).unwrap();

    // Get token
    let token = get_token(config.token_file.as_str());

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
    let timeout = std::time::Duration::from_secs(config.listen_timeout as u64);

    // Initiate discord bot.
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(config.prefix),
                edit_tracker: Some(poise::EditTracker::for_timespan(timeout)),
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
