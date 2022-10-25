use serenity::client::Context;
use serenity::model::guild::Member;
use serenity::model::prelude::RoleId;

use crate::redis_client;

pub async fn handle(ctx: Context, new_member: Member) {
    let mut new_member = new_member;

    let mut connection = redis_client::connect();

    give_follower_role(&mut new_member, &mut connection, &ctx).await;

    // TODO: Logic for checking for a "log" channel ID and logging this
    // event to that channel.
}

async fn give_follower_role(
    new_member: &mut Member,
    connection: &mut redis::Connection,
    ctx: &Context,
) {
    let follower_role = match redis_client::get_follower_role(connection) {
        Ok(role_id_wrapped) => match role_id_wrapped {
            Some(role_id) => role_id,
            None => panic!("Redis: Follower role resolved to none"),
        },
        Err(e) => {
            panic!("{}", e);
        }
    };

    let follower_role = RoleId(follower_role.parse().expect("Follower role ID invalid"))
        .to_role_cached(&ctx.cache)
        .expect("Cant't find follower role");

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
        println!(
            "Give the bot an Admin role and move it above the {} role",
            follower_role.name
        );
    }
    if error_reason == "Missing Access" {
        println!("Bot joined without Admin privileges. ");
    }
}
