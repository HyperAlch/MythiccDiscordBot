use crate::events::modal_submit::ModalDataBundle;
use serenity::model::prelude::component::ActionRowComponent;

use super::errors::ModalError;

pub async fn process(data_bundle: &mut ModalDataBundle) -> Result<String, ModalError> {
    let components = &data_bundle.interaction.data.components;
    let action_row = components.get(0).unwrap();
    let text_input = match action_row.components.get(0).unwrap() {
        ActionRowComponent::InputText(input_text) => input_text,
        _ => {
            return Err(ModalError::UnresolvedData(
                "test_modal".to_string(),
                "InputText `message` is missing from the modal".to_string(),
            ))
        }
    };

    let custom_id = &text_input.custom_id;
    let value = &text_input.value;

    let output = format!("Modal {}: {}", custom_id, value);
    Ok(output)
}
