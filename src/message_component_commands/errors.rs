use thiserror::Error;

type MessageComponentCommandName = String;
type ErrorMessage = String;

#[derive(Error, Debug)]
pub enum ComponentInteractionError {
    #[error("{0}: Expected option(s)")]
    ArgumentMissing(MessageComponentCommandName),

    #[error("{0}: {1}")]
    UnresolvedData(MessageComponentCommandName, ErrorMessage),

    #[error("Redis: {0}")]
    RedisError(ErrorMessage),

    #[error("Cache: {0}")]
    CacheError(ErrorMessage),

    #[error("Error: `{0}`")]
    Other(ErrorMessage),
}
