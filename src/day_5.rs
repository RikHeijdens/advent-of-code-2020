/// Parses the boarding pass and returns the position of the seat as a tuple where
/// the first entry represents the row and the second the column.
pub fn parse_boarding_pass(boarding_pass: &str, num_rows: i32, num_cols: i32) -> (i32, i32) {
    let mut row_instructions: Vec<bool> = Vec::new();
    let mut column_instructions: Vec<bool> = Vec::new();
    for c in boarding_pass.chars() {
        match c {
            _ if c == 'F' => {
                row_instructions.push(false);
            }
            _ if c == 'B' => {
                row_instructions.push(true);
            }
            _ if c == 'R' => {
                column_instructions.push(true);
            }
            _ if c == 'L' => {
                column_instructions.push(false);
            }
            _ => {}
        }
    }
    (
        find_split_position(row_instructions, num_rows),
        find_split_position(column_instructions, num_cols),
    )
}

/// Finds the location of split given a vector of instructions.
///
/// The vector of instructions is interpreted as follows:
/// `false` is interpreted as splitting the lower half, while `true` is implemented
/// as splitting the upper half.
pub fn find_split_position(instructions: Vec<bool>, num_locations: i32) -> i32 {
    let mut start = 0; // inclusive
    let mut end = num_locations; // exclusive
    for instruction in instructions {
        if instruction {
            // Upper half
            start = start + ((end - start) / 2)
        } else {
            // Lower half
            end = end - ((end - start) / 2)
        }
    }
    start
}

/// Returns the identifier of the seat given its position.
pub fn seat_id(seat_position: (i32, i32)) -> i32 {
    seat_position.0 * 8 + seat_position.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_boarding_pass() {
        assert_eq!(parse_boarding_pass("FBFBBFFRLR", 128, 8), (44, 5));
        assert_eq!(parse_boarding_pass("BBFFBBFRLL", 128, 8), (102, 4));
        assert_eq!(parse_boarding_pass("BFFFBBFRRR", 128, 8), (70, 7));
        assert_eq!(parse_boarding_pass("FFFBBBFRRR", 128, 8), (14, 7));
    }

    #[test]
    fn test_find_split_position() {
        assert_eq!(
            find_split_position(vec!(false, true, false, true, true, false, false), 128),
            44
        );
    }

    #[test]
    fn test_parse_seat_id() {
        assert_eq!(seat_id((44, 5)), 357);
    }
}
