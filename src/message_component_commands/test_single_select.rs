use crate::{
    events::message_component::MessageComponentDataBundle,
    message_component_commands::errors::ComponentInteractionError,
};

pub async fn execute(
    data_bundle: &mut MessageComponentDataBundle,
) -> Result<String, ComponentInteractionError> {
    data_bundle.set_ephemeral(true);

    match data_bundle.interaction.data.values.get(0) {
        Some(value) => Ok(value.to_owned()),
        None => Err(ComponentInteractionError::UnresolvedData(
            "test-single-select".to_string(),
            "Selected value".to_string(),
        )),
    }
}
