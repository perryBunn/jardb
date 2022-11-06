#![crate_name = "jardb"]

mod lib;

use poise::serenity_prelude as serenity;
use lib::io;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
struct Data {}

/// Displays your or another user's account creation date
///
/// # Arguments
/// * `ctx` - Context passed from the invoking message.
/// * `user` - User to get the age of. By default will get the age of the user that invoked the command.
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Registers slash commands with discord.
///
/// # Arguments
/// * `ctx` - Context passed from the invoking message.
#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Get config
    let config_result = io::get_config("config.yaml", Some(0));
    let config = match config_result {
        Ok(yaml) => yaml,
        Err(err) => panic!("Problem opening the file: {:?}", err),
    };
    // Get token
    let token = io::get_token(config["token_file"].as_str().unwrap());

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
    // Initiate discord bot.
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("$".into()),
                edit_tracker: Some(poise::EditTracker::for_timespan(std::time::Duration::from_secs(config["listen_timeout"].as_i64().unwrap() as u64))),
                case_insensitive_commands: true,
                ..Default::default()
            },
            commands: vec![age(), register()],
            ..Default::default()
        })
        .token(token)
        .intents(intents)
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}
