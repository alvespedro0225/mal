use std::{panic, sync::LazyLock};

use regex::Regex;

use crate::readline::reader::Reader;

mod reader;

fn read(arg: &str) -> &str {
    arg
}

fn eval(arg: &str) -> &str {
    arg
}

fn print(arg: &str) -> &str {
    arg
}

pub fn rep(arg: &str) {
    let res = read(arg);
    let res = eval(res);
    print(res);
}

fn tokenize(string: &str) -> Box<[&str]> {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#)
            .unwrap()
    });
    let mut matches = Vec::new();
    let captured = RE.captures_iter(string);

    for capture in captured {
        let (_, substring): (&str, [&str; 1]) = capture.extract();
        matches.push(substring[0])
    }

    matches.into()
}

pub fn read_string(string: &str) {
    let tokens = tokenize(string);
    let reader = Reader::new(tokens);
}
