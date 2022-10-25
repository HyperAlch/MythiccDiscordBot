use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("{0}: Expected options")]
    ArgumentMissing(String),
    #[error("Error: `{0}`")]
    Other(String),
}
