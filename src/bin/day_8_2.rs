use advent_of_code::day_8::{find_fix, Instruction};
use std::io;

fn main() {
    let instructions = Instruction::from_reader(&mut io::stdin().lock());
    println!("{}", find_fix(&instructions));
}
