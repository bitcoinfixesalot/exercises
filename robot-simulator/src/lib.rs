// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    horizontal: i32,
    vertical: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            horizontal: x,
            vertical: y,
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self {
            horizontal: self.horizontal,
            vertical: self.vertical,
            direction: direction,
        }
    }

    pub fn turn_left(self) -> Self {
        let direction = match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Self {
            horizontal: self.horizontal,
            vertical: self.vertical,
            direction: direction,
        }
    }

    pub fn advance(self) -> Self {
        let (mut x, mut y) = (self.horizontal, self.vertical);
        match self.direction {
            Direction::North => y += 1,
            Direction::East => x += 1,
            Direction::South => y -= 1,
            Direction::West => x -= 1,
        }
        Self {
            horizontal: x,
            vertical: y,
            direction: self.direction,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| match c {
            'R' => robot.turn_right(),
            'A' => robot.advance(),
            'L' => robot.turn_left(),
            _ => panic!("Invalid instructions"),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.horizontal, self.vertical)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
