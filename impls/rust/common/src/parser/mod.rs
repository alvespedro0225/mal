use crate::parser::env::Env;
use crate::parser::errors::ReplError;
use crate::parser::types::{MalCollection, MalType};
use std::mem;

mod env;
pub mod errors;
mod reader;
mod types;

thread_local! {
    pub static ENV: Env = Env::new();
}

pub fn rep(arg: &str) -> Result<Box<str>, ReplError> {
    let res = read(arg);
    let res = eval(res?)?;
    Ok(print(res))
}

fn read(arg: &str) -> Result<MalType, ReplError> {
    reader::read_string(arg)
}

fn eval(arg: MalType) -> Result<MalType, ReplError> {
    ENV.with(|env| {
        if let Some(debug) = env.get("DEBUG-EVAL")
            && !matches!(debug, MalType::Nil)
            && !matches!(debug, MalType::Bool(false))
        {
            println!("EVAL: {}", print_str(arg.clone()))
        }

        match arg {
            MalType::List { mut tokens } => {
                if tokens.is_empty() {
                    return Ok(MalType::List { tokens });
                }
                match mem::take(&mut tokens[0]) {
                    MalType::Symbol(symbol) => match symbol.as_ref() {
                        "def!" => {
                            if tokens.len() < 3 {
                                return Err(ReplError::Arguments("def!".into()));
                            }

                            let key = {
                                match mem::take(&mut tokens[1]) {
                                    MalType::Symbol(key) => key,
                                    other => {
                                        return Err(ReplError::Type {
                                            expected: "symbol".into(),
                                            received: other.to_string().into(),
                                        });
                                    }
                                }
                            };
                            let retval = eval(mem::take(&mut tokens[2]))?;
                            env.set(key, eval(retval.clone())?);
                            Ok(retval)
                        }
                        "let*" => {
                            if tokens.len() < 3 {
                                return Err(ReplError::Arguments("let*".into()));
                            }
                            env.new_env();
                            let mut pairs = {
                                match mem::take(&mut tokens[1]) {
                                    MalType::List { tokens } | MalType::Vector { tokens } => tokens,
                                    other => {
                                        return Err(ReplError::Type {
                                            expected: "list or vector".into(),
                                            received: other.to_string().into(),
                                        });
                                    }
                                }
                            };

                            if pairs.len() & 1 == 1 {
                                return Err(ReplError::OddLet);
                            }

                            for i in (0..pairs.len()).step_by(2) {
                                let key = mem::take(&mut pairs[i]);
                                let value = mem::take(&mut pairs[i + 1]);

                                let key = match key {
                                    MalType::Symbol(symbol) => symbol,
                                    other => {
                                        return Err(ReplError::Type {
                                            expected: "symbol".into(),
                                            received: other.to_string().into(),
                                        });
                                    }
                                };

                                env.set(key, eval(value)?);
                            }

                            let retval = eval(mem::take(&mut tokens[2]));

                            env.pop_env();

                            retval
                        }
                        _ => {
                            let val = match env.get(&symbol) {
                                Some(val) => val,
                                None => return Err(ReplError::UnknownSymbol(symbol)),
                            };
                            match val {
                                MalType::Function(func) => func(MalType::List {
                                    tokens: tokens[1..].into(),
                                }),
                                _ => Ok(val),
                            }
                        }
                    },
                    other => Err(ReplError::Type {
                        expected: "symbol".into(),
                        received: other.to_string().into(),
                    }),
                }
            }

            MalType::Vector { mut tokens } => {
                for token in &mut tokens {
                    let cur = mem::take(token);
                    *token = eval(cur)?
                }

                Ok(MalType::Vector { tokens })
            }

            MalType::HashMap { mut tokens } => {
                for token in &mut tokens {
                    let cur = mem::take(token);
                    *token = eval(cur)?
                }

                Ok(MalType::HashMap { tokens })
            }
            MalType::Symbol(symbol) => match env.get(&symbol) {
                Some(val) => Ok(val),
                None => Err(ReplError::UnknownSymbol(symbol)),
            },
            _ => Ok(arg),
        }
    })
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
        MalType::Symbol(name) | MalType::String(name) => name.into(),
        MalType::Number(num) => num.to_string(),
        MalType::Bool(boolean) => boolean.to_string(),
        MalType::Nil => "nil".to_string(),
        MalType::List { tokens } => make_collection(tokens, '(', ')'),
        MalType::Vector { tokens } => make_collection(tokens, '[', ']'),
        MalType::HashMap { tokens } => make_collection(tokens, '{', '}'),
        MalType::Function(_) => panic!("Function is not a valid repl print type"),
    }
}
