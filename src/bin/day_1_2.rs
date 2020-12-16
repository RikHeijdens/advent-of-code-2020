use advent_of_code::day_1::{find_entries_2, read_entries};

fn main() {
    // Read the entries from stdin.
    let entries = read_entries();

    // Find the values that sum to 2020.
    let values = find_entries_2(entries.as_slice(), 2020);
    println!("{}", values.0 * values.1 * values.2)
}
