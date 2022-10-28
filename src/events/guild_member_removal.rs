use serenity::model::guild::Member;
use serenity::model::prelude::GuildId;
use serenity::model::user::User;
use serenity::prelude::*;

use crate::log_channel::log_user_left;
use crate::utils::logging::log_error;

pub async fn handle(ctx: Context, _guild_id: GuildId, user: User, _member_data: Option<Member>) {
    match log_user_left(&user, &ctx).await {
        Ok(_) => (),
        Err(error) => log_error(&error),
    };
}
