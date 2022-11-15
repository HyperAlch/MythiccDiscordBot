use crate::redis_client;
use crate::utils::discord_cdn::get_avatar_url;
use crate::utils::time::date_diff;
use chrono::Utc;
use redis::Connection;
use serenity::builder::{CreateEmbedAuthor, CreateEmbedFooter};
use serenity::client::Context;
use serenity::model::id::{ChannelId, UserId};
use serenity::model::user::User;

use thiserror::Error;

type ErrorMessage = String;

#[derive(Error, Debug)]
pub enum LogChannelError {
    #[error("Redis: {0}")]
    RedisError(ErrorMessage),

    #[error("Error: `{0}`")]
    Other(ErrorMessage),
}

const GREEN: i32 = 0x50C878;
const RED: i32 = 0xFF0000;
const YELLOW: i32 = 0xFFFF00;

pub async fn log_voice_chat_moved(
    user: User,
    old_voice_chat_id: ChannelId,
    new_voice_chat_id: ChannelId,
    ctx: &Context,
) -> Result<(), LogChannelError> {
    let mut conn = redis_client::connect();

    let channel_id = unpack_minor_channel_id(&mut conn)?;

    let success = channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                let mut author = CreateEmbedAuthor::default();
                author.icon_url(get_avatar_url(&user));
                author.name(user.name.clone());

                let mut footer = CreateEmbedFooter::default();
                footer.text(format!("ID: {}", user.id));

                e.title("Moved Voice Chat")
                    .color(YELLOW)
                    .description(format!(
                        "<@{}> left voice channel  <#{}>\n<@{}> joined voice channel  <#{}>",
                        user.id,
                        old_voice_chat_id.to_string(),
                        user.id,
                        new_voice_chat_id.to_string()
                    ))
                    .timestamp(Utc::now())
                    .set_author(author)
                    .field(
                        "Username",
                        format!("{}#{}", user.name, user.discriminator),
                        true,
                    )
                    .set_footer(footer)
            })
        })
        .await;

    match success {
        Ok(_) => return Ok(()),
        Err(e) => return Err(LogChannelError::Other(e.to_string())),
    };
}

pub async fn log_voice_chat_left(
    user: User,
    voice_chat_id: ChannelId,
    ctx: &Context,
) -> Result<(), LogChannelError> {
    let mut conn = redis_client::connect();

    let channel_id = unpack_minor_channel_id(&mut conn)?;

    let success = channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                let mut author = CreateEmbedAuthor::default();
                author.icon_url(get_avatar_url(&user));
                author.name(user.name.clone());

                let mut footer = CreateEmbedFooter::default();
                footer.text(format!("ID: {}", user.id));

                e.title("Left Voice Chat")
                    .color(RED)
                    .description(format!(
                        "<@{}> left voice channel  <#{}>",
                        user.id,
                        voice_chat_id.to_string()
                    ))
                    .timestamp(Utc::now())
                    .set_author(author)
                    .field(
                        "Username",
                        format!("{}#{}", user.name, user.discriminator),
                        true,
                    )
                    .set_footer(footer)
            })
        })
        .await;

    match success {
        Ok(_) => return Ok(()),
        Err(e) => return Err(LogChannelError::Other(e.to_string())),
    };
}
pub async fn log_roles_updated(
    user: User,
    new_roles: Vec<String>,
    old_roles: Vec<String>,
    ctx: &Context,
) -> Result<(), LogChannelError> {
    let mut conn = redis_client::connect();

    let channel_id = unpack_major_channel_id(&mut conn)?;

    let old_roles = match old_roles.len() {
        0 => "None".to_string(),
        _ => old_roles.join(" "),
    };

    let new_roles = match new_roles.len() {
        0 => "None".to_string(),
        _ => new_roles.join(" "),
    };

    let success = channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                let mut author = CreateEmbedAuthor::default();
                author.icon_url(get_avatar_url(&user));
                author.name(user.name.clone());

                let mut footer = CreateEmbedFooter::default();
                footer.text(format!("ID: {}", user.id));

                e.title("Roles Updated")
                    .color(YELLOW)
                    .field("New Roles: ", new_roles, true)
                    .field("Removed Roles: ", old_roles, true)
                    .timestamp(Utc::now())
                    .set_author(author)
                    .field(
                        "Username",
                        format!("<@{}> - {}#{}", user.id, user.name, user.discriminator),
                        false,
                    )
                    .set_footer(footer)
            })
        })
        .await;

    match success {
        Ok(_) => return Ok(()),
        Err(e) => return Err(LogChannelError::Other(e.to_string())),
    };
}
pub async fn log_voice_chat_joined(
    user: User,
    voice_chat_id: ChannelId,
    ctx: &Context,
) -> Result<(), LogChannelError> {
    let mut conn = redis_client::connect();

    let channel_id = unpack_minor_channel_id(&mut conn)?;

    let success = channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                let mut author = CreateEmbedAuthor::default();
                author.icon_url(get_avatar_url(&user));
                author.name(user.name.clone());

                let mut footer = CreateEmbedFooter::default();
                footer.text(format!("ID: {}", user.id));

                e.title("Joined Voice Chat")
                    .color(GREEN)
                    .description(format!(
                        "<@{}> joined voice channel  <#{}>",
                        user.id,
                        voice_chat_id.to_string()
                    ))
                    .timestamp(Utc::now())
                    .set_author(author)
                    .field(
                        "Username",
                        format!("{}#{}", user.name, user.discriminator),
                        true,
                    )
                    .set_footer(footer)
            })
        })
        .await;

    match success {
        Ok(_) => return Ok(()),
        Err(e) => return Err(LogChannelError::Other(e.to_string())),
    };
}

pub async fn log_user_unbanned(unbanned_user: &User, ctx: &Context) -> Result<(), LogChannelError> {
    let mut conn = redis_client::connect();

    let channel_id = unpack_major_channel_id(&mut conn)?;

    let user = unbanned_user;

    let success = channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                let mut author = CreateEmbedAuthor::default();
                author.icon_url(get_avatar_url(&user));
                author.name(user.name.clone());

                let mut footer = CreateEmbedFooter::default();
                footer.text(format!("ID: {}", user.id));

                let account_age = date_diff(&user.created_at());

                e.title("Member Unbanned")
                    .color(GREEN)
                    .description(format!(
                        "<@{}> - {}#{}",
                        user.id, user.name, user.discriminator
                    ))
                    .image("https://i.ibb.co/7nqVFKd/unbanned.png")
                    .timestamp(Utc::now())
                    .set_author(author)
                    .field("Account Age", account_age, true)
                    .set_footer(footer)
            })
        })
        .await;

    match success {
        Ok(_) => return Ok(()),
        Err(e) => return Err(LogChannelError::Other(e.to_string())),
    };
}

pub async fn log_user_banned(banned_user: &User, ctx: &Context) -> Result<(), LogChannelError> {
    let mut conn = redis_client::connect();

    let channel_id = unpack_major_channel_id(&mut conn)?;

    let user = banned_user;

    let success = channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                let mut author = CreateEmbedAuthor::default();
                author.icon_url(get_avatar_url(&user));
                author.name(user.name.clone());

                let mut footer = CreateEmbedFooter::default();
                footer.text(format!("ID: {}", user.id));

                let account_age = date_diff(&user.created_at());

                e.title("Member Banned")
                    .color(RED)
                    .description(format!(
                        "<@{}> - {}#{}",
                        user.id, user.name, user.discriminator
                    ))
                    .image("https://i.ibb.co/P4m8YSL/banned.png")
                    .timestamp(Utc::now())
                    .set_author(author)
                    .field("Account Age", account_age, true)
                    .set_footer(footer)
            })
        })
        .await;

    match success {
        Ok(_) => return Ok(()),
        Err(e) => return Err(LogChannelError::Other(e.to_string())),
    };
}

pub async fn log_user_joined(user_id: &UserId, ctx: &Context) -> Result<(), LogChannelError> {
    let mut conn = redis_client::connect();

    let user = match user_id.to_user(&ctx.http).await {
        Ok(x) => x,
        Err(e) => return Err(LogChannelError::Other(e.to_string())),
    };

    let channel_id = unpack_major_channel_id(&mut conn)?;

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
                    .color(GREEN)
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
        Ok(_) => return Ok(()),
        Err(e) => return Err(LogChannelError::Other(e.to_string())),
    };
}

pub async fn log_user_left(user: &User, ctx: &Context) -> Result<(), LogChannelError> {
    let mut conn = redis_client::connect();

    let channel_id = unpack_major_channel_id(&mut conn)?;

    let success = channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                let mut author = CreateEmbedAuthor::default();
                author.icon_url(get_avatar_url(&user));
                author.name(user.name.clone());

                let mut footer = CreateEmbedFooter::default();
                footer.text(format!("ID: {}", user.id));

                let account_age = date_diff(&user.created_at());

                e.title("Member Left")
                    .color(RED)
                    .description(format!(
                        "<@{}> - {}#{}",
                        user.id, user.name, user.discriminator
                    ))
                    .image("https://i.ibb.co/1qyVmzG/left-discord.png")
                    .timestamp(Utc::now())
                    .set_author(author)
                    .field("Account Age", account_age, true)
                    .set_footer(footer)
            })
        })
        .await;

    match success {
        Ok(_) => return Ok(()),
        Err(e) => return Err(LogChannelError::Other(e.to_string())),
    };
}

fn unpack_major_channel_id(conn: &mut Connection) -> Result<ChannelId, LogChannelError> {
    // Query and unpack the log channel id from Redis
    let channel_id = match redis_client::get_major_log_channel(conn) {
        Ok(value) => match value {
            Some(value) => match value.parse::<u64>() {
                Ok(value) => value,
                Err(_) => {
                    return Err(LogChannelError::Other(
                        "Could not parse major log channel id into u64".to_string(),
                    ))
                }
            },
            None => {
                return Err(LogChannelError::RedisError(
                    "Could not resolve major log channel id".to_string(),
                ))
            }
        },
        Err(e) => return Err(LogChannelError::Other(e.to_string())),
    };

    Ok(ChannelId(channel_id))
}

fn unpack_minor_channel_id(conn: &mut Connection) -> Result<ChannelId, LogChannelError> {
    // Query and unpack the log channel id from Redis
    let channel_id = match redis_client::get_minor_log_channel(conn) {
        Ok(value) => match value {
            Some(value) => match value.parse::<u64>() {
                Ok(value) => value,
                Err(_) => {
                    return Err(LogChannelError::Other(
                        "Could not parse minor log channel id into u64".to_string(),
                    ))
                }
            },
            None => {
                return Err(LogChannelError::RedisError(
                    "Could not resolve minor log channel id".to_string(),
                ))
            }
        },
        Err(e) => return Err(LogChannelError::Other(e.to_string())),
    };

    Ok(ChannelId(channel_id))
}
