use std::fmt::format;
use crate::{Context, Error};

/// Registers slash commands with discord.
///
/// # Arguments
/// * `ctx` - Context passed from the invoking message.
#[poise::command(prefix_command, slash_command)]
pub async fn roll(ctx: Context<'_>) -> Result<(), Error> {
    match ctx {
        Context::Application(ctx) => {
            // Something has gon horribly wrong.
        }
        Context::Prefix(ctx) => {
            let digit_iter = ctx.msg.content.as_str().chars().filter(|c| c.is_digit(10)).collect();
            let ceiling: u32 = 0;
            let num = rand::thread_rng().gen_range(0..ceiling);;
            let response = format!("{num}");
            ctx.say(response).await?;
        }
    }
    Ok(())
}