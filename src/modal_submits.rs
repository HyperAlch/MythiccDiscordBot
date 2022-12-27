use crate::events::modal_submit::ModalDataBundle;

use self::errors::ModalError;

pub mod errors;
pub mod test_modal;

pub async fn process_modal_data(data_bundle: &mut ModalDataBundle) -> Result<String, ModalError> {
    let modal_id = data_bundle.interaction.data.custom_id.as_str();

    match modal_id {
        "test-modal" => test_modal::process(data_bundle).await,
        // No match
        _ => Ok("Modal response removed or not implemented".to_string()),
    }
}
