use advent_of_code::day_7::{can_contain_bag, BagRule};
use std::collections::{HashMap, HashSet};
use std::io;

fn main() {
    // Parse rules into a look up table keyed on the color of the bag.
    let mut rule_table: HashMap<String, BagRule> = HashMap::new();
    for rule in BagRule::from_reader(&mut io::stdin().lock()).into_iter() {
        rule_table.insert(rule.bag.color.clone(), rule);
    }

    // The color of the "target" bag.
    let target_color = "shiny gold";

    // We'll track the colors of the bags that may (eventually) contain target_color in found_colors.
    let mut found_colors: HashSet<String> = HashSet::new();

    // Evaluate all bags that we've parsed from the set of rules, and for every bag test whether
    // it can contain bags of "target_color".
    //
    // We optimize the search by referencing whether a color that we reference is already present
    // in found_colors.
    for rule in rule_table.values() {
        if can_contain_bag(&rule.colors(), target_color, &rule_table, &mut found_colors) {
            found_colors.insert(rule.bag.color.to_string());
        }
    }

    println!("{}", found_colors.len());
}
