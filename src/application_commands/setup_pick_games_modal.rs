use crate::application_commands::errors::CommandError;
use crate::events::application_command::CommandDataBundle;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::component::ButtonStyle;
use serenity::model::prelude::interaction::InteractionResponseType;

pub async fn execute(data_bundle: &mut CommandDataBundle) -> Result<String, CommandError> {
    data_bundle.set_ephemeral(true);

    let ctx = &data_bundle.ctx;

    let success = data_bundle
        .interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content("Pick Your Games").components(|c| {
                        c.create_action_row(|row| {
                            row.create_button(|button| {
                                button
                                    .custom_id("pick-games-add")
                                    .label("Add")
                                    .style(ButtonStyle::Success)
                            });
                            row.create_button(|button| {
                                button
                                    .custom_id("pick-games-remove")
                                    .label("Remove")
                                    .style(ButtonStyle::Danger)
                            })
                        })
                    })
                })
        })
        .await;

    match success {
        Ok(_) => Ok(String::new()),
        Err(e) => Err(CommandError::Other(e.to_string())),
    }
}

pub fn setup() -> impl FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    move |command: &mut CreateApplicationCommand| {
        command
            .name("setup-pick-games-modal")
            .description("Send an embedded button message that triggers a 'Pick Your Games' modal")
    }
}
