use crate::application_commands::errors::CommandError;
use crate::events::application_command::CommandDataBundle;
use crate::redis_client::{self, remove_game};
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;

pub async fn execute(data_bundle: &mut CommandDataBundle) -> Result<String, CommandError> {
    data_bundle.set_ephemeral(true);

    let command_interaction = &data_bundle.interaction;
    let options = command_interaction.data.options.get(0);
    let options = match options {
        Some(x) => x,
        None => return Err(CommandError::ArgumentMissing("Remove Game".to_string())),
    };

    let options = options.resolved.as_ref();
    let options = match options {
        Some(x) => x,
        None => {
            return Err(CommandError::UnresolvedData(
                "Remove Game".to_string(),
                "Expected role object".to_string(),
            ))
        }
    };

    if let CommandDataOptionValue::Role(role) = options {
        let mut connection = redis_client::connect();
        match remove_game(&mut connection, role.id.to_string()) {
            Ok(_) => Ok(format!("{} has been removed from the game list", role.name)),
            Err(_) => Err(CommandError::RedisError("remove_game() failed".to_string())),
        }
    } else {
        Err(CommandError::Other(
            "Please provide a valid role".to_string(),
        ))
    }
}

pub fn setup() -> impl FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    move |command: &mut CreateApplicationCommand| {
        command
            .name("remove-game")
            .description("Remove a game role to the list of games")
            .create_option(|option| {
                option
                    .name("game-role")
                    .description("The role to remove")
                    .kind(CommandOptionType::Role)
                    .required(true)
            })
    }
}
