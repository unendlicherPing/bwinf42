use clap::Parser;

use crate::shared::Piece;

const FILE_PARSE_ERROR_MESSAGE: &'static str = "failed to parse file";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub file: std::path::PathBuf,
}

impl Args {
    pub fn parse_file(self) -> (usize, Vec<Piece>) {
        let lines = std::fs::read_to_string(self.file)
            .unwrap()
            .lines()
            .map(String::from)
            .collect::<Vec<String>>();

        let cube_size = lines
            .get(0)
            .expect(FILE_PARSE_ERROR_MESSAGE)
            .parse::<usize>()
            .expect(FILE_PARSE_ERROR_MESSAGE);

        let piece_count = lines
            .get(1)
            .expect(FILE_PARSE_ERROR_MESSAGE)
            .parse::<usize>()
            .expect(FILE_PARSE_ERROR_MESSAGE);

        let mut pieces: Vec<Piece> = Vec::with_capacity(piece_count);

        for i in 0..piece_count {
            let numbers = lines
                .get(2 + i)
                .expect(FILE_PARSE_ERROR_MESSAGE)
                .split(" ")
                .map(|number| number.parse())
                .map(|number| number.expect(FILE_PARSE_ERROR_MESSAGE))
                .collect::<Vec<usize>>();

            if numbers.len() != 3 {
                panic!("{FILE_PARSE_ERROR_MESSAGE}");
            }

            pieces.push((2 + i, (numbers[0], numbers[1], numbers[2])).into());
        }

        (cube_size, pieces)
    }
}
