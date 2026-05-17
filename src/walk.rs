#[derive(Clone, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Debug)]
pub struct RandomWalk {
    pub points: Vec<Point>,
}

impl RandomWalk {
    pub fn new() -> Self {
        Self {
            points: vec![Point { x: 0, y: 0 }],
        }
    }

    pub fn step(&mut self, dx: i64, dy: i64) {
        let last = self.points.last().unwrap();
        self.points.push(Point {
            x: last.x + dx,
            y: last.y + dy,
        });
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn last(&self) -> &Point {
        self.points.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::{Any, TypeId};
    use std::io::{self, BufRead, Write};
    use steel::rvals::CustomType;
    use steel::steel_vm::engine::Engine;
    use steel::steel_vm::register_fn::RegisterFn;

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

    #[test]
    #[ignore]
    fn repl() {
        let mut vm = Engine::new();

        vm.register_fn("make-walk", RandomWalk::new);

        // (step! w '(dx dy))
        vm.register_fn("step!", |w: &mut RandomWalk, v: Vec<i64>| {
            if v.len() != 2 {
                eprintln!("step! expects a list of 2 integers");
                return;
            }
            w.step(v[0], v[1]);
        });

        vm.register_fn("walk-length", |w: &mut RandomWalk| -> usize {
            w.len()
        });

        vm.register_fn("walk-last", |w: &mut RandomWalk| -> Vec<i64> {
            let p = w.last();
            vec![p.x, p.y]
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
}
