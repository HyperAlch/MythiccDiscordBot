use crate::slash_commands as sc;
use crate::slash_commands::errors::CommandError;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;

pub async fn handle(ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(command) = interaction {
        println!("Received command interaction: {}", command.data.name);

        let mut is_ephemeral: bool = true;
        let content = match command.data.name.as_str() {
            "prune" => sc::prune::execute(ctx.http.to_owned(), command.channel_id, &command).await,
            "ping" => sc::ping::execute(&mut is_ephemeral),
            "get-user-id" => sc::get_user_id::execute(&command),

            _ => Ok("Command removed or not implemented".to_string()),
        };

        if let Ok(content) = content {
            create_response(ctx, command, content, is_ephemeral).await;
        } else {
            if let Err(error) = content {
                log_error(&error);
                let content = match_error(error);
                create_response(ctx, command, content, is_ephemeral).await;
            }
        }
    }
}

async fn create_response(
    ctx: Context,
    command: ApplicationCommandInteraction,
    content: String,
    is_ephemeral: bool,
) {
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

fn log_error(error: &CommandError) {
    println!("{}", error);
}

fn match_error(error: CommandError) -> String {
    match error {
        CommandError::ArgumentMissing(_) => "Missing an option...".to_string(),
        CommandError::Other(content) => content,
        CommandError::UnresolvedData(_, content) => content,
    }
}
