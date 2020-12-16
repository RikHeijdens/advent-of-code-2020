// for https://adventofcode.com/2020/day/2
use advent_of_code::day_2::{read_password_policies, PasswordPolicy};

fn main() {
    let mut password_policies: Vec<PasswordPolicy> = read_password_policies();

    // Count the number of policies that are valid.
    password_policies = password_policies
        .into_iter()
        .filter(|policy| policy.is_valid_1())
        .collect();
    println!("{}", password_policies.len());
}
