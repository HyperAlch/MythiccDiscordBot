use crate::log_channel::log_user_unbanned;
use crate::utils::logging::log_error;
use serenity::model::{prelude::GuildId, user::User};
use serenity::prelude::*;

pub async fn handle(ctx: Context, _guild_id: GuildId, unbanned_user: User) {
    match log_user_unbanned(&unbanned_user, &ctx).await {
        Ok(_) => (),
        Err(error) => log_error(&error),
    };
}
