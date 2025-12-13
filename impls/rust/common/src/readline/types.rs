pub enum MalType {
    List { tokens: Vec<MalType> },
    Number(i128),
    Symbol(Box<str>),
    Vector { tokens: Vec<MalType> },
    Bool(bool),
    Nil,
    HashMap { pairs: Vec<MalType> },
}

pub enum MalCollection {
    List,
    Vector,
    HashMap,
}
