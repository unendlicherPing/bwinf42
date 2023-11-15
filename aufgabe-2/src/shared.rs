const ERROR_MESSAGE: &'static str =
    "A piece must have the form <name>:<width>:<height>:<depth> for example 2:3:4:5";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Orientation {
    Front,
    FrontFlipped,
    Left,
    LeftFlipped,
    Top,
    TopFlipped,
}

impl Orientation {
    pub fn iterator() -> impl Iterator<Item = Self> {
        [
            Orientation::Front,
            Orientation::FrontFlipped,
            Orientation::Left,
            Orientation::LeftFlipped,
            Orientation::Top,
            Orientation::TopFlipped,
        ]
        .iter()
        .copied()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub name: usize,
    pub dimensions: (usize, usize, usize),
    pub orientation: Orientation,
}

impl Piece {
    pub fn parse(value: &str) -> Result<Piece, &'static str> {
        let mut iter = value.split(":").filter_map(|element| element.parse().ok());

        let name = iter.next().ok_or(ERROR_MESSAGE)?;
        let width = iter.next().ok_or(ERROR_MESSAGE)?;
        let height = iter.next().ok_or(ERROR_MESSAGE)?;
        let depth = iter.next().ok_or(ERROR_MESSAGE)?;

        Ok(Piece {
            name,
            dimensions: (width, height, depth),
            orientation: Orientation::Front,
        })
    }

    /// sorts the list so that the biggest pieces gets placed first
    pub fn optimize_pieces_list(mut list: Vec<Piece>) -> Vec<Piece> {
        list.sort_by(|a, b| {
            (a.dimensions.0 * a.dimensions.1 * a.dimensions.2)
                .partial_cmp(&(b.dimensions.0 * b.dimensions.1 * b.dimensions.2))
                .unwrap()
        });

        list
    }

    fn rotate(&mut self) {
        let size = self.dimensions;

        match self.orientation {
            Orientation::Front => {}
            Orientation::FrontFlipped => {
                self.dimensions.0 = size.1;
                self.dimensions.1 = size.0;
            }
            Orientation::Left => {
                self.dimensions.0 = size.2;
                self.dimensions.2 = size.0;
            }
            Orientation::LeftFlipped => {
                self.dimensions.0 = size.2;
                self.dimensions.1 = size.0;
                self.dimensions.2 = size.1;
            }
            Orientation::Top => {
                self.dimensions.1 = size.2;
                self.dimensions.2 = size.1;
            }
            Orientation::TopFlipped => {
                self.dimensions.0 = size.1;
                self.dimensions.1 = size.2;
                self.dimensions.2 = size.0;
            }
        }

        self.orientation = Orientation::Front
    }

    pub fn switch_orientation(&mut self, orientation: Orientation) {
        self.rotate();

        let size = self.dimensions;

        match orientation {
            Orientation::Front => {}
            Orientation::FrontFlipped => {
                self.dimensions.0 = size.1;
                self.dimensions.1 = size.0;
            }
            Orientation::Left => {
                self.dimensions.0 = size.2;
                self.dimensions.2 = size.0;
            }
            Orientation::LeftFlipped => {
                self.dimensions.0 = size.1;
                self.dimensions.1 = size.2;
                self.dimensions.2 = size.0;
            }
            Orientation::Top => {
                self.dimensions.1 = size.2;
                self.dimensions.2 = size.1;
            }
            Orientation::TopFlipped => {
                self.dimensions.0 = size.2;
                self.dimensions.1 = size.0;
                self.dimensions.2 = size.1;
            }
        }

        self.orientation = orientation;
    }
}

impl Into<Piece> for (usize, (usize, usize, usize)) {
    fn into(self) -> Piece {
        Piece {
            name: self.0,
            dimensions: self.1,
            orientation: Orientation::Front,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Orientation, Piece};

    #[test]
    fn test_switch_orientation() {
        let mut piece: Piece = (3, (2, 3, 4)).into();

        piece.switch_orientation(Orientation::TopFlipped);

        assert_eq!(piece.dimensions, (4, 2, 3));

        piece.switch_orientation(Orientation::LeftFlipped);

        assert_eq!(piece.dimensions, (3, 4, 2));

        piece.switch_orientation(Orientation::Front);

        assert_eq!(piece.dimensions, (2, 3, 4))
    }
}
