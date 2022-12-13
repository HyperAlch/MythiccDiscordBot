use crate::redis_client::{self, check_admin};
use crate::slash_commands as sc;
use crate::slash_commands::errors::CommandError;
use crate::utils::logging::log_error;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;

pub async fn handle(ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(command) = interaction {
        let command_caller = command.member.as_ref().unwrap().user.id;
        let mut connection = redis_client::connect();
        let mut is_ephemeral: bool = true;

        match check_admin(&mut connection, command_caller.to_string()) {
            Ok(is_admin) => {
                if !is_admin {
                    create_response(
                        ctx,
                        command,
                        "You are not an admin".to_string(),
                        is_ephemeral,
                    )
                    .await;
                    return;
                }
            }
            Err(error) => {
                log_error(&error);
                create_response(
                    ctx,
                    command,
                    "check_admin() failed".to_string(),
                    is_ephemeral,
                )
                .await;
                return;
            }
        }

        let content = match command.data.name.as_str() {
            "prune" => sc::prune::execute(ctx.http.to_owned(), command.channel_id, &command).await,
            "ping" => sc::ping::execute(&mut is_ephemeral),
            "list-admins" => sc::list_admins::execute(&ctx).await,
            "get-user-id" => sc::get_user_id::execute(&command),
            "add-admin" => sc::add_admin::execute(&command),
            "remove-admin" => sc::remove_admin::execute(&command),
            "test-button-message" => {
                sc::test_button_message::execute(&mut is_ephemeral, &ctx).await
            }
            "test-single-select" => sc::test_button_message::execute(&mut is_ephemeral, &ctx).await,
            "test-log-channel" => sc::test_log_channel::execute(&mut is_ephemeral, &ctx).await,
            "test-give-roles" => sc::test_give_roles::execute(&command, &ctx).await,

            _ => Ok("Command removed or not implemented".to_string()),
        };

        if let Ok(content) = content {
            create_response(ctx, command, content, is_ephemeral).await;
        } else if let Err(error) = content {
            log_error(&error);
            let content = match_error(error);
            create_response(ctx, command, content, is_ephemeral).await;
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

fn match_error(error: CommandError) -> String {
    match error {
        CommandError::ArgumentMissing(_) => "Missing an option...".to_string(),
        CommandError::RedisError(content) => content,
        CommandError::Other(content) => content,
        CommandError::UnresolvedData(_, content) => content,
    }
}
