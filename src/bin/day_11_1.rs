use advent_of_code::day_11::{count_total_occupied_seats, read_seating_plan, seat_individuals_1};
use std::io;

fn main() {
    let mut seating_plan = read_seating_plan(&mut io::stdin().lock());
    loop {
        let updated_plan = seat_individuals_1(&seating_plan);
        if updated_plan == seating_plan {
            println!("{}", count_total_occupied_seats(&updated_plan));
            return;
        }
        seating_plan = updated_plan
    }
}
