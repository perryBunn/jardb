#![crate_name = "jardb"]

mod util;
mod commands;

use clap::Parser;
// use log::{trace, debug, info, warn, error, SetLoggerError, LevelFilter};
use poise::serenity_prelude as serenity;
use crate::commands::{
    admin::{register::*},
    fun::{coin::*, ping::*, roll::*, xkcd::*},
    info::{age::*},
    moderation::{}
};
use crate::util::{
    data::*,
    io::*,
    // logger::SimpleLogger
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// static LOGGER: SimpleLogger = SimpleLogger;

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
    // log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));
    let args = Args::parse();
    // debug!("Starting bot!");
    let config_file = args.config.unwrap_or("$HOME/.config/jardb/config.toml".to_string());
    // Get config
    let config: Config = get_config_from_toml(&config_file).unwrap();
    // debug!("config file parsed!");
    // debug!("Generating structs!");
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

    let command_list = vec![
        register(), // Admin
        coin(), ping(), roll(), xkcd(), // Fun
        age(), // Info
    ];
    let timeout = std::time::Duration::from_secs(config.bot.listen_timeout as u64);

    // debug!("Building {}", config.bot.name);
    // Initiate discord bot.
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(config.bot.prefix),
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
    // info!("{} has been configured, asking Discord for listener...", config.bot.name);
    framework.run().await.unwrap();
    // info!("{} has initialized!", config.bot.name)
}
