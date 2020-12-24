use advent_of_code::day_13::{determine_bus_departure_time, read_input_1};
use std::io;

fn main() {
    let (arrival_time, schedules) = read_input_1(&mut io::stdin().lock());
    let (bus_id, departure_time) = determine_bus_departure_time(&arrival_time, &schedules);
    println!("{}", (departure_time - arrival_time) * bus_id);
}
