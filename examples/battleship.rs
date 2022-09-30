extern crate learning_rust;

use learning_rust::battleship::{
    map::{
        Coord,
        Grid,
    }
};

fn main() {
    let mut map: Grid = Grid::new(15);

    let guess: Coord = Coord::new(8, 11);
    println!("Firing at position {}:", guess);
    
    map.guess(&Coord::new(11, 11));
    map.guess(&Coord::new(10, 11));
    map.guess(&Coord::new(9, 11));
    map.guess(&guess);
    map.guess(&Coord::new(7, 11));
    map.guess(&Coord::new(6, 11));
    println!("{}", map);
}