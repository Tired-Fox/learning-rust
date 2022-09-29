use std::{fmt::Display};

use super::Ship;

#[derive(Clone, Copy)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: u16, y: u16) -> Coord {
        Coord {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy)]
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

// ----------------------------------
//  Map and Grid
// ----------------------------------

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    HIT,
    MISS,
    OPEN,
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::HIT => write!(f, "\x1b[31mx\x1b[0m"),
            CellState::MISS => write!(f, "o"),
            CellState::OPEN => write!(f, "\x1b[34m~\x1b[0m"),
        }
    }
}

pub struct Grid {
    ships: Vec<Ship<'static>>,
    cells: Vec<Vec<CellState>>,
}

impl Grid {
    pub fn new(size: usize) -> Grid {
        let mut ships: Vec<Ship<'static>> = Vec::new();
        ships.push(Ship::new("Carrier", 5).set_pos(Coord::new(11, 11), Direction::West));
        ships.push(Ship::new("Battleship", 4).set_pos(Coord{x: 0, y: 0}, Direction::East));
        ships.push(Ship::new("Cruiser", 3).set_pos(Coord{x: 0, y: 0}, Direction::East));
        ships.push(Ship::new("Submarine", 3).set_pos(Coord{x: 0, y: 0}, Direction::East));
        ships.push(Ship::new("Destroyer", 2).set_pos(Coord{x: 0, y: 0}, Direction::East));

        Grid { 
            ships,
            cells: vec![vec![CellState::OPEN; size]; size],
        }
    }

    pub fn update_cell(&mut self, coord: &Coord, new_state: CellState) {
        if self.cells[coord.x][coord.y] == CellState::OPEN {
            self.cells[coord.x][coord.y] = new_state;
        }
    }

    pub fn guess(&mut self, coord: &Coord) {
        for ship in self.ships.iter_mut() {
            if ship.within(&coord) {
                println!("Hit!");

                ship.strike();
                self.update_cell(&coord, CellState::HIT);
                return;
            }
        }
        
        self.update_cell(&coord, CellState::MISS);
        println!("Miss!");
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map: String = String::new();
        for x in self.cells.iter() {
            for y in x.iter() {
                map.push_str(&format!("  {}", y));
            }
            map.push_str("\n");
        }
        write!(f, "{}", map)
    }
}