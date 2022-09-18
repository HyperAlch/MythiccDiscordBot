use serenity::builder::CreateApplicationCommand;

pub fn execute(is_ephemeral: &mut bool) -> String {
    // ping acts as an example on how to enable / disable ephemeral messages
    // is_ephemeral will already be true, so technically this code does nothing
    // other than serve as an example
    *is_ephemeral = true;
    "Hey, I'm alive!".to_string()
}

pub fn setup(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("Check if bot is online")
}
