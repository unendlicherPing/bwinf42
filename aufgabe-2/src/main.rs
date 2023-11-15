use clap::Parser;

pub mod args;
pub mod logic;
pub mod shared;

fn main() {
    let (cube_size, pieces) = args::Args::parse().parse_file();

    let pieces = shared::Piece::optimize_pieces_list(pieces);

    let mut cube = logic::Field::generate(cube_size);

    if cube.fill(pieces) {
        println!("{cube}");
    } else {
        eprintln!("no solution found")
    }
}
