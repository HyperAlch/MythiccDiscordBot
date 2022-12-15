use crate::events::application_command::CommandDataBundle;
use crate::redis_client::{self, add_admin};
use crate::slash_commands::errors::CommandError;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;

pub async fn execute(data_bundle: &mut CommandDataBundle) -> Result<String, CommandError> {
    data_bundle.set_ephemeral(true);

    let command_interaction = &data_bundle.interaction;
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
        match add_admin(&mut connection, user.id.to_string()) {
            Ok(_) => Ok(format!("{} has been added to the admin list", user.tag())),
            Err(_) => Err(CommandError::RedisError("add_admin() failed".to_string())),
        }
    } else {
        Err(CommandError::Other(
            "Please provide a valid user".to_string(),
        ))
    }
}

pub fn setup(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("add-admin")
        .description("Add user as an admin")
        .create_option(|option| {
            option
                .name("id")
                .description("The user to add")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
