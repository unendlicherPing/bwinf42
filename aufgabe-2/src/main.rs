#[derive(Debug, PartialEq, Eq)]
enum Orientation {
    Horizontal,
    Vertical,
}

type Position = (u32, u32);
type Piece = Position;
type Field = Vec<Vec<u32>>;

fn all_position(piece: Piece, field: &Field) -> Vec<(Position, Orientation)> {
    let mut found_positions = Vec::new();

    for row in 0..field.len() {
        for cell in 0..field[row].len() {
            'check_orientation: for orientation in [Orientation::Horizontal, Orientation::Vertical]
            {
                for xoffset in 0..piece.0 {
                    for yoffset in 0..piece.1 {
                        match field
                            .get(
                                row + if orientation == Orientation::Horizontal {
                                    yoffset
                                } else {
                                    xoffset
                                } as usize,
                            )
                            .and_then(|c| {
                                c.get(
                                    cell + if orientation == Orientation::Horizontal {
                                        xoffset
                                    } else {
                                        yoffset
                                    } as usize,
                                )
                            }) {
                            Some(0) => {}
                            _ => continue 'check_orientation,
                        };
                    }
                }

                found_positions.push(((row as u32, cell as u32), orientation))
            }
        }
    }

    found_positions
}

fn place_pieces(mut pieces: Vec<(u32, Piece)>, field: Field) -> (bool, Vec<Vec<u32>>) {
    let (name, piece) = match pieces.pop() {
        Some(piece) => piece,
        None => return (true, field),
    };

    for position in all_position(piece, &field) {
        let mut field = field.clone();

        for x in 0..piece.1 {
            for y in 0..piece.0 {
                if position.1 == Orientation::Horizontal {
                    field[position.0 .0 as usize + x as usize]
                        [position.0 .1 as usize + y as usize] = name;
                } else {
                    field[position.0 .0 as usize + y as usize]
                        [position.0 .1 as usize + x as usize] = name;
                }
            }
        }

        display_field(&field);

        let result = place_pieces(pieces.clone(), field);

        if result.0 {
            return result;
        }
    }

    (false, field)
}

fn main() {
    let field = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];

    let pieces = vec![(3, (2, 1)), (4, (2, 1)), (2, (3, 1)), (5, (1, 1))];

    display_field(&place_pieces(pieces, field).1);
}

fn display_field(field: &Field) {
    for row in field {
        for cell in row {
            print!("{cell} ")
        }
        println!()
    }
    println!()
}

#[cfg(test)]
mod test {
    use crate::{all_position, Orientation};

    #[test]
    fn test_all_positions() {
        let field = vec![vec![0, 1, 0], vec![0, 0, 0]];

        let piece = (2, 1);

        let positions = all_position(piece, &field);

        assert_eq!(
            positions,
            [
                ((0, 0), Orientation::Vertical),
                ((2, 0), Orientation::Vertical),
                ((0, 1), Orientation::Horizontal),
                ((1, 1), Orientation::Horizontal),
            ]
        )
    }
}
