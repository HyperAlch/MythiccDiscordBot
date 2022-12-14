use crate::application_commands::errors::CommandError;
use crate::events::application_command::CommandDataBundle;
use crate::redis_client::{self, get_master_admin, remove_admin};
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;

pub async fn execute(data_bundle: &mut CommandDataBundle) -> Result<String, CommandError> {
    data_bundle.set_ephemeral(true);

    let command_interaction = &data_bundle.interaction;
    let options = command_interaction.data.options.get(0);
    let options = match options {
        Some(x) => x,
        None => return Err(CommandError::ArgumentMissing("Remove Admin".to_string())),
    };

    let options = options.resolved.as_ref();
    let options = match options {
        Some(x) => x,
        None => {
            return Err(CommandError::UnresolvedData(
                "Remove Admin".to_string(),
                "Expected user object".to_string(),
            ))
        }
    };

    if let CommandDataOptionValue::User(user, _member) = options {
        let mut connection = redis_client::connect();
        let user_id = user.id.to_string();
        let master_admin = get_master_admin();
        if user_id == master_admin {
            return Ok("Cannot remove master admin".to_string());
        };
        match remove_admin(&mut connection, user_id) {
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

pub fn setup() -> impl FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    move |command: &mut CreateApplicationCommand| {
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
}
