use crate::{Context, Error};

/// Displays your or another user's account creation date
///
/// # Arguments
/// * `ctx` - Context passed from the invoking message.
/// * `user` - User to get the age of. By default will get the age of the user that invoked the command.
#[poise::command(slash_command, prefix_command)]
pub async fn delete(ctx: Context<'_>, #[description = "Selected user"] number: u64) -> Result<(), Error> {
    let num = number;
    let mut response = format!("The number you passed doesnt pass this comparison: 1 < {num} <= 100");
    if (1 < num) && (num <= 100) {
        let channel_id = ctx.channel_id();
        let messages = channel_id.messages(ctx.discord(), |retriever| retriever.before(ctx.id()).limit(num)).await.unwrap();
        channel_id.delete_messages(ctx.discord(), messages).await?;
        channel_id.delete_message(ctx.discord(), ctx.id()).await?;
        response = format!("Deleted {num} messages.");
    }

    ctx.say(response).await?;
    Ok(())
}

pub async fn _delete_channel() -> Result<(), Error> {
    Ok(())
}