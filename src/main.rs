use std::env;

use mythicc_bot::slash_commands as sc;
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::command::Command;

use serenity::prelude::*;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {}", command.data.name);

            let mut is_ephemeral: bool = true;
            let content = match command.data.name.as_str() {
                "ping" => sc::ping::execute(&mut is_ephemeral),
                "prune" => {
                    sc::prune::execute(ctx.http.to_owned(), command.channel_id, &command).await
                }
                "get-user-id" => sc::get_user_id::execute(&command),
                _ => "Command removed or not implemented".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.ephemeral(is_ephemeral).content(content)
                        })
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| sc::prune::setup(command))
                .create_application_command(|command| sc::get_user_id::setup(command))
            // .create_application_command(|command| {
            //     command
            //         .name("welcome")
            //         .name_localized("de", "begrüßen")
            //         .description("Welcome a user")
            //         .description_localized("de", "Einen Nutzer begrüßen")
            //         .create_option(|option| {
            //             option
            //                 .name("user")
            //                 .name_localized("de", "nutzer")
            //                 .description("The user to welcome")
            //                 .description_localized("de", "Der zu begrüßende Nutzer")
            //                 .kind(CommandOptionType::User)
            //                 .required(true)
            //         })
            //         .create_option(|option| {
            //             option
            //                 .name("message")
            //                 .name_localized("de", "nachricht")
            //                 .description("The message to send")
            //                 .description_localized("de", "Die versendete Nachricht")
            //                 .kind(CommandOptionType::String)
            //                 .required(true)
            //                 .add_string_choice_localized(
            //                     "Welcome to our cool server! Ask me if you need help",
            //                     "pizza",
            //                     [("de", "Willkommen auf unserem coolen Server! Frag mich, falls du Hilfe brauchst")]
            //                 )
            //                 .add_string_choice_localized(
            //                     "Hey, do you want a coffee?",
            //                     "coffee",
            //                     [("de", "Hey, willst du einen Kaffee?")],
            //                 )
            //                 .add_string_choice_localized(
            //                     "Welcome to the club, you're now a good person. Well, I hope.",
            //                     "club",
            //                     [("de", "Willkommen im Club, du bist jetzt ein guter Mensch. Naja, hoffentlich.")],
            //                 )
            //                 .add_string_choice_localized(
            //                     "I hope that you brought a controller to play together!",
            //                     "game",
            //                     [("de", "Ich hoffe du hast einen Controller zum Spielen mitgebracht!")],
            //                 )
            //         })
            // })
            // .create_application_command(|command| {
            //     command
            //         .name("numberinput")
            //         .description("Test command for number input")
            //         .create_option(|option| {
            //             option
            //                 .name("int")
            //                 .description("An integer from 5 to 10")
            //                 .kind(CommandOptionType::Integer)
            //                 .min_int_value(5)
            //                 .max_int_value(10)
            //                 .required(true)
            //         })
            //         .create_option(|option| {
            //             option
            //                 .name("number")
            //                 .description("A float from -3.3 to 234.5")
            //                 .kind(CommandOptionType::Number)
            //                 .min_number_value(-3.3)
            //                 .max_number_value(234.5)
            //                 .required(true)
            //         })
            // })
            // .create_application_command(|command| {
            //     command
            //         .name("attachmentinput")
            //         .description("Test command for attachment input")
            //         .create_option(|option| {
            //             option
            //                 .name("attachment")
            //                 .description("A file")
            //                 .kind(CommandOptionType::Attachment)
            //                 .required(true)
            //         })
            // })
        })
        .await;

        let _global_commands = Command::create_global_application_command(&ctx.http, |command| {
            sc::ping::setup(command)
        })
        .await;

        // Move this to a slash_command::util module
        let verbose_command_registration =
            env::var("VERBOSE_COMMAND_REG").unwrap_or("no_verbose".to_string());

        if verbose_command_registration == "guild".to_string() {
            println!(
                "I now have the following guild slash commands: {:#?}",
                _commands
            );
        }
        if verbose_command_registration == "global".to_string() {
            println!(
                "I now have the following global slash commands: {:#?}",
                _global_commands
            );
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
