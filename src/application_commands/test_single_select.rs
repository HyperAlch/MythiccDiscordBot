use crate::application_commands::errors::CommandError;
use crate::events::application_command::CommandDataBundle;
use crate::redis_client;
use serenity::builder::CreateApplicationCommand;
use serenity::model::id::ChannelId;

pub async fn execute(data_bundle: &mut CommandDataBundle) -> Result<String, CommandError> {
    data_bundle.set_ephemeral(true);

    let ctx = &data_bundle.ctx;

    let mut conn = redis_client::connect();

    // Query and unpack the log channel id from Redis
    let channel_id = match redis_client::get_major_log_channel(&mut conn) {
        Ok(value) => match value {
            Some(value) => match value.parse::<u64>() {
                Ok(value) => value,
                Err(_) => {
                    return Err(CommandError::Other(
                        "Could not parse log channel id into u64".to_string(),
                    ))
                }
            },
            None => {
                return Err(CommandError::RedisError(
                    "Could not resolve log channel id".to_string(),
                ))
            }
        },
        Err(e) => return Err(CommandError::Other(e.to_string())),
    };

    let channel_id = ChannelId(channel_id);
    let success = channel_id
        .send_message(&ctx, |m| {
            m.content("Please select your favorite animal")
                .components(|c| {
                    c.create_action_row(|row| {
                        // An action row can only contain one select menu!
                        row.create_select_menu(|menu| {
                            menu.custom_id("test-single-select");
                            menu.placeholder("No animal selected");
                            menu.options(|f| {
                                f.create_option(|o| o.label("🐈 meow").value("Cat"));
                                f.create_option(|o| o.label("🐕 woof").value("Dog"));
                                f.create_option(|o| o.label("🐎 neigh").value("Horse"));
                                f.create_option(|o| o.label("🦙 hoooooooonk").value("Alpaca"));
                                f.create_option(|o| o.label("🦀 crab rave").value("Ferris"))
                            })
                        })
                    })
                })
        })
        .await;

    match success {
        Ok(_) => Ok("Message sent to logs...".to_string()),
        Err(e) => Err(CommandError::Other(e.to_string())),
    }
}

pub fn setup() -> impl FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    move |command: &mut CreateApplicationCommand| {
        command
            .name("test-single-select")
            .description("Send an embedded single select to the log channel")
    }
}
