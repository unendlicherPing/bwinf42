pub trait ArukoneGenerator {
    fn generate(n: usize, seed: usize) -> Self;
}

#[derive(Debug)]
pub struct Arukone {
    pub size: usize,
    pub pairs: usize,
    pub map: Vec<Vec<usize>>,
}

impl ArukoneGenerator for Arukone {
    fn generate(n: usize, seed: usize) -> Self {
        let pairs = n / 2;
        let mut map = vec![vec![0; n]; n];
        let mut open: Vec<(usize, usize)> = Vec::new();

        for i in 0..n {
            for j in 0..n {
                open.push((i, j));
            }
        }

        for i in 0..(pairs * 2) {
            let random = open.remove(seed % open.len());
            map[random.0][random.1] = (i % pairs) + 1;

            open = open
                .iter()
                .filter(|(x, y)| *x != random.0 && *y != random.1)
                .map(|x| *x)
                .collect();
        }

        Self {
            size: n,
            pairs,
            map,
        }
    }
}

impl std::fmt::Display for Arukone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n", self.size, self.pairs)?;

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
