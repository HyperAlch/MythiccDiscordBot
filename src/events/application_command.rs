use crate::slash_commands as sc;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;

pub async fn handle(ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(command) = interaction {
        println!("Received command interaction: {}", command.data.name);

        let mut is_ephemeral: bool = true;
        let content = match command.data.name.as_str() {
            "ping" => sc::ping::execute(&mut is_ephemeral),
            "prune" => sc::prune::execute(ctx.http.to_owned(), command.channel_id, &command).await,
            "get-user-id" => sc::get_user_id::execute(&command),
            _ => "Command removed or not implemented".to_string(),
        };

        if let Err(why) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.ephemeral(is_ephemeral).content(content)
                    })
            })
            .await
        {
            println!("Cannot respond to slash command: {}", why);
        }
    }
}
