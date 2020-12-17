use std::collections::{HashMap, HashSet};
use std::io::BufRead;

/// Reads customs declarations from `reader`.
pub fn read_declarations<R: BufRead>(reader: &mut R) -> Vec<Vec<String>> {
    let mut declarations: Vec<Vec<String>> = Vec::new();
    loop {
        let mut group_declarations: Vec<String> = Vec::new();
        loop {
            // Allocate a buffer to read standard input into.
            let mut buffer = String::new();

            // Read entries from standard input.
            reader
                .read_line(&mut buffer)
                .expect("Expected to read data from the reader!");

            let line = buffer.trim();
            if line.is_empty() {
                // End of Input
                break;
            }
            group_declarations.push(line.to_string())
        }

        if group_declarations.is_empty() {
            break;
        }
        declarations.push(group_declarations)
    }
    declarations
}

/// Counts the number of declarations to which anyone group members said `yes`.
pub fn count_any_declarations(declarations: &[Vec<String>]) -> i32 {
    let mut count: i32 = 0;
    for group_declarations in declarations.iter() {
        let mut distinct_declarations: HashSet<char> = HashSet::new();
        // Split group declarations on individual chars and track them in a HashSet.
        for declaration in group_declarations.iter() {
            distinct_declarations.extend(declaration.chars())
        }
        count += distinct_declarations.len() as i32
    }
    count
}

/// Counts the number of declarations to which `all` group members said `yes`.
pub fn count_all_declarations(declarations: &[Vec<String>]) -> i32 {
    let mut count: i32 = 0;
    for group_declarations in declarations.iter() {
        let mut declaration_map: HashMap<char, i32> = HashMap::new();
        // Count how often an item was declared per group in an HashMap.
        for declaration in group_declarations.iter() {
            for char in declaration.chars() {
                let stat = declaration_map.entry(char).or_insert(0);
                *stat += 1;
            }
        }

        // Count which items were declared by every single member in the group.
        let group_size = group_declarations.len() as i32;
        for (_, value) in declaration_map.iter() {
            if *value == group_size {
                count += 1
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_read_and_count_declarations() {
        let declarations = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let mut reader = BufReader::new(declarations.as_bytes());
        let d = read_declarations(&mut reader);
        assert_eq!(
            d,
            vec!(
                vec!("abc"),
                vec!("a", "b", "c"),
                vec!("ab", "ac"),
                vec!("a", "a", "a", "a"),
                vec!("b")
            )
        );

        assert_eq!(count_any_declarations(&d), 11);
        assert_eq!(count_all_declarations(&d), 6);
    }
}
