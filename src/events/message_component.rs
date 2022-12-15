use serenity::{
    model::prelude::interaction::message_component::MessageComponentInteraction, prelude::*,
};

pub async fn handle(_ctx: Context, message_component_interaction: MessageComponentInteraction) {
    println!("{:#?}", message_component_interaction);
    let data = message_component_interaction.data.values.get(0).unwrap();
    println!("\n\n\n[Data]\n{:#?}", data);
}
