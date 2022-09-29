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
    
    map.guess(&guess);
    map.guess(&Coord::new(14, 14));
    println!("{}", map);
}