use crate::events::errors::GuildMemberAdditionError;
use crate::utils::logging::log_error;
use serenity::client::Context;
use serenity::model::guild::Member;
use serenity::model::prelude::RoleId;

use crate::redis_client;

pub async fn handle(ctx: Context, new_member: Member) {
    let mut new_member = new_member;

    let mut connection = redis_client::connect();

    match give_follower_role(&mut new_member, &mut connection, &ctx).await {
        Ok(_) => {}
        Err(error) => log_error(&error),
    };

    // TODO: Logic for checking for a "log" channel ID and logging this
    // event to that channel.
}

async fn give_follower_role(
    new_member: &mut Member,
    connection: &mut redis::Connection,
    ctx: &Context,
) -> Result<(), GuildMemberAdditionError> {
    let follower_role = match redis_client::get_follower_role(connection) {
        Ok(role_id_wrapped) => match role_id_wrapped {
            Some(role_id) => role_id,
            None => {
                return Err(GuildMemberAdditionError::RedisError(
                    "Follower role resolved to none".to_string(),
                ))
            }
        },
        Err(e) => return Err(GuildMemberAdditionError::RedisError(e.to_string())),
    };

    let follower_role: u64 = match follower_role.parse() {
        Ok(x) => x,
        Err(_) => {
            return Err(GuildMemberAdditionError::InvalidData(
                "Follower role ID".to_string(),
            ))
        }
    };

    let follower_role = RoleId(follower_role).to_role_cached(&ctx.cache);
    let follower_role = match follower_role {
        Some(x) => x,
        None => {
            return Err(GuildMemberAdditionError::CacheError(
                "Cant't find follower role".to_string(),
            ))
        }
    };

    let success = new_member.add_role(&ctx.http, follower_role.id).await;

    let mut error_reason = "".to_string();
    match success {
        Ok(_) => println!(
            "New member joined {}, giving {} role",
            new_member.display_name(),
            follower_role.name
        ),
        Err(e) => error_reason = e.to_string(),
    }

    if error_reason == "Missing Permissions" {
        return Err(GuildMemberAdditionError::MissingPermissions);
    }

    if error_reason == "Missing Access" {
        return Err(GuildMemberAdditionError::MissingAccess);
    }

    Ok(())
}
