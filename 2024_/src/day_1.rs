// 1. Read from the file into a vector of tupples
// 2. Zip first elements together and second elements together
// 3. Sort lists
// 4. Add elements of both lists by index and store in result list
// 5. Return list total.

use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

pub fn file_to_tupples(path: PathBuf) -> Vec<(u32, u32)> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let mut split_line = line.split_whitespace();
            (
                split_line.next().unwrap().parse::<u32>().unwrap(),
                split_line.last().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect()
}

pub fn left_right_list(list: Vec<(u32, u32)>) -> (Vec<u32>, Vec<u32>) {
    let (mut left, mut right): (Vec<_>, Vec<_>) = list.into_iter().unzip();
    left.sort();
    right.sort();
    (left, right)
}

pub fn weighted_sum(left: Vec<u32>, right: Vec<u32>) -> u32 {
    let mut count_map: HashMap<u32, u32> = HashMap::new();
    for n in right {
        *count_map.entry(n).or_insert(0) += 1;
    }
    let result: u32 = left
        .iter()
        .map(|n| n * count_map.get(n).unwrap_or(&0))
        .sum();
    result
}
