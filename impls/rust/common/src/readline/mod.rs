use regex::Regex;
use std::sync::LazyLock;

use crate::readline::errors::ReplError;
use crate::readline::reader::Reader;
use crate::readline::types::{MalCollection, MalType};

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

        if substring[0].starts_with(";") {
            continue;
        }

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
    fn stringfy_symbol(reader: &mut Reader, symbol: &str) -> Result<MalType, ReplError> {
        let _ = reader.next();
        Ok(MalType::List {
            tokens: vec![MalType::Symbol(symbol.into()), read_form(reader)?],
        })
    }

    let first = match reader.peek() {
        Some(str) => str.chars().next().unwrap_or_default(),
        None => return Err(ReplError::Eof),
    };

    match first {
        '(' => Ok(read_list(reader, MalCollection::List)?),
        '[' => Ok(read_list(reader, MalCollection::Vector)?),
        '{' => Ok(read_list(reader, MalCollection::HashMap)?),
        '\"' => {
            if !reader.peek().unwrap().ends_with('\"') {
                return Err(ReplError::Unclosed);
            }
            Ok(read_atom(reader))
        }
        '\'' => stringfy_symbol(reader, "quote"),
        '`' => stringfy_symbol(reader, "quasiquote"),
        '~' => {
            let name = if reader.peek().unwrap() == "~@" {
                "splice-unquote"
            } else {
                "unquote"
            };
            stringfy_symbol(reader, name)
        }
        '@' => stringfy_symbol(reader, "deref"),
        '^' => {
            let _ = reader.next();
            let first = match read_form(reader) {
                Ok(first) => first,
                Err(ReplError::Eof) => return Err(ReplError::Meta),
                e => return e,
            };
            let second = match read_form(reader) {
                Ok(first) => first,
                Err(ReplError::Eof) => return Err(ReplError::Meta),
                e => return e,
            };
            Ok(MalType::List {
                tokens: vec![MalType::Symbol("with-meta".into()), second, first],
            })
        }
        _ => Ok(read_atom(reader)),
    }
}

fn read_list(reader: &mut Reader, mal_type: MalCollection) -> Result<MalType, ReplError> {
    let mut tokens = Vec::new();
    reader.next();
    let end = match mal_type {
        MalCollection::List => ")",
        MalCollection::Vector => "]",
        MalCollection::HashMap => "}",
    };

    loop {
        let cur = match reader.peek() {
            Some(token) => token.to_owned(),
            None => return Err(ReplError::Unclosed),
        };

        if cur == end {
            break;
        }

        tokens.push(read_form(reader)?);
    }

    let _ = reader.next();

    let collection = match mal_type {
        MalCollection::List => MalType::List { tokens },
        MalCollection::Vector => MalType::Vector { tokens },
        MalCollection::HashMap => MalType::HashMap { pairs: tokens },
    };

    Ok(collection)
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
        MalType::Bool(_) => todo!(),
        MalType::Nil => todo!(),
        MalType::HashMap { pairs } => make_collection(pairs, '{', '}'),
    }
}
