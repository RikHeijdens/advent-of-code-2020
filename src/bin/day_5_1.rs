use std::io;

use advent_of_code::day_5::{parse_boarding_pass, seat_id};

fn main() {
    let stdin = io::stdin();
    let mut max_seat_id = 0;

    loop {
        let mut buffer = String::new();
        stdin
            .read_line(&mut buffer)
            .expect("Expected to read input from stdin");

        let line = buffer.trim();
        if line.is_empty() {
            break;
        }

        // Parse every boarding pass into a seat id, and retain it if the seat id is higher than the one we had.
        let seat_id = seat_id(parse_boarding_pass(line, 128, 8));
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }

    println!("{}", max_seat_id)
}
