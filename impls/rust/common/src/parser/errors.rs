use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReplError {
    #[error("Unclosed '{0}'.")]
    Unclosed(char),

    #[error("Found EOF.")]
    Eof,

    #[error("Missing parameters for '{0}' operation.")]
    Arguments(Box<str>),

    #[error("Invalid type. Expected {expected}, received {received}.")]
    Type {
        expected: Box<str>,
        received: Box<str>,
    },

    #[error("{0} not found.")]
    UnknownSymbol(Box<str>),

    #[error("Tried to divide by 0.")]
    ZeroDivision,

    #[error("Odd number of arguments passed for let*.")]
    OddLet,
}
