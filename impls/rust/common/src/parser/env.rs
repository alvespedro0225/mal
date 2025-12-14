use std::{cell::RefCell, collections::HashMap};

use crate::parser::{self, ENV, errors::ReplError, types::MalType};
pub type Symbols = HashMap<Box<str>, MalType>;

#[derive(Debug)]
pub struct Env {
    envs: RefCell<Vec<Symbols>>,
}

impl Env {
    pub fn new() -> Self {
        let mut map: Symbols = HashMap::new();
        map.insert("+".into(), MalType::Function(add));
        map.insert("-".into(), MalType::Function(sub));
        map.insert("*".into(), MalType::Function(mult));
        map.insert("/".into(), MalType::Function(div));
        #[cfg(debug_assertions)]
        map.insert("DEBUG-EVAL".into(), MalType::Bool(true));
        Env {
            envs: RefCell::new(vec![map]),
        }
    }

    pub fn set(&self, key: Box<str>, value: MalType) {
        self.envs
            .borrow_mut()
            .last_mut()
            .unwrap()
            .insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<MalType> {
        for map in self.envs.borrow().iter().rev() {
            let value = map.get(key);
            if value.is_some() {
                return value.cloned();
            }
        }
        None
    }

    pub fn new_env(&self) {
        self.envs.borrow_mut().push(HashMap::new())
    }

    pub fn pop_env(&self) {
        self.envs.borrow_mut().pop();
    }
}

fn add(args: MalType) -> Result<MalType, ReplError> {
    fn add_list(list: Vec<MalType>) -> Result<MalType, ReplError> {
        if list.is_empty() {
            return Err(ReplError::Arguments("+".into()));
        }
        let mut accumulator = 0;

        for token in list {
            match token {
                MalType::Number(num) => accumulator += num,
                MalType::List { tokens } => {
                    let res = parser::eval(MalType::List { tokens })?;
                    if let MalType::Number(num) = res {
                        accumulator += num;
                    }
                }
                MalType::Symbol(symbol) => {
                    if let Some(value) = get_from_env(&symbol) {
                        match value {
                            MalType::Number(num) => accumulator += num,
                            other => {
                                return Err(ReplError::Type {
                                    expected: "number".into(),
                                    received: other.to_string().into(),
                                });
                            }
                        }
                    } else {
                        return Err(ReplError::UnknownSymbol(symbol));
                    }
                }
                other => {
                    return Err(ReplError::Type {
                        expected: "list or number".into(),
                        received: other.to_string().into(),
                    });
                }
            }
        }

        Ok(MalType::Number(accumulator))
    }

    match args {
        MalType::Number(num) => Ok(MalType::Number(num)),
        MalType::List { tokens } => Ok(add_list(tokens)?),
        other => Err(ReplError::Type {
            expected: "list or number".into(),
            received: other.to_string().into(),
        }),
    }
}

fn sub(args: MalType) -> Result<MalType, ReplError> {
    fn sub_list(list: Vec<MalType>) -> Result<MalType, ReplError> {
        let mut accumulator = None;

        for token in list {
            match token {
                MalType::Number(num) => {
                    accumulator = accumulator.map_or(Some(num), |a| Some(a - num))
                }
                MalType::List { tokens } => {
                    let res = parser::eval(MalType::List { tokens })?;
                    if let MalType::Number(num) = res {
                        accumulator = accumulator.map_or(Some(num), |a| Some(a - num))
                    }
                }
                MalType::Symbol(symbol) => {
                    if let Some(value) = get_from_env(&symbol) {
                        match value {
                            MalType::Number(num) => {
                                accumulator = accumulator.map_or(Some(num), |a| Some(a - num))
                            }
                            other => {
                                return Err(ReplError::Type {
                                    expected: "number".into(),
                                    received: other.to_string().into(),
                                });
                            }
                        }
                    } else {
                        return Err(ReplError::UnknownSymbol(symbol));
                    }
                }
                other => {
                    return Err(ReplError::Type {
                        expected: "list or number".into(),
                        received: other.to_string().into(),
                    });
                }
            }
        }

        Ok(MalType::Number(accumulator.unwrap()))
    }

    match args {
        MalType::Number(num) => Ok(MalType::Number(num)),
        MalType::List { tokens } => Ok(sub_list(tokens)?),
        other => Err(ReplError::Type {
            expected: "list or number".into(),
            received: other.to_string().into(),
        }),
    }
}

fn mult(args: MalType) -> Result<MalType, ReplError> {
    fn mult_list(list: Vec<MalType>) -> Result<MalType, ReplError> {
        if list.is_empty() {
            return Err(ReplError::Arguments("*".into()));
        }
        let mut accumulator = 1;

        for token in list {
            match token {
                MalType::Number(num) => accumulator *= num,
                MalType::List { tokens } => {
                    let res = parser::eval(MalType::List { tokens })?;
                    if let MalType::Number(num) = res {
                        accumulator *= num;
                    }
                }
                MalType::Symbol(symbol) => {
                    if let Some(value) = get_from_env(&symbol) {
                        match value {
                            MalType::Number(num) => accumulator *= num,
                            other => {
                                return Err(ReplError::Type {
                                    expected: "number".into(),
                                    received: other.to_string().into(),
                                });
                            }
                        }
                    } else {
                        return Err(ReplError::UnknownSymbol(symbol));
                    }
                }
                other => {
                    return Err(ReplError::Type {
                        expected: "list or number".into(),
                        received: other.to_string().into(),
                    });
                }
            }
        }

        Ok(MalType::Number(accumulator))
    }

    match args {
        MalType::Number(num) => Ok(MalType::Number(num)),
        MalType::List { tokens } => Ok(mult_list(tokens)?),
        other => Err(ReplError::Type {
            expected: "list or number".into(),
            received: other.to_string().into(),
        }),
    }
}

fn div(args: MalType) -> Result<MalType, ReplError> {
    fn div_list(list: Vec<MalType>) -> Result<MalType, ReplError> {
        let mut accumulator = None;

        for token in list {
            match token {
                MalType::Number(num) => {
                    if num == 0 {
                        return Err(ReplError::ZeroDivision);
                    } else {
                        accumulator = accumulator.map_or(Some(num), |a| Some(a / num))
                    }
                }
                MalType::List { tokens } => {
                    let res = parser::eval(MalType::List { tokens })?;
                    if let MalType::Number(num) = res {
                        if num == 0 {
                            return Err(ReplError::ZeroDivision);
                        } else {
                            accumulator = accumulator.map_or(Some(num), |a| Some(a / num))
                        }
                    }
                }
                MalType::Symbol(symbol) => {
                    if let Some(value) = get_from_env(&symbol) {
                        match value {
                            MalType::Number(num) => {
                                accumulator = accumulator.map_or(Some(num), |a| Some(a / num))
                            }
                            other => {
                                return Err(ReplError::Type {
                                    expected: "number".into(),
                                    received: other.to_string().into(),
                                });
                            }
                        }
                    } else {
                        return Err(ReplError::UnknownSymbol(symbol));
                    }
                }
                other => {
                    return Err(ReplError::Type {
                        expected: "list or number".into(),
                        received: other.to_string().into(),
                    });
                }
            }
        }

        Ok(MalType::Number(accumulator.unwrap()))
    }

    let args = {
        if let MalType::List { tokens } = &args
            && let Some(token) = tokens.first()
            && let MalType::Symbol(_) = token
        {
            parser::eval(args)?
        } else {
            args
        }
    };

    match args {
        MalType::Number(num) => Ok(MalType::Number(num)),
        MalType::List { tokens } => Ok(div_list(tokens)?),
        other => Err(ReplError::Type {
            expected: "list or number".into(),
            received: other.to_string().into(),
        }),
    }
}

fn get_from_env(key: &str) -> Option<MalType> {
    ENV.with(|env| env.get(key))
}
