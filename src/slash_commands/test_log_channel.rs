use crate::redis_client;
use crate::slash_commands::errors::CommandError;
use crate::utils::discord_cdn::get_avatar_url;
use crate::utils::time::date_diff;
use chrono::Utc;
use serenity::builder::{CreateApplicationCommand, CreateEmbedAuthor, CreateEmbedFooter};
use serenity::client::Context;
use serenity::model::id::{ChannelId, UserId};

pub async fn execute(is_ephemeral: &mut bool, ctx: &Context) -> Result<String, CommandError> {
    *is_ephemeral = true;

    let user_id = UserId(224597366324461568);

    let mut conn = redis_client::connect();

    // Query and unpack the log channel id from Redis
    let channel_id = match redis_client::get_major_log_channel(&mut conn) {
        Ok(value) => match value {
            Some(value) => match value.parse::<u64>() {
                Ok(value) => value,
                Err(_) => {
                    return Err(CommandError::Other(
                        "Could not parse log channel id into u64".to_string(),
                    ))
                }
            },
            None => {
                return Err(CommandError::RedisError(
                    "Could not resolve log channel id".to_string(),
                ))
            }
        },
        Err(e) => return Err(CommandError::Other(e.to_string())),
    };

    let channel_id = ChannelId(channel_id);

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
                    .field("Roles Given: ", "<@&888565264705273886>", false)
                    .field("Roles Taken: ", "<@&888565264705273886>", false)
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
