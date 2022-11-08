use std::env;

use mythicc_bot::events;
use mythicc_bot::redis_client;

use serenity::async_trait;
use serenity::model::guild::Member;

use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;

use serenity::model::prelude::GuildId;
use serenity::model::user::User;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        events::application_command::handle(ctx, interaction).await;
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        events::guild_member_addition::handle(ctx, new_member).await;
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        member_data: Option<Member>,
    ) {
        events::guild_member_removal::handle(ctx, guild_id, user, member_data).await;
    }

    async fn guild_ban_addition(&self, ctx: Context, guild_id: GuildId, banned_user: User) {
        events::guild_ban_addition::handle(ctx, guild_id, banned_user).await;
    }

    async fn guild_ban_removal(&self, ctx: Context, guild_id: GuildId, unbanned_user: User) {
        events::guild_ban_removal::handle(ctx, guild_id, unbanned_user).await;
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        events::voice_state_update::handle(ctx, old, new).await;
    }

    async fn guild_member_update(
        &self,
        _ctx: Context,
        old_if_available: Option<Member>,
        new: Member,
    ) {
        let old_roles_state = old_if_available.unwrap().roles;
        let new_roles_state = new.roles;

        let mut new_roles = Vec::new();
        let mut old_roles = Vec::new();

        for x in new_roles_state.iter() {
            if !old_roles_state.contains(&x) {
                new_roles.push(x);
            }
        }

        for x in old_roles_state.iter() {
            if !new_roles_state.contains(&x) {
                old_roles.push(x);
            }
        }

        // Proof of concept
        println!("Member Role Updated...");
        println!("Give roles {:?}", new_roles);
        println!("Taken roles {:?}", old_roles);
    }

    async fn cache_ready(&self, _ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache Ready...");
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        events::start_up::handle(ctx, ready).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_BANS
        | GatewayIntents::GUILD_PRESENCES
        | GatewayIntents::GUILD_MEMBERS;

    // Build our client.
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Check to make sure we can connect to redis, then drop the connection
    let check_connection = redis_client::connect();
    drop(check_connection);

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
