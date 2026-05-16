use std::env;
use std::io::{self, BufRead, Write};
use steel::steel_vm::engine::Engine;
use steel::steel_vm::register_fn::RegisterFn;

fn is_balanced(s: &str) -> bool {
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escape = false;
    for ch in s.chars() {
        if escape {
            escape = false;
            continue;
        }
        match ch {
            '\\' if in_string => escape = true,
            '"' => in_string = !in_string,
            '(' if !in_string => depth += 1,
            ')' if !in_string => depth -= 1,
            _ => {}
        }
    }
    depth <= 0
}

fn main() {
    let mut vm = Engine::new();

    vm.register_fn("add", |a: f64, b: f64| -> f64 { a + b });

    for path in env::args().skip(1) {
        let src = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("could not read {path}: {e}"));
        if let Err(e) = vm.run(src) {
            eprintln!("error loading {path}: {e}");
        }
    }

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buf = String::new();

    loop {
        if buf.is_empty() {
            print!("> ");
        } else {
            print!("  ");
        }
        stdout.flush().unwrap();

        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap() == 0 {
            break; // EOF
        }

        let trimmed = line.trim();
        if trimmed.is_empty() && buf.is_empty() {
            continue;
        }

        if !buf.is_empty() {
            buf.push('\n');
        }
        buf.push_str(trimmed);

        if is_balanced(&buf) {
            let input = std::mem::take(&mut buf);
            match vm.run(input) {
                Ok(vals) => {
                    for v in vals {
                        println!("{:?}", v);
                    }
                }
                Err(e) => eprintln!("error: {e}"),
            }
        }
    }
}
