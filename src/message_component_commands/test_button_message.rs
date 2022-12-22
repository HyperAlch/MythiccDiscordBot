use crate::{
    events::message_component::MessageComponentDataBundle,
    message_component_commands::errors::ComponentInteractionError,
};

pub async fn execute(
    data_bundle: &mut MessageComponentDataBundle,
) -> Result<String, ComponentInteractionError> {
    data_bundle.set_ephemeral(true);

    Ok("Well hello there!".to_string())
}
