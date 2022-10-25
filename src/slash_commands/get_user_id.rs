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
        Ok(format!("{}'s id is {}", user.tag(), user.id))
    } else {
        Err(CommandError::Other(
            "Please provide a valid user".to_string(),
        ))
    }
}

pub fn setup(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("get-user-id")
        .description("Get a user id")
        .create_option(|option| {
            option
                .name("id")
                .description("The user to lookup")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
