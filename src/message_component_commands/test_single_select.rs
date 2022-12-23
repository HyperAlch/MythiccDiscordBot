use crate::{
    events::message_component::{MessageComponentDataBundle, MessageComponentResponseBundle},
    message_component_commands::errors::ComponentInteractionError,
};

pub async fn execute(
    data_bundle: &mut MessageComponentDataBundle,
) -> Result<MessageComponentResponseBundle, ComponentInteractionError> {
    data_bundle.set_ephemeral(true);

    match data_bundle.interaction.data.values.get(0) {
        Some(value) => Ok(MessageComponentResponseBundle {
            message: Some(value.to_owned()),
            modal: None,
        }),
        None => Err(ComponentInteractionError::UnresolvedData(
            "test-single-select".to_string(),
            "Selected value".to_string(),
        )),
    }
}
