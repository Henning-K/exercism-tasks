// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone)]
pub struct Robot {
    orientation: Direction,
    x: isize,
    y: isize,
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {
            x: x,
            y: y,
            orientation: d,
        }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            orientation: match self.orientation {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            orientation: match self.orientation {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
            ..self
        }
    }

    pub fn advance(self) -> Self {
        Robot {
            x: match self.orientation {
                Direction::East => self.x + 1,
                Direction::West => self.x - 1,
                _ => self.x,
            },
            y: match self.orientation {
                Direction::North => self.y + 1,
                Direction::South => self.y - 1,
                _ => self.y,
            },
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut temp = Robot { ..self };
        for c in instructions.chars() {
            temp = match c {
                'L' => temp.clone().turn_left(),
                'R' => temp.clone().turn_right(),
                'A' => temp.clone().advance(),
                _ => temp.clone(),
            };
        }
        temp
    }

    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.orientation
    }
}
