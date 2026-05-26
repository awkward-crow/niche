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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_walk_starts_at_origin() {
        let w = RandomWalk::new();
        assert_eq!(w.last(), (0, 0));
        assert_eq!(w.len(), 1);
    }

    #[test]
    fn step_accumulates() {
        let mut w = RandomWalk::new();
        w.step((1, 0));
        w.step((0, -1));
        assert_eq!(w.last(), (1, -1));
        assert_eq!(w.len(), 3);
    }

    #[test]
    fn path_contains_all_points() {
        let mut w = RandomWalk::new();
        w.step((1, 0));
        w.step((1, 0));
        assert_eq!(w.path(), &[(0, 0), (1, 0), (2, 0)]);
    }

    #[test]
    fn seeded_walk_is_reproducible() {
        use rand::{SeedableRng, rngs::StdRng};
        let mut rng = StdRng::seed_from_u64(42);
        let mut w = RandomWalk::new();
        for _ in 0..20 {
            w.step(random_step(&mut rng));
        }
        let mut rng2 = StdRng::seed_from_u64(42);
        let mut w2 = RandomWalk::new();
        for _ in 0..20 {
            w2.step(random_step(&mut rng2));
        }
        assert_eq!(w.path(), w2.path());
    }
}
