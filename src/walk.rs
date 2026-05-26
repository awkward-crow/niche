use rand::rngs::StdRng;

#[allow(dead_code)]
pub struct SeededRng(pub StdRng);

#[derive(Clone, Debug)]
pub struct RandomWalk {
    pub points: Vec<(i64, i64)>,
}

impl RandomWalk {
    pub fn new() -> Self {
        Self {
            points: vec![(0, 0)],
        }
    }

    pub fn step(&mut self, (dx, dy): (i64, i64)) {
        let (x, y) = self.points.last().unwrap();
        self.points.push((x + dx, y + dy));
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn last(&self) -> (i64, i64) {
        *self.points.last().unwrap()
    }

    pub fn path(&self) -> &[(i64, i64)] {
        &self.points
    }
}

pub fn random_step(rng: &mut impl rand::Rng) -> (i64, i64) {
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    dirs[rng.gen_range(0..4)]
}
