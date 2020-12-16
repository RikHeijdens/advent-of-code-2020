use std::io;

use advent_of_code::day_3;

fn main() {
    // Read the map from standard input.
    let map: Vec<Vec<char>> = day_3::read_map(&mut io::stdin().lock());

    // Define the step sizes.
    let step_x = 3;
    let step_y = 1;

    // Count the number of trees we'll encounter.
    let count = day_3::count_trees(&map, step_x, step_y);
    println!("{}", count);
}
