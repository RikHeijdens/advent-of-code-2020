use advent_of_code::day_13::{find_earliest_bus_departure_time, read_input_2};
use std::io;

fn main() {
    let schedules = read_input_2(&mut io::stdin().lock());
    match find_earliest_bus_departure_time(&schedules) {
        Some(departure_time) => {
            println!("{:?}", departure_time);
        }
        None => {
            println!("Could not find a solution!");
        }
    }
}
