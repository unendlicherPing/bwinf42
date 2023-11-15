use crate::shared::{Orientation, Piece};

type Position = (usize, usize, usize);

#[derive(Clone)]
pub struct Field(Vec<Vec<Vec<usize>>>);

impl Field {
    pub fn generate(size: usize) -> Self {
        let middle = size / 2;
        Field(
            (0..size)
                .map(|x| {
                    (0..size)
                        .map(|y| {
                            (0..size)
                                .map(|z| {
                                    if x == middle && y == middle && z == middle {
                                        1
                                    } else {
                                        0
                                    }
                                })
                                .collect()
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn check_orientation(&self, col: usize, row: usize, cell: usize, piece: &Piece) -> bool {
        for xoffset in 0..piece.dimensions.0 {
            for yoffset in 0..piece.dimensions.1 {
                for zoffset in 0..piece.dimensions.2 {
                    match self
                        .0
                        .get(col + xoffset)
                        .and_then(|col| col.get(row + yoffset))
                        .and_then(|row| row.get(cell + zoffset))
                    {
                        Some(0) => {}
                        _ => return false,
                    };
                }
            }
        }

        true
    }

    fn all_position(&self, mut piece: Piece) -> Vec<(Position, Orientation)> {
        let mut found_positions = Vec::new();

        for col in 0..self.0.len() {
            for row in 0..self.0[col].len() {
                for cell in 0..self.0[col][row].len() {
                    for orientation in Orientation::iterator() {
                        piece.switch_orientation(orientation);

                        if self.check_orientation(col, row, cell, &piece) {
                            found_positions.push(((col, row, cell), piece.orientation))
                        }
                    }
                }
            }
        }

        found_positions
    }

    pub fn fill(&mut self, mut pieces: Vec<Piece>) -> bool {
        let mut piece = match pieces.pop() {
            Some(piece) => piece,
            None => return true,
        };

        for (position, orientation) in self.all_position(piece) {
            let mut field = self.clone();
            piece.switch_orientation(orientation);

            for x in 0..piece.dimensions.0 {
                for y in 0..piece.dimensions.1 {
                    for z in 0..piece.dimensions.2 {
                        field.0[position.0 + x][position.1 + y][position.2 + z] = piece.name;
                    }
                }
            }

            if field.fill(pieces.clone()) {
                self.0 = field.0;
                return true;
            }
        }

        false
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .map(|col| {
                col.iter()
                    .map(|row| {
                        row.iter()
                            .map(|cel| write!(f, "{cel} "))
                            .collect::<std::fmt::Result>()
                            .and(writeln!(f))
                    })
                    .collect::<std::fmt::Result>()
                    .and(writeln!(f))
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::logic::{Field, Orientation};

    #[test]
    fn test_all_positions() {
        let field = Field::generate(5);

        let piece = (1, (5, 2, 5)).into();

        let positions = field.all_position(piece);

        assert_eq!(
            positions,
            [
                ((0, 0, 0), Orientation::Front),
                ((0, 0, 0), Orientation::FrontFlipped),
                ((0, 0, 0), Orientation::Left),
                ((0, 0, 0), Orientation::LeftFlipped),
                ((0, 0, 0), Orientation::Top),
                ((0, 0, 0), Orientation::TopFlipped),
                ((0, 0, 3), Orientation::Top),
                ((0, 0, 3), Orientation::TopFlipped),
                ((0, 3, 0), Orientation::Front),
                ((0, 3, 0), Orientation::Left),
                ((3, 0, 0), Orientation::FrontFlipped),
                ((3, 0, 0), Orientation::LeftFlipped)
            ]
        )
    }
}
