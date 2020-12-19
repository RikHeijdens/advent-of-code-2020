use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::io::BufRead;
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub struct Bag {
    // The color of the bag.
    pub color: String,
}

#[derive(Debug, PartialEq)]
pub struct BagRule {
    // The bag to which the rule applies.
    pub bag: Bag,
    // The colors of the bags that must be contained by this Bag and their respective quantities.
    pub contains: HashMap<String, u32>,
}

impl BagRule {
    /// Reads input into a vector of BagRules.
    pub fn from_reader<R: BufRead>(reader: &mut R) -> Vec<BagRule> {
        let mut rules: Vec<BagRule> = Vec::new();
        loop {
            let mut buffer = String::new();
            reader
                .read_line(&mut buffer)
                .expect("Expected to read input from the reader.");

            let rule = buffer.trim();
            if rule.is_empty() {
                break;
            }
            match BagRule::try_from(rule) {
                Ok(rule) => rules.push(rule),
                _ => {
                    eprintln!("Failed to parse rule: {}", rule)
                }
            }
        }
        rules
    }

    /// Returns the set of colors that are being referenced by this rule.
    pub fn colors(&self) -> HashSet<&String> {
        HashSet::from_iter(self.contains.keys())
    }
}

impl TryFrom<&str> for BagRule {
    type Error = String;

    /// Attempts to parse a BagRule from a &str
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bag_color;
        let mut bag_contents: HashMap<String, u32> = HashMap::new();

        let mut splits = value.split("bags contain");

        // Parse the color.
        match splits.next() {
            Some(c) => bag_color = c.trim(),
            _ => return Err("Failed to parse rule: could not determine color.".to_string()),
        }

        // Attempt to parse the contents of the bag.
        let contents;
        match splits.next() {
            Some(c) => contents = c.trim(),
            _ => return Err("Failed to parse rule: could not determine bag contents.".to_string()),
        }

        if contents == "no other bags." {
            return Ok(BagRule {
                bag: Bag {
                    color: bag_color.to_string(),
                },
                contains: bag_contents,
            });
        }

        // This bag is allowed to contain other bags, attempt to parse the rules.
        for split in contents.split(',') {
            // Parses components such as:
            // 1 shiny gold bag
            // 1 bright white bag
            // 2 muted yellow bags.
            let trimmed = split
                .trim()
                .trim_end_matches('.')
                .trim_end_matches('s')
                .trim_end_matches(" bag");
            let mut iter = trimmed.splitn(2, ' ');
            let quantifier;
            let color;
            // Parse quantifier
            match iter.next() {
                Some(q) => match q.parse::<u32>() {
                    Ok(q) => quantifier = q,
                    _ => return Err(format!("Failed to parse quantifier {} from: {}", q, split)),
                },
                _ => return Err(format!("Failed to parse quantifier for: {}", split)),
            }
            // Parse color
            match iter.next() {
                Some(c) => color = c.trim(),
                _ => return Err(format!("Failed to parse color from: {}", split)),
            }
            bag_contents.insert(color.to_string(), quantifier);
        }

        Ok(BagRule {
            bag: Bag {
                color: bag_color.to_string(),
            },
            contains: bag_contents,
        })
    }
}

/// Tests whether any of the colors in the provided set of `colors` may contain a bag of
/// `target_color` while respecting the rules stored in the provided `rule_table`.
///
/// If a particular color is found to be allowed to contain a bag of a particular color, then
/// that fact is stored in the `found_colors` set in order to optimize subsequent lookups.
pub fn can_contain_bag(
    colors: &HashSet<&String>,
    target_color: &str,
    rule_table: &HashMap<String, BagRule>,
    found_colors: &mut HashSet<String>,
) -> bool {
    for color in colors.iter() {
        // If *this* color is the target_color, then the bag for which we are evaluating
        // colors, return true.
        if *color == target_color {
            return true;
        }
        // Test if we've already found a path from *this* color to target_color.
        // In this case we won't have to perform a DFS.
        if found_colors.contains(color.as_str()) {
            return true;
        }
        // Recursively test whether any of the other bags may contain target_color.
        let rule = rule_table
            .get(*color)
            .unwrap_or_else(|| panic!("Expected to have a rule for: {}", *color));
        let can_contain = can_contain_bag(&rule.colors(), target_color, rule_table, found_colors);
        if can_contain {
            found_colors.insert(color.to_string());
            return true;
        }
    }
    false
}

/// Counts how many bags need to be contained in a bag of "target_color", given the set of rules.
pub fn count_bags(target_color: &str, rule_table: &HashMap<String, BagRule>) -> u32 {
    let mut count = 0;
    let rule = rule_table
        .get(target_color)
        .unwrap_or_else(|| panic!("Expected to have a rule for color {}", target_color));

    for color in rule.colors() {
        // Count the bags that are directly contained by target_color for color.
        let color_count = *rule.contains.get(color).unwrap();
        // And count any bags that are contained by color on its own.
        let child_bag_count = count_bags(color, rule_table);
        count = count + color_count + (color_count * child_bag_count);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_from_reader() {
        let rules = "light red bags contain 1 bright white bag, 2 muted yellow bags.
wavy fuchsia bags contain 5 vibrant magenta bags, 2 dull maroon bags, 4 faded lime bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let mut reader = BufReader::new(rules.as_bytes());
        let rules = BagRule::from_reader(&mut reader);
        assert_eq!(rules.len(), 10);

        // Validate that the rules are being parsed properly.
        let mut rule_map: HashMap<String, u32> = HashMap::new();
        rule_map.insert("bright white".to_string(), 1);
        rule_map.insert("muted yellow".to_string(), 2);
        assert_eq!(
            rules[0],
            BagRule {
                bag: Bag {
                    color: "light red".to_string()
                },
                contains: rule_map
            }
        );

        rule_map = HashMap::new();
        rule_map.insert("vibrant magenta".to_string(), 5);
        rule_map.insert("dull maroon".to_string(), 2);
        rule_map.insert("faded lime".to_string(), 4);
        assert_eq!(
            rules[1],
            BagRule {
                bag: Bag {
                    color: "wavy fuchsia".to_string()
                },
                contains: rule_map
            }
        )
    }

    #[test]
    fn test_can_contain_color() {
        let rules = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let mut reader = BufReader::new(rules.as_bytes());

        // Parse rules into a look up table keyed on the color of the bag.
        let mut rule_table: HashMap<String, BagRule> = HashMap::new();
        for rule in BagRule::from_reader(&mut reader).into_iter() {
            rule_table.insert(rule.bag.color.clone(), rule);
        }

        // The color of the "target" bag.
        let target_color = "shiny gold";

        // We'll track the colors of the bags that may (eventually) contain target_color in found_colors.
        let mut found_colors: HashSet<String> = HashSet::new();

        // Test a simple lookup which does not require recursion.
        assert_eq!(
            can_contain_bag(
                &rule_table.get("bright white").unwrap().colors(),
                target_color,
                &rule_table,
                &mut found_colors
            ),
            true
        );
        // After just one look up we expect the lookup table to be empty.
        assert_eq!(found_colors.len(), 0);

        // Test a more complex lookup for which we need to recurse through rules.
        assert_eq!(
            can_contain_bag(
                &rule_table.get("dark orange").unwrap().colors(),
                target_color,
                &rule_table,
                &mut found_colors
            ),
            true
        );
        // Test that the found colors set was updated.
        assert!(found_colors.len() > 0);
    }

    #[test]
    fn test_count_bags() {
        let rules = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let mut reader = BufReader::new(rules.as_bytes());

        // Parse rules into a look up table keyed on the color of the bag.
        let mut rule_table: HashMap<String, BagRule> = HashMap::new();
        for rule in BagRule::from_reader(&mut reader).into_iter() {
            rule_table.insert(rule.bag.color.clone(), rule);
        }
        assert_eq!(count_bags("shiny gold", &rule_table), 126);
    }
}
