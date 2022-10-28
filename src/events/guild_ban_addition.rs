use crate::log_channel::log_user_banned;
use crate::utils::logging::log_error;
use serenity::model::{prelude::GuildId, user::User};
use serenity::prelude::*;

pub async fn handle(ctx: Context, _guild_id: GuildId, banned_user: User) {
    match log_user_banned(&banned_user, &ctx).await {
        Ok(_) => (),
        Err(error) => log_error(&error),
    };
}
