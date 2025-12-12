pub enum MalType {
    List { types: Vec<MalType> },
    Number(i128),
    Symbol(Box<str>),
}
