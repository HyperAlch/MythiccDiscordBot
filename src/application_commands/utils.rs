use serenity::model::prelude::command::Command;
use serenity::Error;
use std::env;

pub fn check_command_reg_verbose(
    guild_commands: Result<Vec<Command>, Error>,
    global_command: Result<Command, Error>,
) {
    let verbose_command_registration =
        env::var("VERBOSE_COMMAND_REG").unwrap_or_else(|_| "no_verbose".to_string());

    if verbose_command_registration == *"guild" {
        println!(
            "I now have the following guild slash commands: {:#?}",
            guild_commands
        );
    }
    if verbose_command_registration == *"global" {
        println!(
            "I now have the following global slash commands: {:#?}",
            global_command
        );
    }
}
