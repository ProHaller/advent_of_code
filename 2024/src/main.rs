mod day_1;
mod day_2;
use std::fs::read_to_string;
use std::path::PathBuf;

use day_1::day_1::*;
use day_2::day_2::*;

fn main() {
    let path = PathBuf::from("/Volumes/Dock/Dev/Rust/projects/advent/assets/day_2/full.txt");
    if let Ok(input) = read_to_string(path) {
        let result = part_2(&input);
        println!("Result = {result}");
    }
}
