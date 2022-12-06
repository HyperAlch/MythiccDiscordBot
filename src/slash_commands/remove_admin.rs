use crate::redis_client::{self, remove_admin};
use crate::slash_commands::errors::CommandError;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub fn execute(
    command_interaction: &ApplicationCommandInteraction,
) -> Result<String, CommandError> {
    let options = command_interaction.data.options.get(0);
    let options = match options {
        Some(x) => x,
        None => return Err(CommandError::ArgumentMissing("Get User ID".to_string())),
    };

    let options = options.resolved.as_ref();
    let options = match options {
        Some(x) => x,
        None => {
            return Err(CommandError::UnresolvedData(
                "Get User ID".to_string(),
                "Expected user object".to_string(),
            ))
        }
    };

    if let CommandDataOptionValue::User(user, _member) = options {
        let mut connection = redis_client::connect();
        match remove_admin(&mut connection, user.id.to_string()) {
            Ok(_) => Ok(format!(
                "{} has been removed from the admin list",
                user.tag()
            )),
            Err(_) => Err(CommandError::RedisError(
                "remove_admin() failed".to_string(),
            )),
        }
    } else {
        Err(CommandError::Other(
            "Please provide a valid user".to_string(),
        ))
    }
}

pub fn setup(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("remove-admin")
        .description("Remove user from the admin list")
        .create_option(|option| {
            option
                .name("id")
                .description("The user to remove")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
