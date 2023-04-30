use crate::{Context, Error};

/// Sends information about the bot
///
/// # Arguments
/// * `ctx` - Context passed from the invoking message.
#[poise::command(prefix_command, slash_command)]
pub async fn about(ctx: Context<'_>, arg: Option<u32>) -> Result<(), Error> {
    let title = "Jardb (Just Another Rust Discord Bot)";
    let desc = "This bot serves to be a project for me to learn Rust. For no other purpose \
                       than torturing a python developer that hasnt looked at C++ in years.";

    ctx.send(|msg| {
        msg.embed(|embd| embd
            .title(&title)
            .color(0xACACAC)
            .description(&desc)
            .field("Github", "https://github.com/perryBunn/jardb", false)
            .field("Issues", "https://github.com/perryBunn/jardb/issues", false)
        )
    }).await?;

    Ok(())
}