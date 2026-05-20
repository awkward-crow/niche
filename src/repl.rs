// repl.rs

use std::any::{Any, TypeId};
use std::io::{self, BufRead, Write};

use rand::SeedableRng;
use rand::rngs::StdRng;
use steel::rvals::CustomType;
use steel::steel_vm::engine::Engine;
use steel::steel_vm::register_fn::RegisterFn;

use crate::walk::{RandomWalk, SeededRng, random_step};

impl CustomType for SeededRng {
    fn as_any_ref(&self) -> &dyn Any {
        self as &dyn Any
    }
    fn as_any_ref_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
    fn inner_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
    fn display(&self) -> Result<String, std::fmt::Error> {
        Ok("#<seeded-rng>".to_string())
    }
}

impl CustomType for RandomWalk {
    fn as_any_ref(&self) -> &dyn Any {
        self as &dyn Any
    }
    fn as_any_ref_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
    fn inner_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
    fn display(&self) -> Result<String, std::fmt::Error> {
        Ok(format!("#<random-walk {} points>", self.points.len()))
    }
}

pub fn run() {
    let mut vm = Engine::new();

    vm.register_fn("make-walk", RandomWalk::new);
    vm.register_fn("make-rng", |seed: u64| SeededRng(StdRng::seed_from_u64(seed)));

    vm.register_fn("step!", |w: &mut RandomWalk, step: (i64, i64)| {
        w.step(step);
    });

    vm.register_fn("walk-length", |w: &mut RandomWalk| -> usize {
        w.len()
    });

    vm.register_fn("walk-last", |w: &mut RandomWalk| -> (i64, i64) {
        w.last()
    });

    vm.register_fn("walk-path", |w: &mut RandomWalk| -> Vec<(i64, i64)> {
        w.points.clone()
    });

    vm.register_fn("random-step", |rng: &mut SeededRng| -> (i64, i64) {
        random_step(&mut rng.0)
    });

    run_repl(&mut vm);
}

fn run_repl(vm: &mut Engine) {
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
            break;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() && buf.is_empty() {
            continue;
        }

        if let Some(path) = trimmed.strip_prefix(",load").map(str::trim) {
            match std::fs::read_to_string(path) {
                Ok(src) => match vm.run(src) {
                    Ok(vals) => {
                        for v in vals {
                            println!("{v:?}");
                        }
                    }
                    Err(e) => eprintln!("error: {e}"),
                },
                Err(e) => eprintln!("error: {e}"),
            }
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
                        println!("{v:?}");
                    }
                }
                Err(e) => eprintln!("error: {e}"),
            }
        }
    }
}

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


// end
