use crate::{Context, Error};

/// Registers slash commands with discord.
///
/// # Arguments
/// * `ctx` - Context passed from the invoking message.
#[poise::command(prefix_command, slash_command)]
pub async fn coin(ctx: Context<'_>) -> Result<(), Error> {
    let response: String;
    let num = rand::random::<u8>();
    if num >= 4 { // 4-8
        response = String::from("Heads");
    } else { // 0-3
        response = String::from("Tails");
    }
    ctx.say(response).await?;
    Ok(())
}