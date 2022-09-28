extern crate learning_rust;

use learning_rust::battleship::{
    Ship,
    map::{
        Direction,
        Coord,
        Fire::{
            Hit,
            Miss,
        }
    }
};

fn main() {
    let cruiser: Ship = Ship::new("Cruiser", 8)
        .set_pos(Coord::new(11, 11), Direction::West);

    println!("{}", cruiser);

    let guess: Coord = Coord::new(8, 11);
    println!("Firing at position {}:", guess);
    match cruiser.strike(guess) {
        Hit(message) => println!("{}", message),
        Miss(message) => println!("{}", message),
    }
}