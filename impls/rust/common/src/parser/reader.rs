use crate::parser::MalCollection;
use crate::parser::ReplError;
use std::sync::LazyLock;

use regex::Regex;

use crate::parser::types::MalType;

pub struct Reader<'a> {
    tokens: Box<[&'a str]>,
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn peek(&self) -> Option<&str> {
        if self.tokens.len() > self.pos {
            Some(self.tokens[self.pos])
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<&str> {
        if self.tokens.len() > self.pos {
            self.pos += 1;
            Some(self.tokens[self.pos - 1])
        } else {
            None
        }
    }

    pub fn new(tokens: Box<[&str]>) -> Reader<'_> {
        Reader { tokens, pos: 0 }
    }
}

pub fn read_string(string: &str) -> Result<MalType, ReplError> {
    let tokens = tokenize(string);
    let mut reader = Reader::new(tokens);
    read_form(&mut reader)
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
                return Err(ReplError::Unclosed('\"'));
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
                Err(ReplError::Eof) => return Err(ReplError::Arguments("^".into())),
                e => return e,
            };
            let second = match read_form(reader) {
                Ok(first) => first,
                Err(ReplError::Eof) => return Err(ReplError::Arguments("^".into())),
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
            None => return Err(ReplError::Unclosed(end.chars().next().unwrap())),
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
        MalCollection::HashMap => MalType::HashMap { tokens },
    };

    Ok(collection)
}

fn read_atom(reader: &mut Reader) -> MalType {
    let current = reader.next().unwrap();

    if let Ok(num) = current.parse::<i128>() {
        return MalType::Number(num);
    }

    if current.starts_with('\"') {
        let current = current.replace("\\\"", "\"");
        let current = current.replace("\\n", "\n");
        let current = current.replace("\\\\", "\\");
        return MalType::String(current.into());
    }

    match current {
        "false" => MalType::Bool(false),
        "true" => MalType::Bool(true),
        "nil" => MalType::Nil,
        other => MalType::Symbol(other.into()),
    }
}
