#![allow(unused)]
use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map_res,
    error::Error,
    multi::{many1, separated_list1},
    Parser,
};

use crate::part1::*;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let map = parse_map_2(input);

    let mut pos_map: PosDir = map
        .into_iter()
        .enumerate()
        .flat_map(|(li, row)| {
            row.into_iter()
                .enumerate()
                .map(move |(ci, state)| ([li as isize, ci as isize], state))
        })
        .collect();
    trace_with_directions(&mut pos_map);

    let steps = get_steps(&mut pos_map);
    let map = parse_map_2(input);
    let mut pos_map: PosDir = map
        .into_iter()
        .enumerate()
        .flat_map(|(li, row)| {
            row.into_iter()
                .enumerate()
                .map(move |(ci, state)| ([li as isize, ci as isize], state))
        })
        .collect();

    search_blocks(&mut pos_map, &steps);
    print_map(&pos_map);
    println!();

    println!("{}", steps.len());

    Ok("Testing".to_string())
}
pub fn parse_map_2(input: &str) -> Vec<Vec<StateHistory>> {
    let (_rest, map): (&str, Vec<Vec<StateHistory>>) = separated_list1(
        newline::<&str, Error<&str>>,
        many1(map_res(
            alt((
                char('.'),
                char('#'),
                char('^'),
                char('>'),
                char('v'),
                char('<'),
            )),
            |c| Ok::<StateHistory, Error<&str>>(StateHistory::from(c)),
        )),
    )
    .parse(input)
    .unwrap();
    map
}

impl From<char> for StateHistory {
    fn from(value: char) -> Self {
        use Dir::*;
        use StateHistory::*;
        match value {
            '.' => GroundHistory(vec![]),
            '#' => Wall,
            '^' => Guard(Up),
            '>' => Guard(Right),
            'v' => Guard(Down),
            '<' => Guard(Left),
            e => unreachable!("found: {e}"),
        }
    }
}

pub fn trace_with_directions(pos_map: &mut PosDir) {
    let (mut pos, mut dir) = get_guard_pos_dir(pos_map);

    search_path(pos_map, pos, dir);
}

fn search_blocks(pos_map: &mut HashMap<[isize; 2], StateHistory>, steps: &[[isize; 2]]) {
    let (start_pos, start_dir) = get_guard_pos_dir(pos_map);
    dbg!(start_pos, start_dir);
    let mut count = 0;

    for &obs in steps {
        if obs == start_pos {
            continue;
        };
        let mut try_map = pos_map.clone();
        try_map.insert(obs, StateHistory::Wall);
        let mut seen = HashSet::new();
        let mut dir = start_dir;
        let mut pos = start_pos;
        'inner: loop {
            if !seen.insert((pos, dir)) {
                count += 1;
                break 'inner;
            }
            // Mark the current tile as visited ground
            mark_current_pos(&mut try_map, pos, dir);

            // Compute the next position
            let [dr, dc] = dir.delta();
            let next = [pos[0] + dr, pos[1] + dc];

            // Peek at what's ahead without holding a borrow
            let Some(next_state) = try_map.get_mut(&next) else {
                // walking off the known map ends the trace
                break 'inner;
            };

            // eprintln!("pos: {pos:?}, next: {next:?}, next_state: {next_state:?}");
            match next_state {
                StateHistory::Wall => {
                    dir = turn_right(dir);
                    continue 'inner;
                }
                _ => {
                    pos = next;
                }
            }
        }
    }
    dbg!(count);
}

fn get_steps(pos_map: &mut HashMap<[isize; 2], StateHistory>) -> Vec<[isize; 2]> {
    let mut steps: Vec<[isize; 2]> = pos_map
        .iter_mut()
        .filter_map(|(i, s)| {
            if matches!(&s, StateHistory::GroundHistory(dirs) if !dirs.is_empty()) {
                Some(i)
            } else {
                None
            }
        })
        .copied()
        .collect();
    steps
}

fn search_path(pos_map: &mut HashMap<[isize; 2], StateHistory>, mut pos: [isize; 2], mut dir: Dir) {
    loop {
        // Mark the current tile as visited ground
        mark_current_pos(pos_map, pos, dir);

        // Compute the next position
        let [dr, dc] = dir.delta();
        let next = [pos[0] + dr, pos[1] + dc];

        // Peek at what's ahead without holding a borrow
        let Some(next_state) = pos_map.get_mut(&next) else {
            // walking off the known map ends the trace
            break;
        };

        // eprintln!("pos: {pos:?}, next: {next:?}, next_state: {next_state:?}");
        match next_state {
            StateHistory::Wall => {
                dir = turn_right(dir);
                continue;
            }
            StateHistory::GroundHistory(ref mut v) => {
                v.push(dir);
                pos = next;
            }
            StateHistory::Guard(g) => break,
        }
    }
}

fn get_guard_pos_dir(pos_map: &mut HashMap<[isize; 2], StateHistory>) -> ([isize; 2], Dir) {
    let (mut pos, mut dir) = pos_map
        .iter()
        .find_map(|(p, s)| match *s {
            StateHistory::Guard(d) => Some((*p, d)),
            _ => None,
        })
        .expect("no guard found");
    (pos, dir)
}

fn mark_current_pos(pos_map: &mut HashMap<[isize; 2], StateHistory>, pos: [isize; 2], dir: Dir) {
    pos_map
        .entry(pos)
        .and_modify(|sh| match sh {
            StateHistory::Wall => {}
            StateHistory::GroundHistory(dirs) => dirs.push(dir),
            StateHistory::Guard(dir) => {
                *sh = StateHistory::GroundHistory(vec![*dir]);
            }
        })
        .or_insert(StateHistory::GroundHistory(vec![dir]));
}

fn print_map(pos_map: &PosDir) {
    for l in 0..130 {
        for c in 0..130 {
            if c == 0 {
                println!();
            }
            let pos = [l as isize, c as isize];
            let ch = match pos_map.get(&pos).unwrap() {
                StateHistory::Wall => '#',
                StateHistory::Guard(dir) => match dir {
                    Dir::Up => '▲',
                    Dir::Right => '▶',
                    Dir::Down => '▼',
                    Dir::Left => '◀',
                },
                _ => glyph_for_cell(pos_map, pos),
            };
            print!("{ch}");
        }
    }
}

pub type PosDir = HashMap<[isize; 2], StateHistory>;

#[inline]
fn opposite(d: Dir) -> Dir {
    match d {
        Dir::Up => Dir::Down,
        Dir::Right => Dir::Left,
        Dir::Down => Dir::Up,
        Dir::Left => Dir::Right,
    }
}

#[inline]
fn delta_of(d: Dir) -> (isize, isize) {
    match d {
        Dir::Up => (-1, 0),
        Dir::Right => (0, 1),
        Dir::Down => (1, 0),
        Dir::Left => (0, -1),
    }
}

fn cell_has_heading(sh: &StateHistory, d: Dir) -> bool {
    match sh {
        StateHistory::GroundHistory(v) => v.contains(&d),
        StateHistory::Guard(gd) => *gd == d,
        StateHistory::Wall => false,
    }
}

fn is_connected(pos_map: &PosDir, pos: [isize; 2], d: Dir) -> bool {
    let here = match pos_map.get(&pos) {
        Some(s) => s,
        None => return false,
    };
    let (dr, dc) = delta_of(d);
    let npos = [pos[0] + dr, pos[1] + dc];
    let there = match pos_map.get(&npos) {
        Some(s) => s,
        None => return false,
    };
    if matches!(there, StateHistory::Wall) {
        return false;
    }
    // Edge exists if either side recorded movement along this edge
    cell_has_heading(here, d) || cell_has_heading(there, opposite(d))
}

pub fn glyph_for_cell(pos_map: &PosDir, pos: [isize; 2]) -> char {
    let u = is_connected(pos_map, pos, Dir::Up);
    let r = is_connected(pos_map, pos, Dir::Right);
    let d = is_connected(pos_map, pos, Dir::Down);
    let l = is_connected(pos_map, pos, Dir::Left);

    let mask = (u as u8) | (r as u8) << 1 | (d as u8) << 2 | (l as u8) << 3;

    match mask {
        0b0000 => '.',
        0b0001 | 0b0100 | 0b0101 => '│',
        0b0010 | 0b1000 | 0b1010 => '─',

        0b0011 => '└',
        0b0110 => '┌',
        0b1100 => '┐',
        0b1001 => '┘',

        // tees (mising one side):
        0b1110 => '┬', // no Up
        0b1101 => '┤', // no Right
        0b1011 => '┴', // no Down
        0b0111 => '├', // no Left

        0b1111 => '┼', // full crossing
        _ => '?',
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StateHistory {
    Wall,
    GroundHistory(Vec<Dir>),
    Guard(Dir),
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
