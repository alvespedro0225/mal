use regex::Regex;
use std::sync::LazyLock;

use crate::readline::errors::ReplError;
use crate::readline::reader::Reader;
use crate::readline::types::MalType;

pub mod errors;
mod reader;
mod types;

fn read(arg: &str) -> Result<MalType, ReplError> {
    read_string(arg)
}

fn eval(arg: MalType) -> MalType {
    arg
}

fn print(arg: MalType) -> Box<str> {
    let mut ret = print_str(arg);
    ret.push('\n');
    ret.into()
}

pub fn rep(arg: &str) -> Result<Box<str>, ReplError> {
    let res = read(arg);
    let res = eval(res?);
    Ok(print(res))
}

fn tokenize(string: &str) -> Box<[&str]> {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#)
            .unwrap()
    });
    let mut matches = Vec::new();
    for capture in RE.captures_iter(string) {
        let (_, substring): (&str, [&str; 1]) = capture.extract();
        matches.push(substring[0])
    }

    matches.into()
}

pub fn read_string(string: &str) -> Result<MalType, ReplError> {
    let tokens = tokenize(string);
    let mut reader = Reader::new(tokens);
    read_form(&mut reader)
}

fn read_form(reader: &mut Reader) -> Result<MalType, ReplError> {
    let first = match reader.peek() {
        Some(str) => str.chars().next().unwrap_or_default(),
        None => return Err(ReplError::Eof),
    };

    match first {
        '(' => Ok(read_list(reader)?),
        '\'' => {
            let _ = reader.next();
            Ok(MalType::List {
                types: vec![MalType::Symbol("quote".into()), read_atom(reader)],
            })
        }
        _ => Ok(read_atom(reader)),
    }
}

fn read_list(reader: &mut Reader) -> Result<MalType, ReplError> {
    let mut tokens = Vec::new();
    reader.next();

    loop {
        let cur = match reader.peek() {
            Some(token) => token.to_owned(),
            None => return Err(ReplError::Unclosed),
        };

        if cur == ")" {
            break;
        }

        tokens.push(read_form(reader)?);
    }

    let _ = reader.next();

    Ok(MalType::List { types: tokens })
}

fn read_atom(reader: &mut Reader) -> MalType {
    let current = reader.next().unwrap();
    if let Ok(num) = current.parse::<i128>() {
        MalType::Number(num)
    } else {
        MalType::Symbol(current.into())
    }
}

fn print_str(token: MalType) -> String {
    match token {
        MalType::List { types } => {
            let mut str = Vec::new();
            for tkn in types {
                str.push(print_str(tkn));
            }
            let str = str.join(" ");
            let mut ret = String::with_capacity(str.len() + 2);
            ret.push('(');
            ret.push_str(&str);
            ret.push(')');
            ret
        }
        MalType::Number(num) => num.to_string(),
        MalType::Symbol(symbol) => symbol.into(),
    }
}
