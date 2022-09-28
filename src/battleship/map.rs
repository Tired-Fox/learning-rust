use std::fmt::Display;

pub struct Coord {
    pub x: u32,
    pub y: u32,
}

impl Coord {
    pub fn new(x: u32, y: u32) -> Coord {
        Coord {
            x,
            y
        }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::East => write!(f, "East"),
            Direction::South => write!(f, "South"),
            Direction::West => write!(f, "West"),
        }
    }
}

pub enum Fire<'a> {
    Hit(&'a str),
    Miss(&'a str),
}