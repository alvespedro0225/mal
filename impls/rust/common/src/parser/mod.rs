use crate::parser::errors::ReplError;
use crate::parser::symbols::{SYMBOL_MAP, init_symbols};
use crate::parser::types::{MalCollection, MalType};
use std::mem;

pub mod errors;
mod reader;
mod symbols;
mod types;

pub fn rep(arg: &str) -> Result<Box<str>, ReplError> {
    init_symbols();
    let res = read(arg);
    let res = eval(res?)?;
    Ok(print(res))
}

fn read(arg: &str) -> Result<MalType, ReplError> {
    reader::read_string(arg)
}

fn eval(arg: MalType) -> Result<MalType, ReplError> {
    match arg {
        MalType::List { mut tokens } => {
            if tokens.is_empty() {
                return Ok(MalType::List { tokens });
            }
            SYMBOL_MAP.with_borrow(|map| match mem::replace(&mut tokens[0], MalType::Nil) {
                MalType::Symbol(symbol) => {
                    let func = match map.get(&symbol) {
                        Some(func) => func,
                        None => return Err(ReplError::UnknownSymbol(symbol)),
                    };

                    func(MalType::List {
                        tokens: tokens[1..].into(),
                    })
                }
                other => Err(ReplError::Type {
                    expected: "symbol".into(),
                    received: other.to_string().into(),
                }),
            })
        }
        MalType::Vector { mut tokens } => {
            for token in &mut tokens {
                if let MalType::List { tokens: list } = token {
                    let cur = mem::take(list);
                    *token = eval(MalType::List { tokens: cur })?
                }
            }

            Ok(MalType::Vector { tokens })
        }
        MalType::HashMap { mut tokens } => {
            for token in &mut tokens {
                if let MalType::List { tokens: list } = token {
                    let cur = mem::take(list);
                    *token = eval(MalType::List { tokens: cur })?
                }
            }

            Ok(MalType::HashMap { tokens })
        }
        _ => Ok(arg),
    }
}

fn print(arg: MalType) -> Box<str> {
    let mut ret = print_str(arg);
    ret.push('\n');
    ret.into()
}

fn print_str(token: MalType) -> String {
    fn make_collection(tokens: Vec<MalType>, start: char, end: char) -> String {
        let mut str = Vec::new();
        for tkn in tokens {
            let stringified = print_str(tkn);
            if !stringified.is_empty() {
                str.push(stringified);
            }
        }
        let str = str.join(" ");
        let mut ret = String::with_capacity(str.len() + 2);
        ret.push(start);
        ret.push_str(&str);
        ret.push(end);
        ret
    }

    match token {
        MalType::List { tokens } => make_collection(tokens, '(', ')'),
        MalType::Number(num) => num.to_string(),
        MalType::Symbol(symbol) => symbol.into(),
        MalType::Vector { tokens } => make_collection(tokens, '[', ']'),
        MalType::Bool(boolean) => boolean.to_string(),
        MalType::Nil => "nil".to_string(),
        MalType::HashMap { tokens } => make_collection(tokens, '{', '}'),
    }
}
