use poise::serenity_prelude::{Guild, GuildChannel};
use crate::{Context, Error};

/// Displays your or another user's account creation date
///
/// # Arguments
/// * `ctx` - Context passed from the invoking message.
/// * `user` - User to get the age of. By default will get the age of the user that invoked the command.
#[poise::command(slash_command, prefix_command)]
pub async fn clone(ctx: Context<'_>) -> Result<(), Error> {
    let origin_channel_id = ctx.channel_id();
    let guild = ctx.guild().unwrap();
    let guild_channels = guild.channels(ctx.discord()).await.unwrap();
    let origin_guild_channel: &GuildChannel = guild_channels.get(&origin_channel_id).unwrap();
    let channel_copy_name = format!("{}_copy", origin_guild_channel.clone().name);
    _clone_channel(ctx, guild, origin_guild_channel, channel_copy_name.clone()).await?;
    let response= format!("#{}", channel_copy_name);
    ctx.say(response).await?;
    Ok(())
}

pub async fn _clone_channel(ctx: Context<'_>, guild: Guild, channel: &GuildChannel, channel_name: String) -> Result<(), Error> {
    let _ = guild.create_channel(ctx.discord(), |c| c
        .category(channel.parent_id.unwrap())
        .kind(channel.kind)
        .name(channel_name)
        .nsfw(channel.nsfw)
        // .permissions()
        .position((channel.position + 1) as u32)
        .topic(channel.topic.as_ref().unwrap_or(&String::from("")))
    ).await;
    Ok(())
}