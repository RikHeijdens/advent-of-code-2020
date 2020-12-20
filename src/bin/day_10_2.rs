use std::io;
// Reuse the input reading fn from day_9, as it does not make any sense
// to reimplement it for this exercise.
use advent_of_code::day_10::find_adapter_arrangements;
use advent_of_code::day_9::read_input;

fn main() {
    let adapters = read_input(&mut io::stdin().lock());
    println!("{}", find_adapter_arrangements(&adapters));
}
