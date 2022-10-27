use crate::slash_commands::errors::CommandError;
use crate::utils::discord_cdn::get_avatar_url;
use chrono::prelude::*;
use chrono::{DateTime, Utc};
use serenity::builder::{CreateApplicationCommand, CreateEmbedAuthor, CreateEmbedFooter};
use serenity::client::Context;
use serenity::model::id::{ChannelId, UserId};
use serenity::model::Timestamp;

pub async fn execute(is_ephemeral: &mut bool, ctx: &Context) -> Result<String, CommandError> {
    *is_ephemeral = true;

    let channel_id = ChannelId(1034987566245621780);
    let user_id = UserId(224597366324461568);

    let user = match user_id.to_user(&ctx.http).await {
        Ok(x) => x,
        Err(e) => return Err(CommandError::Other(e.to_string())),
    };

    let success = channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                let mut author = CreateEmbedAuthor::default();
                author.icon_url(get_avatar_url(&user));
                author.name(user.name.clone());

                let mut footer = CreateEmbedFooter::default();
                footer.text(format!("ID: {}", user.id));

                let account_age = date_diff(&user.created_at());

                e.title("Member Joined")
                    .color(0x50C878)
                    .description(format!(
                        "<@{}> - {}#{}",
                        user.id, user.name, user.discriminator
                    ))
                    .image(get_avatar_url(&user))
                    .timestamp(Utc::now())
                    .set_author(author)
                    .field("Account Age", account_age, true)
                    .set_footer(footer)
            })
        })
        .await;

    match success {
        Ok(_) => return Ok("Message sent to logs...".to_string()),
        Err(e) => return Err(CommandError::Other(e.to_string())),
    };
}

pub fn setup(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("test-log-channel")
        .description("Send an embedded message to the log channel")
}

fn date_diff(date: &Timestamp) -> String {
    let today = Utc::now();

    // Check if we can access 0..10 before getting it
    let str_length = date.to_string().len();
    assert!(str_length >= 10);

    // Get only the date, not the time
    let date = &date.to_string()[0..10];

    let datetime = DateTime::<Utc>::from_utc(
        chrono::NaiveDate::parse_from_str(&date.to_string(), "%Y-%m-%d")
            .unwrap()
            .and_hms(0, 0, 0),
        Utc,
    );

    let diff = today.signed_duration_since(datetime);
    let days = diff.num_days();
    let years = days / 365;
    let remaining_days = days % 365;
    let months = remaining_days / 30;
    let days = remaining_days % 30;

    format!("{} years {} months {} days", years, months, days)
}
