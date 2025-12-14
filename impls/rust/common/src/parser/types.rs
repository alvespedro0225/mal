use std::fmt::Display;

use crate::parser::errors::ReplError;

#[derive(Clone, Debug, Default)]
pub enum MalType {
    String(Box<str>),
    Number(i128),
    Symbol(Box<str>),
    Bool(bool),
    #[default]
    Nil,
    List {
        tokens: Vec<MalType>,
    },
    Vector {
        tokens: Vec<MalType>,
    },
    HashMap {
        tokens: Vec<MalType>,
    },
    Function(fn(MalType) -> Result<MalType, ReplError>),
}

#[derive(Clone, Debug)]
pub enum MalCollection {
    List,
    Vector,
    HashMap,
}

impl Display for MalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant = match &self {
            Self::String(_) => "string",
            Self::Number(_) => "number",
            Self::Symbol(_) => "symbol",
            Self::Bool(_) => "bool",
            Self::Nil => "nil",
            Self::List { tokens: _ } => "list",
            Self::Vector { tokens: _ } => "vector",
            Self::HashMap { tokens: _ } => "hashmap",
            Self::Function(_) => "function",
        };

        write!(f, "{variant}")
    }
}
