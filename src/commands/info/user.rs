use crate::{Context, Error};

/// Replies with information about a user
///
/// # Arguments
/// * `ctx` - Context passed from the invoking message.
#[poise::command(prefix_command, slash_command)]
pub async fn user(ctx: Context<'_>, arg: Option<u32>) -> Result<(), Error> {
    let title = ctx.author().name.as_str();
    let default_avi = "http://media.sbmania.net/pictures/104a/87.png".to_string();
    let avi = ctx.author().avatar_url().unwrap_or(default_avi);

    ctx.send(|msg| {
        msg.embed(|embd| embd
            .title(title)
            .image(avi)
            .color(0xACACAC)
        )
    }).await?;

    Ok(())
}