use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::*;

use crate::modal_submits::errors::ModalError;
use crate::modal_submits::process_modal_data;
use crate::utils::logging::log_error;

pub async fn handle(ctx: Context, modal_submit_interaction: ModalSubmitInteraction) {
    let mut data_bundle = ModalDataBundle::new(ctx, modal_submit_interaction);
    // println!("{:#?}", &data_bundle.interaction.data);

    let content = process_modal_data(&mut data_bundle).await;

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
    command: ModalSubmitInteraction,
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
        println!("Cannot respond to modal submition: {}", why);
    }
}

fn match_error(error: ModalError) -> String {
    match error {
        ModalError::ArgumentMissing(_) => "Missing an option...".to_string(),
        ModalError::RedisError(content) => content,
        ModalError::Other(content) => content,
        ModalError::UnresolvedData(_, content) => content,
    }
}

// Data bundling
pub struct ModalDataBundle {
    pub ctx: Context,
    pub is_ephemeral: bool,
    pub interaction: ModalSubmitInteraction,
}

impl ModalDataBundle {
    pub fn new(ctx: Context, interaction: ModalSubmitInteraction) -> Self {
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
