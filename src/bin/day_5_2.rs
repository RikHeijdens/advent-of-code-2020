use std::io;

use advent_of_code::day_5::{parse_boarding_pass, seat_id};

fn main() {
    let stdin = io::stdin();
    let mut seat_ids: Vec<i32> = Vec::new();

    loop {
        let mut buffer = String::new();
        stdin
            .read_line(&mut buffer)
            .expect("Expected to read input from stdin");

        let line = buffer.trim();
        if line.is_empty() {
            break;
        }

        // Store seat ids in a vector such that we can find the missing seat.
        seat_ids.push(seat_id(parse_boarding_pass(line, 128, 8)));
    }

    // Sort the vector of seat ids.
    seat_ids.sort_unstable();

    // Find the missing seat!
    for (i, seat_id) in seat_ids.iter().enumerate() {
        // Our seat was not at the beginning or end...
        if i == 0 {
            continue;
        }
        if seat_id - seat_ids[i - 1] > 1 {
            // We've found the missing seat.
            println!("{}", seat_id - 1);
            return;
        }
    }

    println!("Could not find seat!")
}
