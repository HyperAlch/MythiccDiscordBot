use crate::{
    application_commands::errors::CommandError,
    events::application_command::CommandDataBundle,
    redis_client::{self, list_games},
};
use serenity::http::client::Http;
use serenity::{builder::CreateApplicationCommand, model::prelude::RoleId};
use std::env;

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
            None => return Err(fix_roles(&mut connection).await),
        };
        content.push_str(&format!("{}\n", role.name));
    }

    Ok(content)
}

async fn fix_roles(connection: &mut redis::Connection) -> CommandError {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let guild_id = match redis_client::get_guild_id(connection) {
        Ok(id) => match id {
            Some(x) => x,
            None => return CommandError::RedisError("`guild id` missing".to_string()),
        },
        Err(error) => return CommandError::RedisError(error.to_string()),
    };

    let guild_id = match guild_id.parse::<u64>() {
        Ok(id) => id,
        Err(_) => return CommandError::Other("`guild id` is invalid".to_string()),
    };

    let api = Http::new(&token);

    let guild_roles = match api.get_guild_roles(guild_id).await {
        Ok(roles) => roles,
        Err(error) => return CommandError::Other(error.to_string()),
    };

    let mut guild_roles_str = Vec::new();

    for role in guild_roles {
        guild_roles_str.push(role.to_string());
    }

    let guild_roles = guild_roles_str;

    let games = match list_games(connection) {
        Ok(x) => x,
        Err(error) => return CommandError::RedisError(error.to_string()),
    };

    let mut missing_roles = Vec::new();

    for game in games {
        let game_formated = format!("<@&{}>", game);
        if !guild_roles.contains(&game_formated) {
            missing_roles.push(game);
        }
    }

    if missing_roles.is_empty() {
        CommandError::CacheError("One of more roles seem to be missing from the cache, please wait a few minutes and try again".to_string())
    } else {
        for role in missing_roles {
            match redis_client::remove_game(connection, role) {
                Ok(_) => (),
                Err(error) => return CommandError::RedisError(error.to_string()),
            };
        }
        CommandError::Other("One or multiple roles in the games list where deleted. This has been fixed, try running the command again!".to_string())
    }
}

pub fn setup() -> impl FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    move |command: &mut CreateApplicationCommand| {
        command
            .name("list-games")
            .description("List all supported games")
    }
}
