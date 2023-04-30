extern crate chrono;

use std::future::Future;
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::{Duration, SystemTime};
use crate::{Context, Error};
// use reqwest::{get, StatusCode};
// use serde_derive::{Deserialize, Serialize};
use poise::serenity_prelude::{ButtonStyle, ShardMessenger};


/// Grabs either the latest xkcd comic or if a number is specified a specific comic.
///
/// # Arguments
/// * `ctx` - Context passed from the invoking message.
#[poise::command(prefix_command, slash_command)]
pub async fn poll(ctx: Context<'_>, #[description = "Question"] question: Option<String>) -> Result<(), Error> {
    let current_time = SystemTime::now();
    let datetime: DateTime<Utc> = current_time.into();
    let no_response = datetime.format("%Y%m%d%H%M%S%f").to_string();
    let mut question = question.unwrap_or(no_response.clone());

    if question == no_response {
        ctx.say("What is the poll text?");
        question = ctx.author()
            .await_reply(&ctx.discord().shard)
                .channel_id(ctx.channel_id())
                .author_id(ctx.author().id)
                .timeout(Duration::from_secs(30)).await.unwrap().content.clone();
    }

    ctx.say("How many options are there?");
    let num_options = ctx.author()
            .await_reply(&ctx.discord().shard)
                .channel_id(ctx.channel_id())
                .author_id(ctx.author().id)
                .timeout(Duration::from_secs(30)).await.unwrap();

    let answer_1 = "";
    let answer_2 = "";

    ctx.send(|msg| {
        msg.embed(|embd| embd
            .color(0xACACAC)
            .description(question)
            .field("1", answer_1, false)
            .field("2", answer_2, false)
            .title("Poll")
        );
        msg.components(|comp| comp
            .create_action_row(|act_row| act_row
                .create_button(|btn_one| btn_one
                    .custom_id(1)
                    .label("1")
                    .style(ButtonStyle::Primary))
                .create_button(|btn_two| btn_two
                    .custom_id(2)
                    .label("2")
                    .style(ButtonStyle::Primary))
            )
        )
    }).await?;

    Ok(())
}
