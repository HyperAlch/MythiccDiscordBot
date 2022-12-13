use crate::redis_client::{self, check_admin};
use crate::slash_commands as sc;
use crate::slash_commands::errors::CommandError;
use crate::utils::logging::log_error;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::prelude::*;

pub async fn handle(ctx: Context, application_command_interaction: ApplicationCommandInteraction) {
    let mut is_ephemeral: bool = true;

    let command_caller = match application_command_interaction.member.as_ref() {
        Some(member) => member.user.id,
        None => {
            log_error(&CommandError::UnresolvedData(
                "application_command root handle".to_string(),
                "Could not resolve command caller".to_string(),
            ));
            create_response(
                ctx,
                application_command_interaction,
                "Could not resolve command caller".to_string(),
                is_ephemeral,
            )
            .await;
            return;
        }
    };

    let mut connection = redis_client::connect();

    match check_admin(&mut connection, command_caller.to_string()) {
        Ok(is_admin) => {
            if !is_admin {
                create_response(
                    ctx,
                    application_command_interaction,
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
                application_command_interaction,
                "check_admin() failed".to_string(),
                is_ephemeral,
            )
            .await;
            return;
        }
    }

    let content = match application_command_interaction.data.name.as_str() {
        "prune" => {
            sc::prune::execute(
                ctx.http.to_owned(),
                application_command_interaction.channel_id,
                &application_command_interaction,
            )
            .await
        }
        "ping" => sc::ping::execute(&mut is_ephemeral),
        "list-admins" => sc::list_admins::execute(&ctx).await,
        "get-user-id" => sc::get_user_id::execute(&application_command_interaction),
        "add-admin" => sc::add_admin::execute(&application_command_interaction),
        "remove-admin" => sc::remove_admin::execute(&application_command_interaction),
        "test-button-message" => sc::test_button_message::execute(&mut is_ephemeral, &ctx).await,
        "test-single-select" => sc::test_button_message::execute(&mut is_ephemeral, &ctx).await,
        "test-log-channel" => sc::test_log_channel::execute(&mut is_ephemeral, &ctx).await,
        "test-give-roles" => {
            sc::test_give_roles::execute(&application_command_interaction, &ctx).await
        }

        _ => Ok("Command removed or not implemented".to_string()),
    };

    if let Ok(content) = content {
        create_response(ctx, application_command_interaction, content, is_ephemeral).await;
    } else if let Err(error) = content {
        log_error(&error);
        let content = match_error(error);
        create_response(ctx, application_command_interaction, content, is_ephemeral).await;
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
