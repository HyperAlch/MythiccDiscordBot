use serenity::model::prelude::{interaction::InteractionResponseType, RoleId};

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
                "game_remove".to_string(),
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
        .filter(|r| existing_user_roles.contains(&RoleId(**r)))
        .collect();

    let mut select_options: Vec<(String, String)> = vec![];

    for game in games {
        let role = match RoleId(*game).to_role_cached(&ctx.cache) {
            Some(r) => r,
            None => {
                return Err(ComponentInteractionError::Other(format!(
                    "role {} not cached!",
                    *game
                )))
            }
        };

        let role_id = *game;
        let role_id = role_id.to_string();
        select_options.push((role.name, role_id))
    }

    if select_options.is_empty() {
        select_options.push((
            "You currently have no game roles to remove".to_string(),
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
                                    menu.custom_id("game-remove-reply");
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
