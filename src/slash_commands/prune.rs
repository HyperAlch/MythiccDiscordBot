use crate::slash_commands::errors::CommandError;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::{futures::StreamExt, http::Http, model::id::ChannelId, model::id::MessageId};
use std::sync::Arc;

pub async fn execute(
    http: Arc<Http>,
    channel_id: ChannelId,
    command_interaction: &ApplicationCommandInteraction,
) -> Result<String, CommandError> {
    let u_amount: usize;

    let amount = command_interaction.data.options.get(0);
    let amount = match amount {
        Some(a) => a,
        None => return Err(CommandError::ArgumentMissing("Prune".to_string())),
    };

    let amount = amount.resolved.as_ref();
    let amount = match amount {
        Some(a) => a,
        None => return Err(CommandError::ArgumentMissing("Prune".to_string())),
    };

    if let CommandDataOptionValue::Integer(set_amount) = amount {
        if *set_amount < 0 {
            u_amount = 0;
        } else {
            u_amount = *set_amount as usize;
        }
    } else {
        return Ok("Please provide a valid amount".to_string());
    }

    let channel_id = ChannelId::from(channel_id);
    let mut messages = channel_id.messages_iter(&http).boxed();
    let mut message_ids: Vec<MessageId> = Vec::new();
    while let Some(message_result) = messages.next().await {
        match message_result {
            Ok(message) => {
                if message_ids.len() < u_amount {
                    message_ids.push(message.id)
                } else {
                    break;
                };
            }
            Err(error) => return Err(CommandError::Other(error.to_string())),
        }
    }

    let _successful = match channel_id
        .delete_messages(&http, message_ids.into_iter())
        .await
    {
        Ok(x) => x,
        Err(e) => return Err(CommandError::Other(e.to_string())),
    };

    Ok("Prune done!".to_string())
}

pub fn setup(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("prune")
        .description("Delete 'x' amount of messages")
        .create_option(|option| {
            option
                .name("amount")
                .description("Amount to delete")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
}
