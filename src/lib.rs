use extendr_api::prelude::*;
use rand::SeedableRng;

mod walk;
use walk::{random_step, RandomWalk, SeededRng};

#[extendr]
impl SeededRng {
    fn new(seed: f64) -> Self {
        SeededRng(rand::rngs::StdRng::seed_from_u64(seed as u64))
    }
}

pub struct Walk(RandomWalk);

#[extendr]
impl Walk {
    fn new() -> Self {
        Walk(RandomWalk::new())
    }

    fn step(&mut self, rng: &mut SeededRng) {
        self.0.step(random_step(&mut rng.0));
    }

    fn length(&self) -> i32 {
        self.0.len() as i32
    }

    fn last(&self) -> Vec<i32> {
        let (x, y) = self.0.last();
        vec![x as i32, y as i32]
    }

    fn path(&self) -> List {
        let points: Vec<Robj> = self
            .0
            .points
            .iter()
            .map(|(x, y)| vec![*x as i32, *y as i32].into_robj())
            .collect();
        List::from_values(points)
    }
}

extendr_module! {
    mod niche;
    impl SeededRng;
    impl Walk;
}
