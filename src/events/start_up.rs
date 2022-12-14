use serenity::client::Context;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::{ChannelId, GuildChannel, Role, RoleId};

use crate::application_commands::{self as sc, guild_commands_reg};

use serenity::model::prelude::command::Command;
use std::collections::HashMap;
use std::env;

use crate::redis_client::{self, check_master_admin, set_guild_id};

struct LocalGuild {
    role_list: HashMap<RoleId, Role>,
    channel_list: HashMap<ChannelId, GuildChannel>,
    // guild_id: GuildId,
}

impl LocalGuild {
    async fn new(guild_id: &GuildId, ctx: &Context) -> Self {
        let guild_roles = guild_id
            .roles(&ctx.http)
            .await
            .expect("Query to retrieve guild roles failed");

        let guild_channels = guild_id
            .channels(&ctx.http)
            .await
            .expect("Query to retrieve guild channels failed");

        Self {
            role_list: guild_roles,
            channel_list: guild_channels,
            // guild_id: *guild_id,
        }
    }

    fn role_exists(&self, role_id: &RoleId) -> bool {
        self.role_list.get(role_id).is_some()
    }

    fn channel_exists(&self, channel_id: &ChannelId) -> bool {
        self.channel_list.get(channel_id).is_some()
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

    check_master_admin(&mut connection)
        .expect("check_master_admin() failed, try checking your Redis Connection");

    set_guild_id(&mut connection, guild_id.to_string()).expect("Redis: Could not set `guild id`");

    let guild = LocalGuild::new(&guild_id, &ctx).await;

    guild.check_follower_role(&mut connection).await;
    guild.check_log_channel(&mut connection).await;

    register_commands(&ctx, &guild_id).await;
}

async fn register_commands(ctx: &Context, guild_id: &GuildId) {
    // Register guild commands
    let guild_commands = GuildId::set_application_commands(guild_id, &ctx.http, move |commands| {
        guild_commands_reg(commands)
    })
    .await;

    // Register global commands
    let global_command =
        Command::create_global_application_command(&ctx.http, sc::ping::setup()).await;

    // For debugging
    sc::utils::check_command_reg_verbose(guild_commands, global_command);
}

impl LocalGuild {
    async fn check_follower_role(&self, connection: &mut redis::Connection) {
        let follower_id = RoleId(
            env::var("ROLE_FOLLOWER_ID")
                .expect("Expected ROLE_FOLLOWER_ID in environment")
                .parse()
                .expect("ROLE_FOLLOWER_ID must be an integer"),
        );

        if self.role_exists(&follower_id) {
            println!("Follower role found: {}", follower_id);
        } else {
            panic!("Follower role not in guild, please add one!");
        }

        match redis_client::set_follower_role(connection, follower_id.to_string()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
    }

    async fn check_log_channel(&self, connection: &mut redis::Connection) {
        let major_log_channel_id = ChannelId(
            env::var("MAJOR_LOG_CHANNEL_ID")
                .expect("Expected MAJOR_LOG_CHANNEL_ID in environment")
                .parse()
                .expect("MAJOR_LOG_CHANNEL_ID must be an integer"),
        );

        let minor_log_channel_id = ChannelId(
            env::var("MINOR_LOG_CHANNEL_ID")
                .expect("Expected MINOR_LOG_CHANNEL_ID in environment")
                .parse()
                .expect("MINOR_LOG_CHANNEL_ID must be an integer"),
        );

        if self.channel_exists(&major_log_channel_id) {
            println!("Major log channel found: {}", major_log_channel_id);
        } else {
            panic!("Major log channel not found, please add one!");
        }

        if self.channel_exists(&minor_log_channel_id) {
            println!("Minor log channel found: {}", minor_log_channel_id);
        } else {
            panic!("Minor log channel not found, please add one!");
        }

        match redis_client::set_major_log_channel(connection, major_log_channel_id.to_string()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }

        match redis_client::set_minor_log_channel(connection, minor_log_channel_id.to_string()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
    }
}
