use std::collections::HashMap;

use rand::Rng;

use self::hamiltonian_cycle::{HamiltonianCycle, Point};

mod hamiltonian_cycle;

#[derive(Debug)]
pub struct Arukone {
    pub size: usize,
    pub pair_count: usize,
    pub map: Vec<Vec<usize>>,
}

impl Arukone {
    fn pick_two(
        hamiltonian_cycle: &mut HamiltonianCycle,
        size: usize,
        rng: &mut rand::rngs::ThreadRng,
    ) -> (Point, Point) {
        let padding = ((size * size) - (size / 2)) / (size / 2 as usize * 2);

        let first = rng.gen_range(0..padding);

        let last = first + rng.gen_range(2..=padding);
        let ret = (hamiltonian_cycle.0[first], hamiltonian_cycle.0[last]);

        dbg!((first, last, &ret));

        for _ in 0..=last {
            hamiltonian_cycle.0.remove(0);
        }

        ret
    }

    pub fn generate(size: usize, rng: &mut rand::rngs::ThreadRng) -> Self {
        let mut hamiltonian_cycle =
            hamiltonian_cycle::HamiltonianCycle::generate(size, rng).unwrap();

        let pair_count = size / 2;
        let pairs: HashMap<Point, usize> = (0..pair_count)
            .map(|_| Arukone::pick_two(&mut hamiltonian_cycle, size, rng))
            .enumerate()
            .flat_map(|(index, points)| {
                HashMap::from([(points.0, index + 1), (points.1, index + 1)])
            })
            .collect();

        let map = (0..size)
            .map(|collum| {
                (0..size)
                    .map(|row| *pairs.get(&(collum as isize, row as isize)).unwrap_or(&0))
                    .collect()
            })
            .collect::<Vec<Vec<usize>>>();

        Self {
            size,
            pair_count,
            map,
        }
    }
}

impl std::fmt::Display for Arukone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n", self.size, self.pair_count)?;

        self.map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|col| write!(f, "{col} "))
                    .collect::<std::fmt::Result>()
                    .and(writeln!(f))
            })
            .collect()
    }
}
