use arukone::ArukoneGenerator;
use clap::Parser;

pub mod args;
pub mod arukone;

fn main() {
    let args = args::Args::parse();

    let n = args.n;
    let seed = match args.seed {
        Some(x) => x,
        None => rand::random(),
    };

    let arukone = arukone::Arukone::generate(n, seed);

    println!("Output generated using seed: {seed}:");
    println!("{arukone}")
}
