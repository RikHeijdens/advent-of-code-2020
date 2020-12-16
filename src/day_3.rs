use std::io::BufRead;

pub fn count_trees(map: &Vec<Vec<char>>, step_x: usize, step_y: usize) -> i32 {
    let mut count: i32 = 0;
    // Trees are represented as a '#', count the number of trees that we crash into.
    for i in 0..map.len() {
        let row_index = i * step_y;
        if row_index > map.len() - 1 {
            // Prevent going out of bounds.
            break;
        }
        let col_index = (i * step_x) % map[row_index].len();
        if map[row_index][col_index] == '#' {
            count += 1
        }
    }
    count
}

pub fn read_map<R: BufRead>(input: &mut R) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();

    loop {
        // Allocate a buffer to read standard input into.
        let mut buffer = String::new();

        // Read entries from standard input.
        input
            .read_line(&mut buffer)
            .expect("Expected to read data from stdin!");

        if buffer.len() == 0 {
            // End of Input
            break;
        }

        let chars: Vec<char> = buffer.trim().chars().collect();
        map.push(chars);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test() {
        let map = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        let mut reader = BufReader::new(map.as_bytes());
        let map_vec = read_map(&mut reader);
        assert_eq!(map_vec.len(), 11);
        assert_eq!(
            map_vec[0],
            vec!('.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.')
        );

        let tree_count = count_trees(&map_vec, 3, 1);
        assert_eq!(tree_count, 7);
    }
}
