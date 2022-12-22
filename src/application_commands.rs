use crate::events::application_command::{
    CommandDataBundle, CommandInstanceSetup, CommandSetupList,
};

use self::errors::CommandError;

pub mod add_admin;
pub mod errors;
pub mod get_user_id;
pub mod list_admins;
pub mod ping;
pub mod prune;
pub mod remove_admin;
pub mod test_button_message;
pub mod test_give_roles;
pub mod test_log_channel;
pub mod test_modal;
pub mod test_multiple_select;
pub mod test_single_select;
pub mod utils;

pub fn guild_commands_reg() -> CommandSetupList {
    let mut guild_commands_list = CommandSetupList::new();

    // Test commands
    guild_commands_list.add(CommandInstanceSetup::new(test_give_roles::setup));
    guild_commands_list.add(CommandInstanceSetup::new(test_log_channel::setup));
    guild_commands_list.add(CommandInstanceSetup::new(test_button_message::setup));
    guild_commands_list.add(CommandInstanceSetup::new(test_single_select::setup));
    guild_commands_list.add(CommandInstanceSetup::new(test_multiple_select::setup));
    guild_commands_list.add(CommandInstanceSetup::new(test_modal::setup));

    // Admin commands
    guild_commands_list.add(CommandInstanceSetup::new(add_admin::setup));
    guild_commands_list.add(CommandInstanceSetup::new(list_admins::setup));
    guild_commands_list.add(CommandInstanceSetup::new(remove_admin::setup));

    // Util commands
    guild_commands_list.add(CommandInstanceSetup::new(prune::setup));
    guild_commands_list.add(CommandInstanceSetup::new(get_user_id::setup));

    guild_commands_list
}

pub async fn execute_command(data_bundle: &mut CommandDataBundle) -> Result<String, CommandError> {
    let command_name = data_bundle.interaction.data.name.as_str();

    match command_name {
        // Test commands
        "test-give-roles" => test_give_roles::execute(data_bundle).await,
        "test-log-channel" => test_log_channel::execute(data_bundle).await,
        "test-button-message" => test_button_message::execute(data_bundle).await,
        "test-single-select" => test_single_select::execute(data_bundle).await,
        "test-multiple-select" => test_multiple_select::execute(data_bundle).await,
        "test-modal" => test_modal::execute(data_bundle).await,
        "ping" => ping::execute(data_bundle).await,

        // Admin commands
        "add-admin" => add_admin::execute(data_bundle).await,
        "list-admins" => list_admins::execute(data_bundle).await,
        "remove-admin" => remove_admin::execute(data_bundle).await,

        // Util commands
        "prune" => prune::execute(data_bundle).await,
        "get-user-id" => get_user_id::execute(data_bundle).await,

        // No match
        _ => Ok("Command removed or not implemented".to_string()),
    }
}
