use std::io::BufRead;

pub fn read_input<R: BufRead>(reader: &mut R) -> Vec<u64> {
    let mut numbers: Vec<u64> = Vec::new();
    loop {
        let mut buffer = String::new();
        reader
            .read_line(&mut buffer)
            .expect("Expected to read input from the reader");

        let line = buffer.trim();
        if line.is_empty() {
            return numbers;
        }

        numbers.push(
            line.parse::<u64>()
                .unwrap_or_else(|err| panic!("Could not parse {} into an u64: {}", line, err)),
        );
    }
}

/// Validates the provided numbers using XMAS cipher and attempts to find the invalid number in
/// the sequence.
pub fn find_invalid_number(preamble_len: u32, numbers: &[u64]) -> Option<u64> {
    let mut preamble: Vec<u64> = Vec::new();
    for (idx, n) in numbers.iter().enumerate() {
        if (idx as u32) < preamble_len {
            preamble.push(*n);
            continue;
        }

        // Check if the entry is valid, entries are only valid if
        // any two entries in `preamble` sum to `n`, with the limitation that
        // the two entries may not be the same entry.
        let mut valid = false;
        for i in preamble.iter() {
            for j in preamble.iter() {
                if i + j == *n {
                    valid = true;
                    break;
                }
            }
            if valid {
                break;
            }
        }

        if !valid {
            return Some(*n);
        }

        // Remove the first entry from preamble.
        preamble.remove(0);
        // And push this number.
        preamble.push(*n);
    }
    None
}

/// Finds a sequence of numbers that sums to the provided `invalid_number`
/// by applying a windowed search.
pub fn find_sequence(invalid_number: u64, numbers: &[u64]) -> Option<u64> {
    for start_index in 0..(numbers.len() - 2) {
        for end_index in (start_index + 1)..numbers.len() {
            // Sum all entries from 'numbers' between 'start_index' and 'end_index'.
            // If the sum is larger than 'invalid_number'.
            let slice = &numbers[start_index..end_index];
            let sum: u64 = slice.iter().sum();

            // Found the start, and end of the sequence.
            // Now, we need to find the smallest and largest number between
            // numbers[start_index]..numbers[end_index] and sum them.
            if sum == invalid_number {
                let min_value = slice.iter().min().unwrap();
                let max_value = slice.iter().max().unwrap();
                return Some(min_value + max_value);
            }

            if sum > invalid_number {
                // No point in searching further using this start_index as
                // we're already summing over the invalid number.
                break;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn get_test_input() -> Vec<u64> {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let mut reader = BufReader::new(input.as_bytes());
        let numbers = read_input(&mut reader);
        numbers
    }

    #[test]
    fn test_reading_input() {
        let numbers = get_test_input();
        assert_eq!(numbers[0], 35);
        assert_eq!(*numbers.last().unwrap(), 576);
    }

    #[test]
    fn test_find_invalid_number() {
        let numbers = get_test_input();
        assert_eq!(find_invalid_number(5, &numbers).unwrap(), 127);
    }

    #[test]
    fn find_invalid_sequence() {
        let numbers = get_test_input();
        let invalid_number = find_invalid_number(5, &numbers).unwrap();
        assert_eq!(find_sequence(invalid_number, &numbers).unwrap(), 62);
    }
}
