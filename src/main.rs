use std::env;

use mythicc_bot::events;
use mythicc_bot::redis_client;

use serenity::async_trait;

use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;

use serenity::model::prelude::RoleId;

use serenity::prelude::*;
struct Handler;

const FOLLOWER_ROLE_ID: RoleId = RoleId(888568260151345192);

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        events::application_command::handle(ctx, interaction).await;
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let mut new_member = new_member;
        let follower_role = FOLLOWER_ROLE_ID
            .to_role_cached(&ctx.cache)
            .expect("Cant't find follower role");

        println!("{}", follower_role);

        let success = new_member.add_role(&ctx.http, follower_role.id).await;

        println!("{:#?}", success);
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

        // println!("{:#?}", _success);
        // let msg = ChannelId(935647333830520912)
        //     .send_message(&ctx.http, |m| {
        //         m.content(format!("Unable to assign {}", follower_role.name))
        //     })
        //     .await;
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
