use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReplError {
    #[error("Unclosed '('")]
    Unclosed,

    #[error("Found EOF")]
    Eof,

    #[error("Missing parameters for '{0}' operation")]
    Arguments(Box<str>),

    #[error("Invalid type. Expected {expected}, received {received}")]
    Type {
        expected: Box<str>,
        received: Box<str>,
    },

    #[error("Unkown symbol: {0}")]
    UnknownSymbol(Box<str>),

    #[error("Tried to divide by 0")]
    ZeroDivision,
}
