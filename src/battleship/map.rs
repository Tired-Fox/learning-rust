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
    SUNK,
    OPEN,
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::HIT => write!(f, "\x1b[31mx\x1b[0m"),
            CellState::MISS => write!(f, "o"),
            CellState::OPEN => write!(f, "\x1b[34m~\x1b[0m"),
            CellState::SUNK => write!(f, "\x1b[30mx\x1b[0m"),
        }
    }
}

pub struct Grid {
    sunk: u8,
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
            sunk: 0,
            ships,
            cells: vec![vec![CellState::OPEN; size]; size],
        }
    }

    pub fn update_cell(&mut self, coord: &Coord, new_state: CellState) {
        self.cells[coord.x][coord.y] = new_state;
    }

    pub fn guess(&mut self, coord: &Coord) -> bool {
        if self.cells[coord.x][coord.y] == CellState::OPEN {
            for ship in 0..self.ships.len() {
                if self.ships[ship].within(&coord) {    
                    self.ships[ship].strike();
                    self.update_cell(&coord, CellState::HIT);

                    let curr_ship = self.ships[ship].clone();
                    if self.ships[ship].is_sunk() {
                        self.sunk += 1;
                        for i in 0..self.ships[ship].coords.len() {
                            let coord = curr_ship.coords[i].clone();
                            self.update_cell(&coord, CellState::SUNK);
                        }
                    } 

                    return true;
                }
            }
            
            self.update_cell(&coord, CellState::MISS);
            return true;
        } else {
            return false;
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map: Vec<String> = vec![String::new(); self.cells.len()];

        for x in 0..self.cells.len() {
            for y in 0..self.cells[x].len() {
                map[y].push_str(&format!("  {}", self.cells[x][y]));
            }            
        }
        
        write!(f, "{}", map.join("\n"))
    }
}