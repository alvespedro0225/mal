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
