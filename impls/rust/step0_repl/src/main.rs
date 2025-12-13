use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    loop {
        let _ = stdout.write_all(b"user> ");
        let _ = stdout.flush();
        if let Ok(read) = stdin.read_line(&mut buffer) {
            if read == 0 {
                let _ = stdout.write_all(b"Found EOF. Terminating.\n");
                break;
            }

            let _ = stdout.write_all(buffer.as_bytes());
            let _ = stdout.flush();
            buffer.clear();
        }
    }
}
