use chrono::Utc;
use serenity::{
    builder::{CreateEmbedAuthor, CreateEmbedFooter},
    model::prelude::{interaction::InteractionResponseType, RoleId},
};

use crate::{
    events::message_component::{MessageComponentDataBundle, MessageComponentResponseBundle},
    message_component_commands::errors::ComponentInteractionError,
    utils::discord_cdn::get_avatar_url,
};
const YELLOW: i32 = 0xFFFF00;

pub async fn execute(
    data_bundle: &mut MessageComponentDataBundle,
) -> Result<MessageComponentResponseBundle, ComponentInteractionError> {
    data_bundle.set_ephemeral(true);

    let ctx = &data_bundle.ctx;

    // Grab the games and the user from the interaction data
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

    // Retreive all the existing roles that the user has
    let existing_user_roles = &user.roles;

    // Convert all games to a vector of u64's
    // Store in new variable game_roles
    let mut game_roles = vec![];
    for game in games {
        let role = match game.parse::<u64>() {
            Ok(x) => x,
            Err(error) => return Err(ComponentInteractionError::Other(error.to_string())),
        };

        game_roles.push(role);
    }

    // Filter out games that the user has NOT already selected
    // Don't filter out 0 `**r == 0`, this is for "You currently have no game roles to remove" option
    let games: Vec<&u64> = game_roles
        .iter()
        .filter(|r| existing_user_roles.contains(&RoleId(**r)) || **r == 0)
        .collect();

    // If games is empty, there is nothing to do. They must already removed all the possiable roles
    if games.is_empty() {
        return Ok(MessageComponentResponseBundle {
            message: Some("You already removed the role(s) selected".to_string()),
            modal: None,
        });
    }

    // Convert the vector of u64's into a vector of RoleId's
    let mut remove_list = vec![];
    for game in games {
        if *game == 0 {
            return Ok(MessageComponentResponseBundle {
                message: Some("No roles removed".to_string()),
                modal: None,
            });
        }

        // Conversion
        remove_list.push(RoleId(*game));
    }

    // Remove all the roles in remove_list
    match user.remove_roles(&ctx.http, &remove_list).await {
        Ok(_) => (),
        Err(error) => return Err(ComponentInteractionError::Other(error.to_string())),
    };

    // Get the display string to put in the embed message ready
    let mut display_roles = String::new();
    for role in remove_list {
        let role = role.to_string();
        display_roles.push_str(&format!("<@&{}> ", role));
    }

    // Reassign user as `interaction.user` instead of `interaction.member`
    let user = &data_bundle.interaction.user;

    // Send the reply to the user
    let success = data_bundle
        .interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m| {
                    m.ephemeral(data_bundle.is_ephemeral);
                    m.embed(|e| {
                        let mut author = CreateEmbedAuthor::default();
                        author.icon_url(get_avatar_url(&user));
                        author.name(user.name.clone());

                        let mut footer = CreateEmbedFooter::default();
                        footer.text(format!("ID: {}", user.id));

                        e.title("Roles Updated")
                            .color(YELLOW)
                            .description("🔄 🔄 🔄")
                            .field("Removed Roles: ", display_roles, true)
                            .timestamp(Utc::now())
                            .set_author(author)
                            .field(
                                "Username",
                                format!("<@{}> - {}#{}", user.id, user.name, user.discriminator),
                                false,
                            )
                            .set_footer(footer)
                    })
                })
        })
        .await;

    // Check if successful
    match success {
        Ok(_) => Ok(MessageComponentResponseBundle {
            message: None,
            modal: None,
        }),
        Err(e) => Err(ComponentInteractionError::Other(e.to_string())),
    }
}
