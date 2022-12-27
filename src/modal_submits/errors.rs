use thiserror::Error;

type ModalName = String;
type ErrorMessage = String;

#[derive(Error, Debug)]
pub enum ModalError {
    #[error("{0}: Expected option(s)")]
    ArgumentMissing(ModalName),

    #[error("{0}: {1}")]
    UnresolvedData(ModalName, ErrorMessage),

    #[error("Redis: {0}")]
    RedisError(ErrorMessage),

    #[error("Error: `{0}`")]
    Other(ErrorMessage),
}
