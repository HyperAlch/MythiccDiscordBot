use serenity::model::prelude::RoleId;

use crate::{
    events::message_component::{MessageComponentDataBundle, MessageComponentResponseBundle},
    message_component_commands::errors::ComponentInteractionError,
};

pub async fn execute(
    data_bundle: &mut MessageComponentDataBundle,
) -> Result<MessageComponentResponseBundle, ComponentInteractionError> {
    data_bundle.set_ephemeral(true);

    let ctx = &data_bundle.ctx;

    let games = &data_bundle.interaction.data.values;
    let user = data_bundle
        .interaction
        .member
        .as_mut()
        .expect("Member data missing");
    let mut message = "**You now have the following roles**\n\n".to_string();
    let mut new_roles = false;

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

    for game in games {
        let role_name = match RoleId(*game).to_role_cached(&ctx.cache) {
            Some(r) => r.name,
            None => {
                return Err(ComponentInteractionError::Other(format!(
                    "role {} not cached!",
                    game
                )))
            }
        };

        match user.add_role(&ctx.http, RoleId(*game)).await {
            Ok(_) => {
                new_roles = true;
                message.push_str(&format!("`{}`    ", role_name));
            }
            Err(error) => return Err(ComponentInteractionError::Other(error.to_string())),
        }
    }

    if new_roles {
        Ok(MessageComponentResponseBundle {
            message: Some(message),
            modal: None,
        })
    } else {
        Ok(MessageComponentResponseBundle {
            message: Some("You already have the role(s) selected".to_string()),
            modal: None,
        })
    }
}
