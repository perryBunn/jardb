use crate::{Context, Error};
use reqwest::{get, StatusCode};
use serde_derive::{Deserialize, Serialize};
use poise::serenity_prelude::{ButtonStyle};

#[derive(Debug, Serialize, Deserialize)]
struct Comic {
    num: u16,          // Numeric ID of the xkcd comic
    alt: String,       // Caption of the xkcd comic
    img: String,       // Image URL of the xkcd comic
    title: String,     // Title of the xkcd comic
    transcript: String // Captions of the xkcd comic
}

/// Grabs either the latest xkcd comic or if a number is specified a specific comic.
///
/// # Arguments
/// * `ctx` - Context passed from the invoking message.
#[poise::command(prefix_command, slash_command)]
pub async fn xkcd(ctx: Context<'_>, arg: Option<u32>) -> Result<(), Error> {
    let comic_num = arg.unwrap_or(0);
    let selected_comic = format!("https://xkcd.com/{comic_num}/info.0.json");
    let latest_comic = "https://xkcd.com/info.0.json";

    let request = get(if comic_num == 0 { latest_comic } else { &selected_comic }).await?;
    if request.status() == StatusCode::NOT_FOUND {
        ctx.say("You did not provide a valid comic id.").await?;
        return Ok(());
    }

    let response: Comic = request.json().await?;
    let num = response.num;
    let page = format!("https://xkcd.com/{num}");
    let wiki = format!("https://explainxkcd.com/wiki/index.php/{num}");

    ctx.send(|msg| {
        msg.embed(|embd| embd
            .title(&response.title)
            .color(0xACACAC)
            .description(&response.alt)
            .image(&response.img)
            .footer(|foot| foot
                .text(format!("xkcd comic no. {num}"))
            )
        );
        msg.components(|comp| comp
            .create_action_row(|act_row| act_row
                .create_button(|btn_page| btn_page
                    .style(ButtonStyle::Link)
                    .url(page)
                    .label("View image on xkcd"))
                .create_button(|btn_wiki| btn_wiki
                    .style(ButtonStyle::Link)
                    .url(wiki)
                    .label("View wiki")
                )
            )
        )
    }).await?;

    Ok(())
}