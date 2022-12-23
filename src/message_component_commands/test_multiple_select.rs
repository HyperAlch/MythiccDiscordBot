use crate::{
    events::message_component::{MessageComponentDataBundle, MessageComponentResponseBundle},
    message_component_commands::errors::ComponentInteractionError,
};

pub async fn execute(
    data_bundle: &mut MessageComponentDataBundle,
) -> Result<MessageComponentResponseBundle, ComponentInteractionError> {
    data_bundle.set_ephemeral(true);

    let msg = format!("{:#?}", data_bundle.interaction.data.values);
    println!("{}", msg);
    Ok(MessageComponentResponseBundle {
        message: Some(msg),
        modal: None,
    })
}
