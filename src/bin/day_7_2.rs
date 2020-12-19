use advent_of_code::day_7::{count_bags, BagRule};
use std::collections::HashMap;
use std::io;

fn main() {
    // Parse rules into a look up table keyed on the color of the bag.
    let mut rule_table: HashMap<String, BagRule> = HashMap::new();
    for rule in BagRule::from_reader(&mut io::stdin().lock()).into_iter() {
        rule_table.insert(rule.bag.color.clone(), rule);
    }

    // The color of the "target" bag.
    let target_color = "shiny gold";
    println!("{}", count_bags(target_color, &rule_table));
}
