use crate::{Context, Error};
// use reqwest::{get, StatusCode};
// use serde_derive::{Deserialize, Serialize};
use poise::serenity_prelude::{ButtonStyle};


/// Grabs either the latest xkcd comic or if a number is specified a specific comic.
///
/// # Arguments
/// * `ctx` - Context passed from the invoking message.
#[poise::command(prefix_command, slash_command)]
pub async fn poll(ctx: Context<'_>, arg: Option<u32>) -> Result<(), Error> {
    let question = "";
    let answer_1 = "";
    let answer_2 ="";

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
                .create_button(|btn_page| btn_page
                    .custom_id(1)
                    .label("1"))
                    .style(ButtonStyle::Primary)
                .create_button(|btn_wiki| btn_wiki
                    .custom_id(2)
                    .label("2"))
                    .style(ButtonStyle::Primary)
            )
        )
    }).await?;

    Ok(())
}
