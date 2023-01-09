use self::errors::ComponentInteractionError;
use crate::events::message_component::{
    MessageComponentDataBundle, MessageComponentResponseBundle,
};

pub mod errors;
pub mod game_add;
pub mod game_add_reply;
pub mod game_remove;
pub mod game_remove_reply;
pub mod test_button_message;
pub mod test_modal;
pub mod test_multiple_select;
pub mod test_single_select;

pub async fn execute_command(
    data_bundle: &mut MessageComponentDataBundle,
) -> Result<MessageComponentResponseBundle, ComponentInteractionError> {
    let command_id = data_bundle.interaction.data.custom_id.as_str();

    match command_id {
        // Test commands
        "test-single-select" => test_single_select::execute(data_bundle).await,
        "test-multiple-select" => test_multiple_select::execute(data_bundle).await,
        "test-button-message" => test_button_message::execute(data_bundle).await,
        "test-modal" => test_modal::execute(data_bundle).await,
        "pick-games-remove" => game_remove::execute(data_bundle).await,
        "pick-games-add" => game_add::execute(data_bundle).await,
        "game-remove-reply" => game_remove_reply::execute(data_bundle).await,
        "game-add-reply" => game_add_reply::execute(data_bundle).await,

        // No match
        _ => Ok(MessageComponentResponseBundle {
            message: Some("Message Component response removed or not implemented".to_string()),
            modal: None,
        }),
    }
}
