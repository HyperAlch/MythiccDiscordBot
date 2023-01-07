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

    let mut select_options: Vec<(String, String)> = vec![];

    for game in games {
        let role = match game.parse::<u64>() {
            Ok(x) => x,
            Err(error) => return Err(ComponentInteractionError::Other(error.to_string())),
        };

        let role = match RoleId(role).to_role_cached(&ctx.cache) {
            Some(r) => r,
            None => {
                return Err(ComponentInteractionError::Other(format!(
                    "role {} not cached!",
                    role
                )))
            }
        };

        select_options.push((role.name, game))
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
