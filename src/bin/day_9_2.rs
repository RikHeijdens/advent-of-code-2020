use advent_of_code::day_9::{find_invalid_number, find_sequence, read_input};
use std::io;

fn main() {
    let numbers = read_input(&mut io::stdin().lock());
    let invalid_number = find_invalid_number(25, &numbers).unwrap();
    println!(
        "{}",
        find_sequence(invalid_number, &numbers)
            .unwrap_or_else(|| panic!("Could not find the weakness!"))
    );
}
