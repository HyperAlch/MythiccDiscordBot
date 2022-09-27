use serenity::client::Context;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::permissions::Permissions;
use serenity::model::prelude::{Role, RoleId};

use crate::slash_commands as sc;

use serenity::model::prelude::command::Command;
use std::collections::HashMap;
use std::env;

use crate::redis_client;

struct Guild {
    list: HashMap<RoleId, Role>,
    guild_id: GuildId,
}

impl Guild {
    async fn new(guild_id: &GuildId, ctx: &Context) -> Self {
        let guild_roles = guild_id
            .roles(&ctx.http)
            .await
            .expect("Query to retrieve guild roles failed");

        Self {
            list: guild_roles,
            guild_id: *guild_id,
        }
    }
    fn role_exists_from_str(&self, role_id: &String) -> bool {
        let role_id_u64: u64 = role_id
            .parse()
            .expect(&format!("{} cannot be parsed into u64", role_id)[..]);
        self.role_exists_from_u64(&role_id_u64)
    }
    fn role_exists_from_u64(&self, role_id: &u64) -> bool {
        self.role_exists(&RoleId(*role_id))
    }
    fn role_exists(&self, role_id: &RoleId) -> bool {
        match self.list.get(role_id) {
            Some(_) => true,
            None => false,
        }
    }
}

pub async fn handle(ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);

    // Create a GuildId by parsing the Environmental Variable GUILD_ID
    let guild_id = GuildId(
        env::var("GUILD_ID")
            .expect("Expected GUILD_ID in environment")
            .parse()
            .expect("GUILD_ID must be an integer"),
    );

    // Open a connection to Redis
    let mut connection = redis_client::connect();

    let guild = Guild::new(&guild_id, &ctx).await;

    guild.check_bot_admin_role(&mut connection, &ctx).await;
    guild.check_follower_role(&mut connection).await;

    register_commands(&ctx, &guild_id).await;
}

async fn register_commands(ctx: &Context, guild_id: &GuildId) {
    // Register guild commands
    let guild_commands = GuildId::set_application_commands(guild_id, &ctx.http, |commands| {
        commands
            .create_application_command(|command| sc::prune::setup(command))
            .create_application_command(|command| sc::get_user_id::setup(command))
    })
    .await;

    // Register global commands
    let global_command =
        Command::create_global_application_command(&ctx.http, |command| sc::ping::setup(command))
            .await;

    // For debugging
    sc::utils::check_command_reg_verbose(guild_commands, global_command);
}

impl Guild {
    async fn check_bot_admin_role(&self, connection: &mut redis::Connection, ctx: &Context) {
        // Attempt to query from Redis
        let bot_admin_role = redis_client::get_bot_role(connection);
        match bot_admin_role {
            // If the redis query was successful
            Ok(role_id) => match role_id {
                Some(role_id) => {
                    if self.role_exists_from_str(&role_id) {
                        println!("Bot Admin Role Found: {}", role_id);
                    } else {
                        self.create_bot_admin_role(connection, &ctx).await;
                    }
                }
                None => self.create_bot_admin_role(connection, &ctx).await,
            },
            // If the redis query fails
            Err(e) => panic!("{}", e),
        }
    }
    async fn create_bot_admin_role(&self, connection: &mut redis::Connection, ctx: &Context) {
        println!("Starting Bot Admin Role Creation Process...");

        // Get all the roles in the guild
        let roles_amount = self
            .guild_id
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

        // Put the `MythiccBot` role directly under the lowest Admin role
        let mythicc_role = self
            .guild_id
            .create_role(&ctx.http, |r| {
                r.hoist(true)
                    .name("MythiccBot")
                    .permissions(Permissions::ADMINISTRATOR)
                    .position(lowest_admin_position as u8)
            })
            .await
            .unwrap();

        // Save the new `MythiccBot` role id inside redis
        match redis_client::set_bot_role(connection, mythicc_role.id.to_string()) {
            Ok(_) => println!("Bot Admin Role Process Set!"),
            Err(e) => panic!("{}", e),
        }
    }
    async fn check_follower_role(&self, connection: &mut redis::Connection) {
        let follower_id = RoleId(
            env::var("ROLE_FOLLOWER_ID")
                .expect("Expected ROLE_FOLLOWER_ID in environment")
                .parse()
                .expect("ROLE_FOLLOWER_ID must be an integer"),
        );

        if self.role_exists(&follower_id) {
            println!("Follower role found: {}", follower_id.to_string());
        } else {
            panic!("Follower role not in guild, please add one!")
        }

        match redis_client::set_follower_role(connection, follower_id.to_string()) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }
}
