pub mod map;

use std::fmt::Display;
use map::{
    Direction,
    Coord,    
};

#[derive(Clone)]
pub struct Ship<'a> {
    name:  &'a str,
    length: u16,
    hits: u16,
    direction: Direction,
    coords: Vec<Coord>,
}

impl<'a> Ship<'a> {
    pub fn new(name: &str, length: u16) -> Ship {
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

    pub fn length(&self) -> u16 {
        self.length
    }

    pub fn set_pos(mut self, coord: Coord, direction: Direction) -> Self {
        match direction {
            Direction::North => { 
                for i in 0..self.length { 
                    self.coords.push(Coord {x: coord.x, y: usize::try_from(i).unwrap() + coord.y}) 
                } 
            },
            Direction::East  => { 
                for i in 0..self.length { 
                    self.coords.push(Coord {x: coord.x + usize::try_from(i).unwrap(), y: coord.y}) 
                } 
            },
            Direction::South => { 
                for i in 0..self.length { 
                    self.coords.push(Coord {x: coord.x, y: coord.y - usize::try_from(i).unwrap()}) 
                } 
            },
            Direction::West  => { 
                for i in 0..self.length { 
                    self.coords.push(Coord {x: coord.x - usize::try_from(i).unwrap(), y: coord.y}) 
                } 
            },
        }
        self.direction = direction;

        self
    }

    pub fn within(&self, coord: &Coord) -> bool {
        match self.direction {
            Direction::North => { 
                if coord.x == self.coords[0].x {
                    if coord.y >= self.coords[0].y && coord.y <= self.coords.last().unwrap().y {
                        return true;
                    }
                } 
            },
            Direction::East => { 
                if coord.y == self.coords[0].y {
                    if coord.x >= self.coords[0].x && coord.x <= self.coords.last().unwrap().x {
                        return true;
                    }
                } 
            },
            Direction::South => { 
                if coord.x == self.coords[0].x {
                    if coord.y >= self.coords.last().unwrap().y && coord.y <= self.coords[0].y {
                        return true;
                    }
                } 
            },
            Direction::West => { 
                if coord.y == self.coords[0].y {
                    if coord.x >= self.coords.last().unwrap().x && coord.x <= self.coords[0].x {
                        return true;
                    }
                }
            },
        }
        return false;
    }

    pub fn is_sunk(&self) -> bool {
        if self.hits == self.length - 1 {
            return true;
        } 
        return false;
    }

    pub fn strike(&mut self) {
        self.hits += 1;
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