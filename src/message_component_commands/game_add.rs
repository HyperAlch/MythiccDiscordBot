use serenity::http::client::Http;
use serenity::model::prelude::{interaction::InteractionResponseType, RoleId};
use std::env;

use crate::redis_client::list_games;
use crate::{
    events::message_component::{MessageComponentDataBundle, MessageComponentResponseBundle},
    message_component_commands::errors::ComponentInteractionError,
    redis_client,
};

pub async fn execute(
    data_bundle: &mut MessageComponentDataBundle,
) -> Result<MessageComponentResponseBundle, ComponentInteractionError> {
    data_bundle.set_ephemeral(true);

    let ctx = &data_bundle.ctx;

    let mut connection = redis_client::connect();
    let games = match redis_client::list_games(&mut connection) {
        Ok(x) => x,
        Err(error) => return Err(ComponentInteractionError::RedisError(error.to_string())),
    };
    let user = match data_bundle.interaction.member.as_mut() {
        Some(u) => u,
        None => {
            return Err(ComponentInteractionError::UnresolvedData(
                "game_add".to_string(),
                "Interaction caller data missing".to_string(),
            ))
        }
    };

    let existing_user_roles = &user.roles;
    let mut game_roles = vec![];

    for game in games {
        let role = match game.parse::<u64>() {
            Ok(x) => x,
            Err(error) => return Err(ComponentInteractionError::Other(error.to_string())),
        };

        game_roles.push(role);
    }

    let games: Vec<&u64> = game_roles
        .iter()
        .filter(|r| !existing_user_roles.contains(&RoleId(**r)))
        .collect();

    let mut select_options: Vec<(String, String)> = vec![];

    for game in games {
        let role = match RoleId(*game).to_role_cached(&ctx.cache) {
            Some(r) => r,
            None => return Err(fix_roles(&mut connection).await),
        };

        let role_id = *game;
        let role_id = role_id.to_string();
        select_options.push((role.name, role_id))
    }

    if select_options.is_empty() {
        select_options.push((
            "You currently have all available game roles".to_string(),
            "0".to_string(),
        ))
    }

    let success = data_bundle
        .interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .content("Please select the games you're interested in")
                        .ephemeral(data_bundle.is_ephemeral)
                        .components(|c| {
                            c.create_action_row(|row| {
                                // An action row can only contain one select menu!
                                row.create_select_menu(|menu| {
                                    menu.custom_id("game-add-reply");
                                    menu.placeholder("No games selected");
                                    menu.max_values(u64::try_from(select_options.len()).unwrap());
                                    menu.options(move |f| {
                                        for option in select_options {
                                            f.create_option(|o| o.label(option.0).value(option.1));
                                        }
                                        f
                                    })
                                })
                            })
                        })
                })
        })
        .await;

    match success {
        Ok(_) => Ok(MessageComponentResponseBundle {
            message: None,
            modal: None,
        }),
        Err(e) => Err(ComponentInteractionError::Other(e.to_string())),
    }
}

async fn fix_roles(connection: &mut redis::Connection) -> ComponentInteractionError {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let guild_id = match redis_client::get_guild_id(connection) {
        Ok(id) => match id {
            Some(x) => x,
            None => return ComponentInteractionError::RedisError("`guild id` missing".to_string()),
        },
        Err(error) => return ComponentInteractionError::RedisError(error.to_string()),
    };

    let guild_id = match guild_id.parse::<u64>() {
        Ok(id) => id,
        Err(_) => return ComponentInteractionError::Other("`guild id` is invalid".to_string()),
    };

    let api = Http::new(&token);

    let guild_roles = match api.get_guild_roles(guild_id).await {
        Ok(roles) => roles,
        Err(error) => return ComponentInteractionError::Other(error.to_string()),
    };

    let mut guild_roles_str = Vec::new();

    for role in guild_roles {
        guild_roles_str.push(role.to_string());
    }

    let guild_roles = guild_roles_str;

    let games = match list_games(connection) {
        Ok(x) => x,
        Err(error) => return ComponentInteractionError::RedisError(error.to_string()),
    };

    let mut missing_roles = Vec::new();

    for game in games {
        let game_formated = format!("<@&{}>", game);
        if !guild_roles.contains(&game_formated) {
            missing_roles.push(game);
        }
    }

    if missing_roles.is_empty() {
        ComponentInteractionError::CacheError("One of more roles seem to be missing from the cache, please wait a few minutes and try again".to_string())
    } else {
        for role in missing_roles {
            match redis_client::remove_game(connection, role) {
                Ok(_) => (),
                Err(error) => return ComponentInteractionError::RedisError(error.to_string()),
            };
        }
        ComponentInteractionError::Other("One or multiple roles in the games list where deleted. This has been fixed, dismiss this message and try again!".to_string())
    }
}
