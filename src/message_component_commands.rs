use self::errors::ComponentInteractionError;
use crate::events::message_component::MessageComponentDataBundle;

pub mod errors;
pub mod test_single_select;

pub async fn execute_command(
    data_bundle: &mut MessageComponentDataBundle,
) -> Result<String, ComponentInteractionError> {
    let command_id = data_bundle.interaction.data.custom_id.as_str();
    println!("\n[Component Command ID]\n{:#?}", command_id);

    match command_id {
        // Test commands
        "test-single-select" => test_single_select::execute(data_bundle).await,
        // No match
        _ => Ok("Command removed or not implemented".to_string()),
    }
}
