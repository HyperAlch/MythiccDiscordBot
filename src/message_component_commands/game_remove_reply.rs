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
    let user = match data_bundle.interaction.member.as_mut() {
        Some(u) => u,
        None => {
            return Err(ComponentInteractionError::UnresolvedData(
                "game_remove_rely".to_string(),
                "Interaction caller data missing".to_string(),
            ))
        }
    };
    let mut message = "**You no longer have the following roles**\n\n".to_string();
    let mut removed_roles = false;

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
        .filter(|r| existing_user_roles.contains(&RoleId(**r)) || **r == 0)
        .collect();

    for game in games {
        if *game == 0 {
            return Ok(MessageComponentResponseBundle {
                message: Some("No roles removed".to_string()),
                modal: None,
            });
        }
        let role_name = match RoleId(*game).to_role_cached(&ctx.cache) {
            Some(r) => r.name,
            None => {
                return Err(ComponentInteractionError::Other(format!(
                    "role {} not cached!",
                    game
                )))
            }
        };

        match user.remove_role(&ctx.http, RoleId(*game)).await {
            Ok(_) => {
                removed_roles = true;
                message.push_str(&format!("`{}`    ", role_name));
            }
            Err(error) => return Err(ComponentInteractionError::Other(error.to_string())),
        }
    }

    if removed_roles {
        Ok(MessageComponentResponseBundle {
            message: Some(message),
            modal: None,
        })
    } else {
        Ok(MessageComponentResponseBundle {
            message: Some("You already removed the role(s) selected".to_string()),
            modal: None,
        })
    }
}
