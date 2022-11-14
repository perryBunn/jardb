use rand::Rng;
use crate::{Context, Error};

/// Registers slash commands with discord.
///
/// # Arguments
/// * `ctx` - Context passed from the invoking message.
#[poise::command(prefix_command, slash_command)]
pub async fn roll(ctx: Context<'_>, arg: String) -> Result<(), Error> {
    match ctx {
        Context::Application(ctx) => {
            // Something has gon horribly wrong.
        }
        Context::Prefix(pctx) => {
            let ceiling: u32 = arg.parse::<u32>().unwrap();
            let num = rand::thread_rng().gen_range(0..ceiling) + 1;
            let response = format!("{num}");
            ctx.say(response).await?;
        }
    }
    Ok(())
}