#[allow(dead_code)]
mod walk;

use walk::{RandomWalk, random_step};

fn main() {
    let n: usize = std::env::args()
        .nth(1)
        .expect("usage: niche <n>")
        .parse()
        .expect("n must be a positive integer");

    let mut w = RandomWalk::new();
    for _ in 0..n {
        w.step(random_step());
    }

    for (x, y) in &w.points {
        println!("{x} {y}");
    }
}
