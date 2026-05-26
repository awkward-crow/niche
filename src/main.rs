mod walk;

#[cfg(feature = "repl")]
mod repl;

use rand::{SeedableRng, rngs::StdRng};
use walk::{RandomWalk, random_step};

fn main() {
    let mut n: usize = 10;
    let mut seed: u64 = 410037;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let (key, val) = if let Some((k, v)) = arg.split_once('=') {
            (k.to_string(), Some(v.to_string()))
        } else {
            (arg, None)
        };
        let mut val = |name| val.clone().or_else(|| args.next()).unwrap_or_else(|| panic!("{name} requires a value"));
        match key.as_str() {
            #[cfg(feature = "repl")]
            "--repl" => { repl::run(); return; }
            "--n" => n = val("--n").parse().expect("n must be a positive integer"),
            "--seed" => seed = val("--seed").parse().expect("seed must be an integer"),
            other => eprintln!("unknown argument: {other}"),
        }
    }

    let mut rng = StdRng::seed_from_u64(seed);
    let mut w = RandomWalk::new();
    for _ in 0..n {
        w.step(random_step(&mut rng));
    }

    for (x, y) in w.path() {
        println!("{x} {y}");
    }
    let (fx, fy) = w.last();
    eprintln!("after {} steps: ({fx}, {fy})", w.len() - 1);
}
