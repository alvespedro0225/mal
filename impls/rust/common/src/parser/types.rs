use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum MalType {
    List { tokens: Vec<MalType> },
    Number(i128),
    Symbol(Box<str>),
    Vector { tokens: Vec<MalType> },
    Bool(bool),
    Nil,
    HashMap { tokens: Vec<MalType> },
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
            Self::List { tokens: _ } => "list",
            Self::Number(_) => "number",
            Self::Symbol(_) => "symbol",
            Self::Vector { tokens: _ } => "vector",
            Self::Bool(_) => "bool",
            Self::Nil => "nil",
            Self::HashMap { tokens: _ } => "hashmap",
        };

        write!(f, "{variant}")
    }
}
