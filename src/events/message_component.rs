use serenity::{
    model::prelude::interaction::{
        message_component::MessageComponentInteraction, InteractionResponseType,
    },
    prelude::*,
};

use crate::{
    message_component_commands::{errors::ComponentInteractionError, execute_command},
    utils::logging::log_error,
};

pub async fn handle(ctx: Context, message_component_interaction: MessageComponentInteraction) {
    // println!("{:#?}", message_component_interaction);
    let mut data_bundle = MessageComponentDataBundle::new(ctx, message_component_interaction);

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
    command: MessageComponentInteraction,
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
        println!("Cannot respond to message component: {}", why);
    }
}

fn match_error(error: ComponentInteractionError) -> String {
    match error {
        ComponentInteractionError::ArgumentMissing(_) => "Missing an option...".to_string(),
        ComponentInteractionError::RedisError(content) => content,
        ComponentInteractionError::Other(content) => content,
        ComponentInteractionError::UnresolvedData(_, content) => content,
    }
}

// Data Bundle for Message Components
pub struct MessageComponentDataBundle {
    pub ctx: Context,
    pub is_ephemeral: bool,
    pub interaction: MessageComponentInteraction,
}

impl MessageComponentDataBundle {
    pub fn new(ctx: Context, interaction: MessageComponentInteraction) -> Self {
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
