use clap::Parser;
use arukone::ArukoneGenerator;

pub mod arukone;
pub mod args;

fn main() {
    let args = args::Args::parse();

    let n = args.n;
    let seed = match args.seed {
        Some(x) => x,
        None => rand::random()
    };

    let arukone = arukone::Arukone::generate(n, seed);

    println!("Output generated using seed: {seed}:");
    println!("{arukone}")
}
