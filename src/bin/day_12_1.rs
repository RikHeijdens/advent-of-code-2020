use advent_of_code::day_12::{read_instructions, Ship};
use std::io;

fn main() {
    let instructions = read_instructions(&mut io::stdin().lock());
    let mut ship = Ship::new();
    for instruction in instructions {
        ship.move_ship(&instruction);
    }
    println!("{}", ship.distance());
}
