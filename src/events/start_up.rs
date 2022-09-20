use redis::{ErrorKind, RedisError};
use serenity::client::Context;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::permissions::Permissions;
use serenity::model::prelude::{Role, RoleId};

use crate::slash_commands as sc;

use serenity::model::prelude::command::Command;

use std::env;

use crate::redis_client;

pub async fn handle(ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
    let guild_id = GuildId(
        env::var("GUILD_ID")
            .expect("Expected GUILD_ID in environment")
            .parse()
            .expect("GUILD_ID must be an integer"),
    );

    let mut connection = redis_client::connect();

    check_bot_admin_role(&mut connection, &ctx, &guild_id).await;
    register_commands(&ctx, &guild_id).await;
}

async fn register_commands(ctx: &Context, guild_id: &GuildId) {
    let guild_commands = GuildId::set_application_commands(guild_id, &ctx.http, |commands| {
        commands
            .create_application_command(|command| sc::prune::setup(command))
            .create_application_command(|command| sc::get_user_id::setup(command))
    })
    .await;

    let global_command =
        Command::create_global_application_command(&ctx.http, |command| sc::ping::setup(command))
            .await;

    sc::utils::check_command_reg_verbose(guild_commands, global_command);
}

async fn check_bot_admin_role(
    connection: &mut redis::Connection,
    ctx: &Context,
    guild_id: &GuildId,
) {
    // Attempt to query from Redis
    let bot_admin_role = redis_client::get_bot_role(connection);
    match bot_admin_role {
        // If the redis query was successful
        Ok(role_id) => {
            // Get all the guild roles in a hashmap
            let guild_roles = guild_id
                .roles(&ctx.http)
                .await
                .expect("Query to retrieve guild roles failed");

            // Parse the role_id pulled from redis into a u64
            let role_id: u64 = role_id
                .parse()
                .expect("Saved 'bot admin role' in redis cannot be parsed into u64");

            // Create a RoleId using the parsed role_id
            let bot_admin_role = &RoleId(role_id);

            // Attempt to find bot_admin_role inside the hashmap of roles
            match guild_roles.get(bot_admin_role) {
                Some(_) => println!("Bot Admin Role Found: {}", role_id),
                None => create_bot_admin_role(None, &ctx, connection, &guild_id).await,
            };
        }
        // If the redis query fails
        Err(e) => create_bot_admin_role(Some(e), &ctx, connection, &guild_id).await,
    }
}

async fn create_bot_admin_role(
    error: Option<RedisError>,
    ctx: &Context,
    conn: &mut redis::Connection,
    guild_id: &GuildId,
) {
    // If an error is present, only allow execution if it's a type error
    // This code shouldn't run if it's a connection error for example
    // Also, if no error is presented, run the code by default
    let should_continue = match error {
        Some(x) => {
            if let ErrorKind::TypeError = x.kind() {
                true
            } else {
                false
            }
        }
        None => true,
    };
    if should_continue {
        println!("Redis: 'bot admin role' missing or nil");
        println!("Starting Bot Admin Role Process...");

        // Get all the roles in the guild
        let roles_amount = guild_id
            .roles(&ctx.http)
            .await
            .expect("Could not get roles for guild");

        // Find all the administrator roles, and figure out the one lowest on the list
        let mut lowest_admin_position = roles_amount.len();
        for (_, r) in roles_amount {
            if r.permissions.administrator() {
                if r.position < lowest_admin_position as i64 {
                    lowest_admin_position = r.position as usize;
                }
            };
        }

        // But the `MythiccBot` role directly under the lowest Admin role
        let mythicc_role = guild_id
            .create_role(&ctx.http, |r| {
                r.hoist(true)
                    .name("MythiccBot")
                    .permissions(Permissions::ADMINISTRATOR)
                    .position(lowest_admin_position as u8)
            })
            .await
            .unwrap();

        // Save the new `MythiccBot` role id inside redis
        match redis_client::set_bot_role(conn, mythicc_role.id.to_string()) {
            Ok(_) => println!("Bot Admin Role Process Set!"),
            Err(e) => panic!("{}", e),
        }
    }
}
