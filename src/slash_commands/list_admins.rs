use crate::{
    redis_client::{self, list_admins},
    slash_commands::errors::CommandError,
};
use serenity::{builder::CreateApplicationCommand, model::prelude::UserId, prelude::Context};

pub async fn execute(ctx: &Context) -> Result<String, CommandError> {
    let mut connection = redis_client::connect();
    let admins = match list_admins(&mut connection) {
        Ok(x) => x,
        Err(error) => return Err(CommandError::RedisError(error.to_string())),
    };
    let mut content = "".to_string();
    for admin in admins {
        let user_id = match admin.parse::<u64>() {
            Ok(x) => x,
            Err(error) => return Err(CommandError::Other(error.to_string())),
        };
        let user = match UserId(user_id).to_user(&ctx).await {
            Ok(x) => x,
            Err(error) => {
                return Err(CommandError::UnresolvedData(
                    "list-admins".to_string(),
                    error.to_string(),
                ))
            }
        };
        content.push_str(&format!("{}#{}\n", user.name, user.discriminator));
    }

    Ok(content)
}

pub fn setup(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("list-admins")
        .description("List all bot admins")
}
