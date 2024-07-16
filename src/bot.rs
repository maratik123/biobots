use crate::Point;
use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    N = 0b000,
    NW = 0b001,
    W = 0b010,
    SW = 0b011,
    S = 0b100,
    SE = 0b101,
    E = 0b110,
    NE = 0b111,
}

impl Direction {
    pub fn change_rotation(direction: Direction, rotate_to: Point<i32>) -> Direction {
        match rotate_to.try_into() {
            Ok(direction_shift) => direction + direction_shift,
            _ => direction,
        }
    }
}

impl From<Direction> for Point<i32> {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::N => Point { x: 0, y: -1 },
            Direction::S => Point { x: 0, y: 1 },
            Direction::E => Point { x: 1, y: 0 },
            Direction::W => Point { x: -1, y: 0 },
            Direction::NE => Point { x: 1, y: -1 },
            Direction::SE => Point { x: 1, y: 1 },
            Direction::SW => Point { x: -1, y: 1 },
            Direction::NW => Point { x: -1, y: -1 },
        }
    }
}

impl TryFrom<Point<i32>> for Direction {
    type Error = ();

    fn try_from(point: Point<i32>) -> Result<Self, Self::Error> {
        Ok(match point {
            Point { x: 0, y } if y < 0 => Direction::N,
            Point { x: 0, y } if y > 0 => Direction::S,
            Point { x, y: 0 } if x > 0 => Direction::E,
            Point { x, y: 0 } if x < 0 => Direction::W,
            Point { x, y } if x > 0 && y < 0 => Direction::NE,
            Point { x, y } if x > 0 && y > 0 => Direction::SE,
            Point { x, y } if x < 0 && y > 0 => Direction::SW,
            Point { x, y } if x < 0 && y < 0 => Direction::NW,
            // case of x: 0, y: 0
            Point { .. } => return Err(()),
        })
    }
}

impl TryFrom<u32> for Direction {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0b000 => Direction::N,
            0b001 => Direction::NW,
            0b010 => Direction::W,
            0b011 => Direction::SW,
            0b100 => Direction::S,
            0b101 => Direction::SE,
            0b110 => Direction::E,
            0b111 => Direction::NE,
            _ => return Err(()),
        })
    }
}

impl Add for Direction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // SAFETY: safe to call unwrap() because of mod-8 adding
        ((self as u32 + rhs as u32) & 0b111).try_into().unwrap()
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::N => "North",
                Direction::NW => "North-West",
                Direction::W => "West",
                Direction::SW => "South-West",
                Direction::S => "South",
                Direction::SE => "South-East",
                Direction::E => "East",
                Direction::NE => "North-East",
            }
        )
    }
}
