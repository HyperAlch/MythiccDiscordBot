use crate::{
    application_commands::errors::CommandError,
    events::application_command::CommandDataBundle,
    redis_client::{self, list_games},
};
use serenity::{builder::CreateApplicationCommand, model::prelude::RoleId};

pub async fn execute(data_bundle: &mut CommandDataBundle) -> Result<String, CommandError> {
    data_bundle.set_ephemeral(true);

    let ctx = &data_bundle.ctx;
    let mut connection = redis_client::connect();
    let games = match list_games(&mut connection) {
        Ok(x) => x,
        Err(error) => return Err(CommandError::RedisError(error.to_string())),
    };
    let mut content = "".to_string();
    for game in games {
        let role_id = match game.parse::<u64>() {
            Ok(x) => x,
            Err(error) => return Err(CommandError::Other(error.to_string())),
        };

        let role = match RoleId(role_id).to_role_cached(&ctx.cache) {
            Some(r) => r,
            None => return Err(CommandError::Other(format!("role {} not cached!", role_id))),
        };
        content.push_str(&format!("{}\n", role.name));
    }

    Ok(content)
}

pub fn setup() -> impl FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    move |command: &mut CreateApplicationCommand| {
        command
            .name("list-games")
            .description("List all supported games")
    }
}
