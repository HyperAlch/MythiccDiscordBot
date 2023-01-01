use crate::events::application_command::CommandDataBundle;

use self::errors::CommandError;
use serenity::builder::CreateApplicationCommands;

pub mod add_admin;
pub mod errors;
pub mod get_user_id;
pub mod list_admins;
pub mod ping;
pub mod prune;
pub mod remove_admin;
pub mod setup_pick_games_modal;
pub mod test_button_message;
pub mod test_give_roles;
pub mod test_log_channel;
pub mod test_modal;
pub mod test_multiple_select;
pub mod test_single_select;
pub mod utils;

pub fn guild_commands_reg(
    commands: &mut CreateApplicationCommands,
) -> &mut CreateApplicationCommands {
    // Test Commands
    commands.create_application_command(test_give_roles::setup());
    commands.create_application_command(test_log_channel::setup());
    commands.create_application_command(test_button_message::setup());
    commands.create_application_command(test_single_select::setup());
    commands.create_application_command(test_multiple_select::setup());
    commands.create_application_command(test_modal::setup());

    // UI Component Commands
    commands.create_application_command(setup_pick_games_modal::setup());

    // Admin Commands
    commands.create_application_command(add_admin::setup());
    commands.create_application_command(list_admins::setup());
    commands.create_application_command(remove_admin::setup());

    // Util Commands
    commands.create_application_command(prune::setup());
    commands.create_application_command(get_user_id::setup())
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

        // UI Component Commands
        "setup-pick-games-modal" => setup_pick_games_modal::execute(data_bundle).await,

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
