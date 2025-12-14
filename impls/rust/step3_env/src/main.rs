use std::io::{self, Write};

use common::parser::{self, errors::ReplError};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    loop {
        if stdout.write_all(b"user> ").is_err() {
            continue;
        }

        while stdout.flush().is_err() {}

        if let Ok(read) = stdin.read_line(&mut buffer) {
            if read == 0 {
                let _ = stdout.write_all(b"EOF\n");
                let _ = stdout.flush();
                break;
            }

            match parser::rep(&buffer) {
                Ok(parsed) => {
                    let _ = stdout.write_all(parsed.as_bytes());
                    let _ = stdout.flush();
                }
                Err(e) => match e {
                    ReplError::Unclosed(_) => {
                        let _ = stdout.write_all(b"unbalanced\n");
                        let _ = stdout.flush();
                    }
                    ReplError::Eof => {
                        break;
                    }
                    _ => {
                        let _ = stdout.write_all(e.to_string().as_bytes());
                        let _ = stdout.write_all(b"\n");
                        let _ = stdout.flush();
                    }
                },
            }
            buffer.clear();
        }
    }
}
