use aoc_utils::parsing::{into_pos_map_with, Pos};
use std::collections::{HashMap, HashSet};

use crate::dfs::{depth_first_search, depth_first_search_with_paths, Goal, Neighbors};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64> {
    // get the size of the input dynamically.
    let lines: Vec<&str> = input.lines().collect();
    let input_size = (lines.len(), lines[0].len());

    // map each number to a position line,column
    let map: HashMap<u8, HashSet<Pos<usize>>> =
        into_pos_map_with(input, |ch| ch.to_digit(10).map(|d| d as u8));
    // There shouldn't be more than 10 different numbers.
    assert_eq!(10, map.len());

    // loop over the starting positions and return a set of reachable 9s from each 0s
    let history = dfs_for_all(map, input_size);

    // Sum the length of each starting point reachable 9s set.
    let sum = history.values().map(|v| v.len() as u64).sum();

    // Return the sum as result
    Ok(sum)
}

// Define a way to check if the goal is reached for DFS
impl Goal<Vec<&Pos<usize>>> for Pos<usize> {
    // Takes a vector of possible 9s
    fn is_goal(&self, goal: &Vec<&Pos<usize>>) -> bool {
        // if self is contained within the vector of solutions, it is valid
        goal.contains(&self)
    }
}

// Define a way to generate Neighbors for DFS
impl Neighbors<HashMap<u8, HashSet<Pos<usize>>>, (usize, usize)> for Pos<usize> {
    // Takes a graph of nodes and the size of the input.
    // HACK: I should probably extract size from that logic since that's input dependent
    fn get_neighbors(
        &self,
        graph: &HashMap<u8, HashSet<Pos<usize>>>,
        size: &(usize, usize),
    ) -> Vec<Self> {
        // HACK: Not the best logic, but I search for the Key of the current position
        if let Some((curr, _pos)) = graph.iter().find(|(_, p)| p.contains(self)) {
            // Get neighbors of the current position bound by input size
            let neigh: Vec<Pos<usize>> = get_neighbors(*self, *size)
                .to_vec()
                .into_iter()
                // If there are neighbors I returns the ones that have a valid current value + 1
                .filter(|o| {
                    o.is_some_and(|p| graph.get(&(curr + 1)).is_some_and(|hs| hs.contains(&p)))
                })
                .flatten()
                .collect();
            neigh
        } else {
            // if no valid neighbors return an empty vector
            Vec::new()
        }
    }
}

// DFS applied on all trailheads (0)
pub fn dfs_for_all(
    map: HashMap<u8, HashSet<Pos<usize>>>,
    size: (usize, usize),
) -> HashMap<Pos<usize>, HashSet<Vec<Pos<usize>>>> {
    // Create a set of unique stating points
    let trailheads: HashSet<&Pos<usize>> = map.get(&0).unwrap().iter().collect();
    // Create a set of unique ending points
    let goals: Vec<&Pos<usize>> = map.get(&9).unwrap().iter().collect();
    // Prepare an empty set of valid 9s positions for each starting positions
    let mut valid_trails: HashMap<Pos<usize>, HashSet<Vec<Pos<usize>>>> = HashMap::new();

    for head in trailheads {
        let valid_paths = depth_first_search_with_paths(&map, head, &goals, &size);
        // If I can't find a 9, I continue to the next starting point
        if valid_paths.is_empty() {
            continue;
        } else {
            // If I do find one or more valid trails, I add them to the unique solutions set.
            valid_trails.insert(*head, valid_paths);
        }
    }

    // Return the solutions
    valid_trails
}

pub fn get_neighbors(pos: Pos<usize>, size: (usize, usize)) -> [Option<Pos<usize>>; 4] {
    // Return an array of possible neighbors
    // WARN: They are within the input bounds, but may not be the good value at this point
    [
        // Up
        (pos.line > 0).then(|| Pos {
            line: pos.line - 1,
            ..pos
        }),
        // Left
        (pos.column > 0).then(|| Pos {
            column: pos.column - 1,
            ..pos
        }),
        // Right
        (pos.column + 1 < size.1).then(|| Pos {
            column: pos.column + 1,
            ..pos
        }),
        // Down
        (pos.line + 1 < size.0).then(|| Pos {
            line: pos.line + 1,
            ..pos
        }),
    ]
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    const INPUT3: &str = ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";
    const INPUT13: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
    const INPUT227: &str = "012345
123456
234567
345678
4.6789
56789.";

    #[test]
    fn test_process() -> miette::Result<()> {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .try_init();

        assert_eq!(81, process(INPUT)?);
        Ok(())
    }

    #[allow(unused)]
    fn print_pos_map(valid: Vec<Pos<usize>>, x: char) {
        let mut valid = valid;
        valid.sort();
        valid.dedup();

        let input_lines: Vec<_> = INPUT.lines().collect();
        for l in 0..8 {
            eprintln!();
            let mut row = String::new();
            for c in 0..8 {
                if valid.contains(&Pos { line: l, column: c }) {
                    row.push_str(&format!("{x}"));
                    valid.retain(|p| &Pos { line: l, column: c } != p);
                } else {
                    row.push('.');
                }
            }
            row.push(' ');
            if let Some(line) = input_lines.get(l) {
                eprint!("{} ", line);
            }
            eprint!("{row}");
        }
        eprintln!();
    }
}

