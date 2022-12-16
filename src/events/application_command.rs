use crate::redis_client::{self, check_admin};
use crate::slash_commands::errors::CommandError;
use crate::slash_commands::execute_command;
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

    let content = execute_command(&mut data_bundle).await;

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

// Individual CommandSetup Instances
pub struct CommandInstanceSetup {
    pub setup_fn: fn(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand,
}

impl CommandInstanceSetup {
    pub fn new(
        setup_fn: fn(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand,
    ) -> Self {
        Self { setup_fn }
    }

    pub fn setup(self, command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        (self.setup_fn)(command)
    }
}

// List structure of CommandSetup Instances
pub struct CommandSetupList {
    pub setup_list: Vec<CommandInstanceSetup>,
}

impl CommandSetupList {
    pub fn new() -> Self {
        Self {
            setup_list: Vec::new(),
        }
    }

    pub fn add(&mut self, command_instance_setup: CommandInstanceSetup) {
        self.setup_list.push(command_instance_setup);
    }
}

impl Default for CommandSetupList {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for CommandSetupList {
    type Item = CommandInstanceSetup;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.setup_list.is_empty() {
            Some(self.setup_list.pop().unwrap())
        } else {
            None
        }
    }
}

// Data bundling for commands
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
