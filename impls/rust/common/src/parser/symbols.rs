use std::{cell::RefCell, collections::HashMap};

use crate::parser::{self, errors::ReplError, types::MalType};
type Symbols = RefCell<HashMap<Box<str>, fn(MalType) -> Result<MalType, ReplError>>>;

thread_local! {
    pub static SYMBOL_MAP: Symbols = RefCell::new(HashMap::new());
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

pub fn init_symbols() {
    SYMBOL_MAP.with_borrow_mut(|map| {
        map.insert("+".into(), add);
        map.insert("-".into(), sub);
        map.insert("*".into(), mult);
        map.insert("/".into(), div)
    });
}
