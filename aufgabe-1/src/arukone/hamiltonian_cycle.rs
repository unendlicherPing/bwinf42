use rand::seq::SliceRandom;

pub type Point = (isize, isize);

#[derive(Debug)]
pub struct HamiltonianCycle(pub Vec<Point>);

impl HamiltonianCycle {
    pub fn generate(square_length: usize, rng: &mut rand::rngs::ThreadRng) -> Option<Self> {
        let mut path = Vec::with_capacity(square_length * square_length);

        let points = (0..square_length)
            .flat_map(|x| {
                (0..square_length)
                    .map(|y| (x as isize, y as isize))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        for point in points {
            path.push(point);

            if let Some(()) =
                HamiltonianCycle::hamiltonian_cycle_until(point, &mut path, square_length, rng)
            {
                return Some(Self(path));
            }

            path.pop();
        }

        None
    }

    fn hamiltonian_cycle_until(
        current: Point,
        path: &mut Vec<Point>,
        square_length: usize,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<()> {
        if path.len() == square_length * square_length {
            return Some(());
        }

        let moves = HamiltonianCycle::shuffle_moves(rng);

        for mov in moves {
            let next: Point = (current.0 + mov.0, current.1 + mov.1);

            if HamiltonianCycle::is_valid_move(next, path, square_length) {
                path.push(next);

                if let Some(()) =
                    HamiltonianCycle::hamiltonian_cycle_until(next, path, square_length, rng)
                {
                    return Some(());
                }

                path.pop();
            }
        }

        None
    }

    fn shuffle_moves(rng: &mut rand::rngs::ThreadRng) -> [(isize, isize); 4] {
        let mut moves = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        moves.shuffle(rng);

        moves
    }

    fn is_valid_move(point: Point, path: &mut Vec<Point>, square_length: usize) -> bool {
        0 <= point.0
            && point.0 < square_length as isize
            && 0 <= point.1
            && point.1 < square_length as isize
            && !path.contains(&point)
    }
}
