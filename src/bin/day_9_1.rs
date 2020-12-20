use advent_of_code::day_9::{find_invalid_number, read_input};
use std::io;

fn main() {
    let numbers = read_input(&mut io::stdin().lock());
    println!("{}", find_invalid_number(25, &numbers).unwrap());
}
