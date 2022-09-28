pub mod map;

use std::fmt::Display;
use map::{
    Direction,
    Coord,
    Fire
};

pub struct Ship<'a> {
    name:  &'a str,
    length: u32,
    hits: i8,
    direction: Direction,
    coords: Vec<Coord>,
}

impl<'a> Ship<'a> {
    pub fn new(name: &str, length: u32) -> Ship {
        Ship {
            name,
            length,
            hits: 0,
            direction: Direction::North,
            coords: Vec::new()
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn set_pos(mut self, coord: Coord, direction: Direction) -> Self {
        match direction {
            Direction::North => { 
                for i in 0..self.length { 
                    self.coords.push(Coord {x: coord.x, y: coord.y + i}) 
                } 
            },
            Direction::East  => { 
                for i in 0..self.length { 
                    self.coords.push(Coord {x: coord.x + i, y: coord.y}) 
                } 
            },
            Direction::South => { 
                for i in 0..self.length { 
                    self.coords.push(Coord {x: coord.x, y: coord.y - i}) 
                } 
            },
            Direction::West  => { 
                for i in 0..self.length { 
                    self.coords.push(Coord {x: coord.x - i, y: coord.y}) 
                } 
            },
        }
        self.direction = direction;

        self
    }

    pub fn strike(&self, coord: Coord) -> Fire<'a> {
        match self.direction {
            Direction::North => { 
                if coord.x == self.coords[0].x {
                    if coord.y >= self.coords[0].y && coord.y <= self.coords.last().unwrap().y {
                        return Fire::Hit("Hit!")
                    }
                } 
            },
            Direction::East => { 
                if coord.y == self.coords[0].y {
                    if coord.x >= self.coords[0].x && coord.x <= self.coords.last().unwrap().x {
                        return Fire::Hit("Hit!")
                    }
                } 
            },
            Direction::South => { 
                if coord.x == self.coords[0].x {
                    if coord.y >= self.coords.last().unwrap().y && coord.y <= self.coords[0].y {
                        return Fire::Hit("Hit!")
                    }
                } 
            },
            Direction::West => { 
                if coord.y == self.coords[0].y {
                    if coord.x >= self.coords.last().unwrap().x && coord.x <= self.coords[0].x {
                        return Fire::Hit("Hit!")
                    }
                }
            },
        }
        return Fire::Miss("Miss!")
    }
}

impl<'a> Display for Ship<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let coordinates: Vec<String> = self.coords
            .iter()
            .map(|coord: &Coord| coord.to_string())
            .collect();

        write!(f, "Ship {{\n  name: {},\n  length: {},\n  direction: {},\n  hits: {},\n  coords: [{}]\n}}", self.name, self.length, self.direction, self.hits, coordinates.join(", "))
    }
}