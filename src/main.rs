use std::env;
use std::io::{self, BufRead, Write};
use steel::steel_vm::engine::Engine;

fn main() {
    let mut vm = Engine::new();

    for path in env::args().skip(1) {
        let src = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("could not read {path}: {e}"));
        if let Err(e) = vm.run(src) {
            eprintln!("error loading {path}: {e}");
        }
    }
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();

        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap() == 0 {
            break; // EOF
        }

        let input = line.trim().to_owned();
        if input.is_empty() {
            continue;
        }

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
