use advent_of_code::day_4::Passport;
use std::io;

fn main() {
    let mut passports = Passport::from_reader(&mut io::stdin().lock());

    // Filter invalid passports out.
    passports = passports.into_iter().filter(|p| p.is_valid_1()).collect();
    println!("{}", passports.len());
}
