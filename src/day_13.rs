use std::collections::HashSet;
use std::io::BufRead;

/// Determines the ID of the bus and the departure time of the bus for the given `arrival_time`.
pub fn determine_bus_departure_time(
    arrival_time: &isize,
    schedule: &HashSet<isize>,
) -> (isize, isize) {
    let mut departure_time = isize::MAX;
    let mut bus_id = 0;

    for bus in schedule.iter() {
        let mut closest_bus_departure_time = isize::MAX;
        let mut n = 1;
        loop {
            // Calculate the departure time of the n-th bus on the `bus` schedule.
            let d = n * bus;
            // Calculate the difference between the departure time of the n-th bus
            // and our arrival time. If it is negative, that means we can't take
            // this bus because it arrives before we did.
            let diff = (d - arrival_time) as i32;
            if diff < 0 {
                // We can't take this bus, it arrives before we get there.
                n += 1;
                continue;
            }
            if diff > (closest_bus_departure_time - arrival_time) as i32
                && closest_bus_departure_time != isize::MAX
            {
                // Stop, a previously arriving bus was a better idea.
                break;
            }
            closest_bus_departure_time = d;
            n += 1;
        }
        // Check if this bus provides a better option than any of the other bus schedules we
        // considered earlier.
        if closest_bus_departure_time < departure_time {
            bus_id = *bus;
            departure_time = closest_bus_departure_time;
        }
    }
    (bus_id, departure_time)
}

/// Finds the earliest timestamp such that the first bus ID departs at that time
/// and each subsequent listed bus ID departs at that subsequent minute.
pub fn find_earliest_bus_departure_time(schedule: &[String]) -> Option<i64> {
    // Determine the divisors and the desired remainders.
    let mut divisors = Vec::new();
    let mut remainders = Vec::new();

    for (index, bus_schedule) in schedule.iter().enumerate() {
        // Solution doesn t matter.
        if bus_schedule == "x" {
            continue;
        }

        let b = bus_schedule
            .parse::<i64>()
            .expect(&*format!("Expected '{}' to be an integer.", bus_schedule));
        divisors.push(b);
        remainders.push(b - index as i64);
    }

    // Solve the Chinese Remainder Theorem in order to find the solution.
    chinese_remainder_theorem(&divisors, &remainders)
}

/// Extended Euclid's Algorithm
/// See: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
#[allow(clippy::many_single_char_names)]
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (g, x, y) = egcd(b % a, a);
    (g, y - (b / a) * x, x)
}

/// Calculates the modular multiplicative inverse of x modulo n.
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        return Some((x % n + n) % n);
    }
    None
}

/// Solves the Chinese Remainder Theorem.
/// See: https://en.wikipedia.org/wiki/Chinese_remainder_theorem
///
/// Implementation adapted from: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
pub fn chinese_remainder_theorem(divisors: &[i64], remainders: &[i64]) -> Option<i64> {
    // Calculate the product of the divisors.
    let product = divisors.iter().product::<i64>();

    let mut sum = 0;
    for (&remainder, &divisor) in remainders.iter().zip(divisors) {
        let p = product / divisor;
        sum += remainder * mod_inv(p, divisor)? * p
    }

    Some(sum % product)
}

/// Reads the input for part 1 of the exercise.
pub fn read_input_1<R: BufRead>(reader: &mut R) -> (isize, HashSet<isize>) {
    let arrival_time;
    let mut schedule_times = HashSet::new();

    // Read the expected departure time.
    let mut buffer = String::new();
    reader
        .read_line(&mut buffer)
        .expect("Expected to read input from the reader");

    let line = buffer.trim();

    match line.parse::<isize>() {
        Ok(d) => arrival_time = d,
        Err(e) => panic!("Could not parse departure time '{}': {}", line, e),
    }

    // Read the schedules
    buffer = String::new();
    reader
        .read_line(&mut buffer)
        .expect("Expected to read schedules from the reader");

    for schedule in buffer.trim().split(',') {
        if schedule == "x" {
            continue;
        }
        let schedule_time = schedule
            .parse::<isize>()
            .unwrap_or_else(|e| panic!("Could not parse '{}' as a schedule time: {}", schedule, e));
        schedule_times.insert(schedule_time);
    }

    (arrival_time, schedule_times)
}

pub fn read_input_2<R: BufRead>(reader: &mut R) -> Vec<String> {
    let mut schedules = Vec::new();

    let mut buffer = String::new();
    reader
        .read_line(&mut buffer)
        .expect("Expected to read input from the reader");

    buffer = String::new();
    reader
        .read_line(&mut buffer)
        .expect("Expected to read input form the reader");
    for schedule in buffer.trim().split(',') {
        schedules.push(schedule.to_string());
    }

    schedules
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn get_test_input_reader() -> BufReader<&'static [u8]> {
        let input = "939
7,13,x,x,59,x,31,19";
        return BufReader::new(input.as_bytes());
    }

    #[test]
    fn test_read_input_1() {
        let (arrival_time, schedules) = read_input_1(&mut get_test_input_reader());
        assert_eq!(arrival_time, 939);
        assert_eq!(schedules.len(), 5);
        assert!(schedules.contains(&19));
    }

    #[test]
    fn test_read_input_2() {
        let schedules = read_input_2(&mut get_test_input_reader());
        assert_eq!(schedules.len(), 8);
        assert_eq!(schedules[0], "7");
        assert_eq!(schedules[7], "19");
    }

    #[test]
    fn test_determine_bus_departure_time() {
        let (arrival_time, schedules) = read_input_1(&mut get_test_input_reader());
        let (bus_id, departure_time) = determine_bus_departure_time(&arrival_time, &schedules);
        assert_eq!(bus_id, 59);
        assert_eq!(departure_time, 944);
    }

    #[test]
    fn test_find_earliest_bus_departure_time() {
        assert_eq!(
            find_earliest_bus_departure_time(
                &(vec!["7", "13", "x", "x", "59", "x", "31", "19"])
                    .into_iter()
                    .map(|s| String::from(s))
                    .collect()
            ),
            Some(1068781)
        );
        assert_eq!(
            find_earliest_bus_departure_time(
                &(vec!["17", "x", "13", "19"])
                    .into_iter()
                    .map(|s| String::from(s))
                    .collect()
            ),
            Some(3417)
        );
        assert_eq!(
            find_earliest_bus_departure_time(
                &(vec!["67", "7", "59", "61"])
                    .into_iter()
                    .map(|s| String::from(s))
                    .collect()
            ),
            Some(754018)
        );
        assert_eq!(
            find_earliest_bus_departure_time(
                &(vec!["67", "x", "7", "59", "61"])
                    .into_iter()
                    .map(|s| String::from(s))
                    .collect()
            ),
            Some(779210)
        );
        assert_eq!(
            find_earliest_bus_departure_time(
                &(vec!["67", "7", "x", "59", "61"])
                    .into_iter()
                    .map(|s| String::from(s))
                    .collect()
            ),
            Some(1261476)
        );
        assert_eq!(
            find_earliest_bus_departure_time(
                &(vec!["1789", "37", "47", "1889"])
                    .into_iter()
                    .map(|s| String::from(s))
                    .collect()
            ),
            Some(1202161486)
        );
    }
}
