use std::cmp::{max, min};
use std::io::BufRead;
use std::iter;

pub fn read_seating_plan<R: BufRead>(reader: &mut R) -> Vec<Vec<char>> {
    let mut seating_plan = Vec::new();
    loop {
        let mut buffer = String::new();
        reader
            .read_line(&mut buffer)
            .expect("Expected to read input from the provided reader");

        let line = buffer.trim();
        if line.is_empty() {
            break;
        }

        seating_plan.push(line.chars().collect());
    }
    seating_plan
}

/// Seats individuals by applying all rules to all spots at the same time.
pub fn seat_individuals_1(seating_plan: &[Vec<char>]) -> Vec<Vec<char>> {
    // Allocate a new Vec<Vec<char>> for the updated seating plan.
    let mut updated_seating_plan = seating_plan.to_owned();
    for row_index in 0..seating_plan.len() {
        for col_index in 0..seating_plan[row_index].len() {
            match seating_plan[row_index][col_index] {
                'L' => {
                    // Seat is empty, the seat becomes occupied if there is no seat
                    // adjacent to this seat that is occupied.
                    if count_occupied_adjacent_seats(&seating_plan, row_index, col_index) == 0 {
                        updated_seating_plan[row_index][col_index] = '#';
                    }
                }
                '#' => {
                    // Seat is occupied. If four or more seats are adjacent to it are also occupied,
                    // the seat becomes empty.
                    if count_occupied_adjacent_seats(&seating_plan, row_index, col_index) >= 4 {
                        updated_seating_plan[row_index][col_index] = 'L';
                    }
                }
                _ => {}
            }
        }
    }
    updated_seating_plan
}

/// Sets individuals for part two of the exercise.
pub fn seat_individuals_2(seating_plan: &[Vec<char>]) -> Vec<Vec<char>> {
    // Allocate a new Vec<Vec<char>> for the updated seating plan.
    let mut updated_seating_plan = seating_plan.to_owned();
    for row_index in 0..seating_plan.len() {
        for col_index in 0..seating_plan[row_index].len() {
            match seating_plan[row_index][col_index] {
                'L' => {
                    // Seat is empty, the seat becomes occupied if there is no seat in any
                    // direction that is occupied.
                    if count_occupied_seats_directions(&seating_plan, row_index, col_index) == 0 {
                        updated_seating_plan[row_index][col_index] = '#';
                    }
                }
                '#' => {
                    // Seat is occupied. If five or more seats are adjacent to it are also occupied,
                    // the seat becomes empty.
                    if count_occupied_seats_directions(&seating_plan, row_index, col_index) >= 5 {
                        updated_seating_plan[row_index][col_index] = 'L';
                    }
                }
                _ => {}
            }
        }
    }
    updated_seating_plan
}

/// Counts the number of occupied seats adjacent to the seat indexed by `row_index` and `col_index`.
///
/// This function is used for part 1 of the exercise.
fn count_occupied_adjacent_seats(
    seating_plan: &[Vec<char>],
    row_index: usize,
    col_index: usize,
) -> usize {
    // Count start and end indices for the region to consider.
    let row_start = max(row_index as isize - 1, 0) as usize;
    let row_end = min(row_index + 1, seating_plan.len() - 1);
    let col_start = max(col_index as isize - 1, 0) as usize;
    let col_end = min(col_index + 1, seating_plan[row_end].len() - 1);

    let mut num_occupied = 0;
    #[allow(clippy::needless_range_loop)]
    for i in row_start..=row_end {
        for j in col_start..=col_end {
            if i == row_index && j == col_index {
                continue;
            }
            if seating_plan[i][j] == '#' {
                num_occupied += 1;
            }
        }
    }

    num_occupied
}

/// Counts the number of occupied seats by considering the first visible seat in any of the 8
/// possible directions.
fn count_occupied_seats_directions(
    seating_plan: &[Vec<char>],
    row_index: usize,
    col_index: usize,
) -> usize {
    let mut num_occupied = 0;

    // Check if there is an occupied seat to the north.
    let mut iter = (0..row_index).rev().zip(iter::repeat(col_index));
    num_occupied += count_seats_direction(seating_plan, &mut iter);

    // Check if there is an occupied seat to the south.
    let mut iter = (row_index + 1..seating_plan.len()).zip(iter::repeat(col_index));
    num_occupied += count_seats_direction(seating_plan, &mut iter);

    // Check if there is an occupied seat to the west.
    let mut iter = iter::repeat(row_index).zip((0..col_index).rev());
    num_occupied += count_seats_direction(seating_plan, &mut iter);

    // Count the number of occupied seats to the east.
    let mut iter = iter::repeat(row_index).zip(col_index + 1..seating_plan[row_index].len());
    num_occupied += count_seats_direction(seating_plan, &mut iter);

    // Count the number of seats to the north-west.
    let mut iter = (0..row_index).rev().zip((0..col_index).rev());
    num_occupied += count_seats_direction(seating_plan, &mut iter);

    // Count the number of seats to the north-east.
    let mut iter = (0..row_index)
        .rev()
        .zip(col_index + 1..seating_plan[0].len());
    num_occupied += count_seats_direction(seating_plan, &mut iter);

    // Count the number of seats to the south-east.
    let mut iter = (row_index + 1..seating_plan.len())
        .zip(col_index + 1..seating_plan[seating_plan.len() - 1].len());
    num_occupied += count_seats_direction(seating_plan, &mut iter);

    // Count the number of seats to the south-west.
    let mut iter = (row_index + 1..seating_plan.len()).zip((0..col_index).rev());
    num_occupied += count_seats_direction(seating_plan, &mut iter);

    num_occupied
}

/// Counts the total number of occupied seats.
pub fn count_total_occupied_seats(seating_plan: &[Vec<char>]) -> usize {
    seating_plan.iter().flatten().filter(|x| **x == '#').count()
}

/// Counts the number of seats visible in a certain direction, using the indices provided by `iter`.
fn count_seats_direction<I>(seating_plan: &[Vec<char>], iter: &mut I) -> usize
where
    I: Iterator<Item = (usize, usize)>,
{
    for (i, j) in iter {
        match seating_plan[i][j] {
            '#' => {
                return 1;
            }
            'L' => {
                return 0;
            }
            _ => {}
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_seat_individuals_1() {
        let initial_plan = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
        let plan = read_seating_plan(&mut BufReader::new(initial_plan.as_bytes()));
        assert_eq!(plan.len(), 10);
        assert_eq!(plan[0].len(), 10);
        assert_eq!(
            plan[0],
            vec!('L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L')
        );

        let plan_after_one_round = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";
        let one_round = read_seating_plan(&mut BufReader::new(plan_after_one_round.as_bytes()));
        let after_one_round = seat_individuals_1(&plan);
        assert_eq!(after_one_round, one_round);

        let plan_after_two_rounds = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";
        let second_round = read_seating_plan(&mut BufReader::new(plan_after_two_rounds.as_bytes()));
        let after_two_rounds = seat_individuals_1(&after_one_round);
        assert_eq!(after_two_rounds, second_round);

        let plan_after_three_rounds = "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##";
        let third_round =
            read_seating_plan(&mut BufReader::new(plan_after_three_rounds.as_bytes()));
        let after_three_rounds = seat_individuals_1(&after_two_rounds);
        assert_eq!(after_three_rounds, third_round);
    }

    #[test]
    fn test_seat_individuals_2() {
        let initial_plan = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let plan = read_seating_plan(&mut BufReader::new(initial_plan.as_bytes()));

        let plan_after_one_round = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";
        let one_round = read_seating_plan(&mut BufReader::new(plan_after_one_round.as_bytes()));
        let after_one_round = seat_individuals_2(&plan);
        assert_eq!(after_one_round, one_round);

        let plan_after_second_round = "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#";
        let second_round =
            read_seating_plan(&mut BufReader::new(plan_after_second_round.as_bytes()));
        let after_second_round = seat_individuals_2(&after_one_round);
        assert_eq!(after_second_round, second_round);

        let plan_after_third_round = "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#";
        let third_round = read_seating_plan(&mut BufReader::new(plan_after_third_round.as_bytes()));
        let after_third_round = seat_individuals_2(&after_second_round);
        assert_eq!(after_third_round, third_round);
    }

    #[test]
    fn test_num_occupied() {
        let plan = vec![
            vec!['L', 'L', '#'],
            vec!['L', 'L', 'L'],
            vec!['L', 'L', 'L'],
        ];
        assert_eq!(count_occupied_adjacent_seats(&plan, 1, 1), 1);
        assert_eq!(count_occupied_adjacent_seats(&plan, 2, 0), 0);
        assert_eq!(count_total_occupied_seats(&plan), 1);
    }

    #[test]
    fn test_num_occupied_directions() {
        let plan_occupied = read_seating_plan(&mut BufReader::new(
            ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....
"
            .as_bytes(),
        ));
        assert_eq!(count_occupied_seats_directions(&plan_occupied, 4, 3), 8);

        let plan_empty = read_seating_plan(&mut BufReader::new(
            ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.
"
            .as_bytes(),
        ));
        assert_eq!(count_occupied_seats_directions(&plan_empty, 3, 3), 0);
    }
}
