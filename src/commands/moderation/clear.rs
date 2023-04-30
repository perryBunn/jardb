use poise::serenity_prelude::GuildChannel;
use crate::{Context, Error};
use crate::commands::moderation::clone::_clone_channel;

/// Displays your or another user's account creation date
///
/// # Arguments
/// * `ctx` - Context passed from the invoking message.
/// * `user` - User to get the age of. By default will get the age of the user that invoked the command.
#[poise::command(slash_command, prefix_command)]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    let origin_channel_id = ctx.channel_id();
    let guild = ctx.guild().unwrap();
    let guild_channels = guild.channels(ctx.discord()).await.unwrap();
    let origin_guild_channel: &GuildChannel = guild_channels.get(&origin_channel_id).unwrap();
    let channel_copy_name = format!("{}_copy", origin_guild_channel.name);
    _clone_channel(ctx, guild, origin_guild_channel, channel_copy_name).await?;


    Ok(())
}