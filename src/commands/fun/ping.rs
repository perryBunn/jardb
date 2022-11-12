use crate::{Context, Error};

/// Ping! -> Pong!
///
/// # Arguments
/// * `ctx` - Context passed from the invoking message.
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let response: String = String::from("Pong!");
    ctx.say(response).await?;
    Ok(())
}