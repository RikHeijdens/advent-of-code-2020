use std::collections::{HashMap, HashSet};

pub fn test_adapters(adapters: &[u64]) -> i32 {
    // Sort the list of adapters on rating in ascending order.
    let mut sorted_adapters = adapters.to_owned();
    sorted_adapters.sort_unstable();

    // Determine the effective jolt rate of the device adapter.
    // The rating of the device adapter is 3 jolts higher than the maximum in `adapters`.
    sorted_adapters.push(sorted_adapters.last().unwrap_or(&0) + 3);

    // Find a chain that uses all adapters.
    let mut effective_jolt_rating = 0;

    // Keep track of the difference counts in the jolt_differences map.
    let mut jolt_differences: HashMap<u8, i32> = HashMap::new();

    // Iterate over the sorted list of adapters and keep track of the differences in ratings.
    for adapter in sorted_adapters.iter() {
        let difference: u8 = (adapter - effective_jolt_rating) as u8;
        let entry = jolt_differences.entry(difference).or_insert(0);
        *entry += 1;
        effective_jolt_rating = *adapter;
    }

    jolt_differences[&1] * jolt_differences[&3]
}

fn find_permutations_reverse(current_index: usize, adapters: &[u64], cache: &mut [u64]) -> u64 {
    let mut num_permutations = 0;

    // Check if we've already computed the number of permutations for this sub solution.
    if current_index > 0 && current_index < adapters.len() && cache[current_index] > 0 {
        return cache[current_index];
    }

    // Check if *this* adapter plugs into the charging outlet.
    let start_adapter = adapters[current_index];
    if start_adapter <= 3 {
        // This adapter plugs straight into the charging outlet, which means that we found
        // an end of the chain.
        num_permutations += 1;
    }

    // Check if there are any additional adapters that we can connect to.
    if current_index < adapters.len() - 1 {
        // There are more adapters that we could connect to.
        for next_index in current_index + 1..adapters.len() {
            if start_adapter - adapters[next_index] > 3 {
                // The step to the next adapter is too large.
                break;
            }
            let n = find_permutations_reverse(next_index, adapters, cache);
            // Cache the solution for 'next_index', this is critical if we don't do this the
            // solution won't scale to the full problem.
            cache[next_index] = n;
            num_permutations += n
        }
    }
    num_permutations
}

/// Finds the total number of adapter arrangements by working backwards from the device.
/// This solution uses memoization in order to ensure that the problem can be solved.
pub fn find_adapter_arrangements(adapters: &[u64]) -> u64 {
    // Sort the last of adapters to simplify graph construction.
    let mut sorted_adapters = adapters.to_owned();
    sorted_adapters.sort_unstable();
    sorted_adapters.reverse();

    // Create a cache to memoize answers to sub-solutions in.
    let mut cache = vec![0; adapters.len()];
    find_permutations_reverse(0, &sorted_adapters, &mut cache)
}

/// Computes the total number of adapter arrangements by modeling the adapters as vertices in a
/// graph, and using the maximum difference of 3 constraint to define the edges.
///
/// The task is then solved by counting all possible paths from any of the valid start nodes
/// to the end_node.
///
/// Note that this solution only works as expected on the test input, and doesn't scale to the
/// full problem.
pub fn find_adapter_arrangements_graph(adapters: &[u64]) -> u64 {
    // Sort the last of adapters to simplify graph construction.
    let mut sorted_adapters = adapters.to_owned();
    sorted_adapters.sort_unstable();

    // Construct a directed graph of adapters that are allowed to connect to each other.
    let mut edges: HashMap<u64, Vec<(u64, u64)>> = HashMap::new();
    for (idx, a1) in sorted_adapters.iter().enumerate() {
        for a2 in sorted_adapters[idx + 1..].iter() {
            if a2 - a1 <= 3 {
                // a1 connects to a2
                let entry = edges.entry(*a1).or_insert(Vec::new());
                entry.push((*a1, *a2));
            }
        }
    }
    // Determine the possible starting adapters, i.e. our "start nodes".
    let mut start_adapters: Vec<u64> = Vec::new();
    for adapter in sorted_adapters.iter() {
        if *adapter > 3 {
            break;
        }
        start_adapters.push(*adapter)
    }

    // Determine the end node, i.e. our chains should end with this value.
    let end_node = sorted_adapters.last().unwrap();

    // Find all possible paths from any start_node to end_node.
    let mut arrangements: u64 = 0;
    for start_node in start_adapters.iter() {
        // Perform a depth-first search and memoize valid paths
        let mut visited_nodes: HashSet<u64> = HashSet::new();
        arrangements += depth_first_search(&edges, &mut visited_nodes, start_node, end_node)
    }

    arrangements
}

fn depth_first_search(
    edges: &HashMap<u64, Vec<(u64, u64)>>,
    visited_nodes: &mut HashSet<u64>,
    start_node: &u64,
    end_node: &u64,
) -> u64 {
    let mut path_count = 0;
    if visited_nodes.contains(start_node) {
        // Already visited this node. Stop searching.
        return 0;
    }
    visited_nodes.insert(*start_node);

    if start_node == end_node {
        // Found a new path.
        visited_nodes.remove(start_node);
        return 1;
    }

    // Check if this start_node has any neighbours.
    if let Some(neighbours) = edges.get(start_node) {
        // Recursively search using the neighbours.
        for neighbour in neighbours.iter() {
            path_count += depth_first_search(edges, visited_nodes, &neighbour.1, end_node);
        }
    }
    visited_nodes.remove(start_node);
    path_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_9::read_input;
    use std::io::BufReader;

    fn get_test_input() -> Vec<u64> {
        let adapters = "16
10
15
5
1
11
7
19
6
12
4";
        read_input(&mut BufReader::new(adapters.as_bytes()))
    }

    #[test]
    fn test_test_adapters() {
        let ratings = get_test_input();
        assert_eq!(test_adapters(&ratings), 7 * 5);
    }

    #[test]
    fn test_find_adapter_arrangements() {
        let ratings = get_test_input();
        assert_eq!(find_adapter_arrangements(&ratings), 8);
    }

    #[test]
    fn test_find_adapter_arrangements_graph() {
        let ratings = get_test_input();
        assert_eq!(find_adapter_arrangements_graph(&ratings), 8);
    }

    #[test]
    fn test_find_adapter_arrangements_2() {
        let adapters = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let ratings = read_input(&mut BufReader::new(adapters.as_bytes()));
        assert_eq!(find_adapter_arrangements(&ratings), 19208);
        assert_eq!(find_adapter_arrangements_graph(&ratings), 19208);
    }
}
