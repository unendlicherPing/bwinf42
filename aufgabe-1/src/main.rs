trait ArukoneGenerator {
    fn generate(n: usize, seed: usize) -> Self;
}

#[derive(Debug)]
struct Arukone {
    size: usize,
    pairs: usize,
    map: Vec<Vec<usize>>,
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
        f.write_fmt(format_args!("{}\n{}\n", self.size, self.pairs))?;

        for row in self.map.iter() {
            for column in row {
                f.write_fmt(format_args!("{column} "))?
            }

            f.write_str("\n")?
        }

        Ok(())
    }
}

fn main() {
    let n = 30;
    let seed = rand::random();

    println!("Input: n = {n}, seed = {seed}");

    let arukone = Arukone::generate(n, seed);

    println!("Output generated using seed: {seed}: \n{arukone}")
}
