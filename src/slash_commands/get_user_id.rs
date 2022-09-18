use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub fn execute(command_interaction: &ApplicationCommandInteraction) -> String {
    let options = command_interaction
        .data
        .options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    if let CommandDataOptionValue::User(user, _member) = options {
        format!("{}'s id is {}", user.tag(), user.id)
    } else {
        "Please provide a valid user".to_string()
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
