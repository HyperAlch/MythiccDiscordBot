use crate::application_commands::errors::CommandError;
use crate::events::application_command::CommandDataBundle;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::RoleId;

pub async fn execute(data_bundle: &mut CommandDataBundle) -> Result<String, CommandError> {
    data_bundle.set_ephemeral(true);

    let command_interaction = &data_bundle.interaction;
    let ctx = &data_bundle.ctx;

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

    if let CommandDataOptionValue::User(user, _partial_member) = options {
        let mut member = command_interaction
            .guild_id
            .unwrap()
            .member(&ctx.http, user)
            .await
            .unwrap();

        member
            .add_roles(
                &ctx.http,
                &[RoleId(934946626932465744), RoleId(934946628631142410)],
            )
            .await
            .expect("Adding roles failed");

        Ok(format!("{}'s has been given test roles", user.tag()))
    } else {
        Err(CommandError::Other(
            "Please provide a valid user".to_string(),
        ))
    }
}

pub fn setup() -> impl FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    move |command: &mut CreateApplicationCommand| {
        command
            .name("test-give-roles")
            .description("Give 2 test roles")
            .create_option(|option| {
                option
                    .name("id")
                    .description("The user to lookup")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
    }
}
