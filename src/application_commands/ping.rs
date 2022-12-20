use crate::{
    application_commands::errors::CommandError, events::application_command::CommandDataBundle,
};
use serenity::builder::CreateApplicationCommand;

pub async fn execute(data_bundle: &mut CommandDataBundle) -> Result<String, CommandError> {
    data_bundle.set_ephemeral(true);

    Ok("Hey, I'm alive!".to_string())
}

pub fn setup(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("Check if bot is online")
}
