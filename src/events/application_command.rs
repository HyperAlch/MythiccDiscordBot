use std::future::Future;

use crate::redis_client::{self, check_admin};
use crate::slash_commands as sc;
use crate::slash_commands::errors::CommandError;
use crate::utils::logging::log_error;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::prelude::*;

pub async fn handle(ctx: Context, application_command_interaction: ApplicationCommandInteraction) {
    let mut data_bundle = CommandDataBundle::new(ctx, application_command_interaction);
    let command_caller = match data_bundle.interaction.member.as_ref() {
        Some(member) => member.user.id,
        None => {
            log_error(&CommandError::UnresolvedData(
                "application_command root handle".to_string(),
                "Could not resolve command caller".to_string(),
            ));
            create_response(
                data_bundle.ctx,
                data_bundle.interaction,
                "Could not resolve command caller".to_string(),
                data_bundle.is_ephemeral,
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
                    data_bundle.ctx,
                    data_bundle.interaction,
                    "You are not an admin".to_string(),
                    data_bundle.is_ephemeral,
                )
                .await;
                return;
            }
        }
        Err(error) => {
            log_error(&error);
            create_response(
                data_bundle.ctx,
                data_bundle.interaction,
                "check_admin() failed".to_string(),
                data_bundle.is_ephemeral,
            )
            .await;
            return;
        }
    }

    // let command_list = CommandList {
    //     commands: vec![TestGiveRoles],
    // };

    let command_instance = CommandInstanceExecuter {
        execute_fn: sc::test_give_roles::execute,
    };

    let content = match data_bundle.interaction.data.name.as_str() {
        "prune" => sc::prune::execute(&mut data_bundle).await,
        "ping" => sc::ping::execute(&mut data_bundle).await,
        "list-admins" => sc::list_admins::execute(&mut data_bundle).await,
        "get-user-id" => sc::get_user_id::execute(&mut data_bundle).await,
        "add-admin" => sc::add_admin::execute(&mut data_bundle).await,
        "remove-admin" => sc::remove_admin::execute(&mut data_bundle).await,
        "test-button-message" => sc::test_button_message::execute(&mut data_bundle).await,
        "test-single-select" => sc::test_single_select::execute(&mut data_bundle).await,
        // "test-log-channel" => sc::test_log_channel::execute(&mut data_bundle).await,
        "test-give-roles" => command_instance.execute(&mut data_bundle).await,
        _ => Ok("Command removed or not implemented".to_string()),
    };

    if let Ok(content) = content {
        create_response(
            data_bundle.ctx,
            data_bundle.interaction,
            content,
            data_bundle.is_ephemeral,
        )
        .await;
    } else if let Err(error) = content {
        log_error(&error);
        let content = match_error(error);
        create_response(
            data_bundle.ctx,
            data_bundle.interaction,
            content,
            data_bundle.is_ephemeral,
        )
        .await;
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

/*
       _____                                          _      _____ _                   _
      / ____|                                        | |    / ____| |                 | |
     | |     ___  _ __ ___  _ __ ___   __ _ _ __   __| |   | (___ | |_ _ __ _   _  ___| |_ _   _ _ __ ___
     | |    / _ \| '_ ` _ \| '_ ` _ \ / _` | '_ \ / _` |    \___ \| __| '__| | | |/ __| __| | | | '__/ _ \
     | |___| (_) | | | | | | | | | | | (_| | | | | (_| |    ____) | |_| |  | |_| | (__| |_| |_| | | |  __/
      \_____\___/|_| |_| |_|_| |_| |_|\__,_|_| |_|\__,_|   |_____/ \__|_|   \__,_|\___|\__|\__,_|_|  \___|
*/

pub struct CommandInstanceSetup {
    pub setup_fn: fn(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand,
}

impl CommandInstanceSetup {
    pub fn setup(self, command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        (self.setup_fn)(command)
    }
}

pub struct CommandInstanceExecuter<'a, F>
where
    F: Future<Output = Result<String, CommandError>>,
{
    pub execute_fn: fn(data_bundle: &'a mut CommandDataBundle) -> F,
}

impl<'a, F> CommandInstanceExecuter<'a, F>
where
    F: Future<Output = Result<String, CommandError>>,
{
    async fn execute(
        &self,
        data_bundle: &'a mut CommandDataBundle,
    ) -> Result<String, CommandError> {
        (self.execute_fn)(data_bundle).await
    }
}

pub struct CommandDataBundle {
    pub ctx: Context,
    pub is_ephemeral: bool,
    pub interaction: ApplicationCommandInteraction,
}

impl CommandDataBundle {
    pub fn new(ctx: Context, interaction: ApplicationCommandInteraction) -> Self {
        Self {
            ctx,
            is_ephemeral: true,
            interaction,
        }
    }

    pub fn set_ephemeral(&mut self, is_ephemeral: bool) {
        self.is_ephemeral = is_ephemeral;
    }
}
