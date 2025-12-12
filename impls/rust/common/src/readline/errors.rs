use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReplError {
    #[error("Unclosed '('")]
    Unclosed,

    #[error("Found EOF")]
    Eof,
}
