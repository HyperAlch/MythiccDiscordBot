use thiserror::Error;

type SlashCommandName = String;
type ErrorMessage = String;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("{0}: Expected option(s)")]
    ArgumentMissing(SlashCommandName),

    #[error("{0}: {1}")]
    UnresolvedData(SlashCommandName, ErrorMessage),

    #[error("Redis: {0}")]
    RedisError(ErrorMessage),

    #[error("Cache: {0}")]
    CacheError(ErrorMessage),

    #[error("Error: `{0}`")]
    Other(ErrorMessage),
}
