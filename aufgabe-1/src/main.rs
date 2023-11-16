use clap::Parser;

pub mod args;
pub mod arukone;

fn main() {
    let args = args::Args::parse();

    let size = args.size;
    let mut rng = rand::thread_rng();

    let arukone = arukone::Arukone::generate(size, &mut rng);

    println!("{arukone}")
}
