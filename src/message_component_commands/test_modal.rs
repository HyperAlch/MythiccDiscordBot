use crate::{
    events::message_component::{
        MessageComponentDataBundle, MessageComponentResponseBundle, ModalSettings,
    },
    message_component_commands::errors::ComponentInteractionError,
};

use serenity::{builder::CreateComponents, model::prelude::component::InputTextStyle};

pub async fn execute(
    data_bundle: &mut MessageComponentDataBundle,
) -> Result<MessageComponentResponseBundle, ComponentInteractionError> {
    data_bundle.set_ephemeral(true);

    let mut modal_components = CreateComponents::default();
    modal_components.create_action_row(|row| {
        row.create_input_text(|input| {
            input.custom_id("message");
            input.style(InputTextStyle::Short);
            input.label("Message");
            input.placeholder("Type your message here");
            input.required(true)
        })
    });

    let modal = ModalSettings::new(
        "test-modal".to_string(),
        "Test Modal".to_string(),
        modal_components,
    );

    Ok(MessageComponentResponseBundle {
        message: None,
        modal: Some(modal),
    })
}
