use advent_of_code::day_8::{Instruction, Processor};
use std::io;

fn main() {
    let instructions = Instruction::from_reader(&mut io::stdin().lock());
    let mut processor = Processor::new();
    println!("{}", processor.find_cycle(&instructions).1);
}
