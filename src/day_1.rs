use std::io;
use std::io::Read;

/// Reads the entries from stdin.
pub fn read_entries() -> Vec<i32> {
    // Allocate a buffer to read standard input into.
    let mut buffer = String::new();

    // Read entries from standard input.
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Expected to read integers from stdin");

    // Split the buffer on new lines.
    let mut entries: Vec<i32> = Vec::new();
    for entry in buffer.split('\n') {
        if entry.len() == 0 {
            continue;
        }
        let entry: i32 = entry.parse().expect("Invalid input provided");
        entries.push(entry);
    }

    entries
}

/// Finds any two entries in `entries` that sum to `value` and return the multiple.
pub fn find_entries_1(entries: &[i32], value: i32) -> (i32, i32) {
    for i in 0..entries.len() {
        for j in 0..entries.len() {
            if entries[i] + entries[j] == value {
                return (entries[i], entries[j]);
            }
        }
    }
    (-1, -1)
}

/// Finds any two entries in `entries` that sum to `value` and return the multiple.
pub fn find_entries_2(entries: &[i32], value: i32) -> (i32, i32, i32) {
    // We need at least three entries.
    if entries.len() < 3 {
        return (-1, -1, -1);
    }
    for i in 0..entries.len() {
        for j in 0..entries.len() {
            for k in 0..entries.len() {
                if entries[i] + entries[j] + entries[k] == value {
                    return (entries[i], entries[j], entries[k]);
                }
            }
        }
    }
    (-1, -1, -1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_entries() {
        let entries = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(find_entries_1(&entries, 2020), (1721, 299));

        let entries = [1, 2];
        assert_eq!(find_entries_1(&entries, 2020), (-1, -1));
    }

    // Unit tests go here.
    #[test]
    fn test_find_entries_2() {
        let entries = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(find_entries_2(&entries, 2020), (979, 366, 675));

        let entries = [1, 2, 3];
        assert_eq!(find_entries_2(&entries, 2020), (-1, -1, -1));
    }
}
