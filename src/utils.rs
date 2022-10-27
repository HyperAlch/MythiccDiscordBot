pub mod logging {
    use std::error::Error;

    pub fn log_error(error: &impl Error) {
        println!("{}", error);
    }
}

pub mod discord_cdn {
    use serenity::model::user::User;

    pub fn get_avatar_url(user: &User) -> String {
        format!(
            "https://cdn.discordapp.com/avatars/{}/{}.png",
            user.id.to_string(),
            user.avatar.as_ref().unwrap()
        )
    }
}
