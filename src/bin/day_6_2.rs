use advent_of_code::day_6::{count_all_declarations, read_declarations};
use std::io;

fn main() {
    let declarations: Vec<Vec<String>> = read_declarations(&mut io::stdin().lock());
    println!("{}", count_all_declarations(&declarations));
}
