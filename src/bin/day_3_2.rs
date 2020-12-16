use std::io;

use advent_of_code::day_3;

fn main() {
    // Read the map.
    let map: Vec<Vec<char>> = day_3::read_map(&mut io::stdin().lock());

    // Define the steps.
    let steps = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut tree_multiple: i64 = 1;

    // Count the number of trees on each of the paths and multiply them.
    for (step_x, step_y) in steps.iter() {
        tree_multiple *= day_3::count_trees(&map, *step_x, *step_y) as i64;
    }

    println!("{}", tree_multiple);
}
