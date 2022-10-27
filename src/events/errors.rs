use thiserror::Error;

type ErrorMessage = String;

#[derive(Error, Debug)]
pub enum GuildMemberAdditionError {
    #[error("Redis: {0}")]
    RedisError(ErrorMessage),

    #[error("Guild Member Addition Error: Invalid Data - `{0}`")]
    InvalidData(ErrorMessage),

    #[error("Guild Member Addition Error: Cache Error - `{0}`")]
    CacheError(ErrorMessage),

    #[error("Guild Member Addition Error: Give the bot an Admin role and move it to the top of your list of roles")]
    MissingPermissions,

    #[error("Guild Member Addition Error: Bot joined without Admin privileges. Please fix this.")]
    MissingAccess,

    #[error("Guild Member Addition Error: `{0}`")]
    Other(ErrorMessage),
}
