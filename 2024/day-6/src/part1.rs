use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map_res,
    error::Error,
    multi::{many1, separated_list1},
    Parser,
};

pub type Pos = HashMap<[isize; 2], State>;
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let map = parse_map(input);

    let mut pos_map: Pos = map
        .into_iter()
        .enumerate()
        .flat_map(|(li, row)| {
            row.into_iter()
                .enumerate()
                .map(move |(ci, state)| ([li as isize, ci as isize], state))
        })
        .collect();
    trace(&mut pos_map);

    let sum: usize = pos_map
        .iter()
        .filter(|(vpos, state)| state.ground_is_true())
        .count();

    Ok(sum.to_string())
}

pub fn trace(pos_map: &mut Pos) {
    // 1) Find starting (pos, dir) without keeping any &mut alive
    let (mut pos, mut dir) = pos_map
        .iter()
        .find_map(|(p, s)| match *s {
            State::Guard(d) => Some((*p, d)),
            _ => None,
        })
        .expect("no guard found");

    loop {
        // Mark the current tile as visited ground
        pos_map.insert(pos, State::Ground(true));

        // Compute the next position
        let [dr, dc] = dir.delta(); // implement as in previous message
        let next = [pos[0] + dr, pos[1] + dc];

        // Peek at what's ahead without holding a borrow
        let Some(next_state) = pos_map.get(&next).copied() else {
            // walking off the known map ends the trace
            break;
        };

        match next_state {
            State::Wall => {
                // turn and try again
                dir = turn_right(dir); // e.g., via ORDER array
                continue;
            }
            State::Ground(_) => {
                // move the guard into `next`
                pos_map.insert(next, State::Guard(dir));
                pos = next;
            }
            State::Guard(_) => unreachable!("expected only one guard"),
        }
    }
}

impl Dir {
    #[inline]
    fn delta(self) -> [isize; 2] {
        match self {
            Dir::Up => [-1, 0],
            Dir::Right => [0, 1],
            Dir::Down => [1, 0],
            Dir::Left => [0, -1],
        }
    }
}

pub const ORDER: [Dir; 4] = [Dir::Up, Dir::Right, Dir::Down, Dir::Left];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Wall,
    Ground(bool),
    Guard(Dir),
}

impl State {
    pub fn step(&mut self, front: &State) -> Option<Self> {
        if !matches!(self, State::Guard(_)) {
            panic!();
        }
        let next = *self;

        match front {
            State::Wall => None,
            State::Ground(_) => {
                *self = State::Ground(true);
                Some(next)
            }
            _ => unreachable!(),
        }
    }
    pub fn ground_is_true(&self) -> bool {
        match self {
            State::Wall => false,
            State::Ground(true) => true,
            State::Ground(false) => false,
            State::Guard(_) => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[inline]
fn turn_right(d: Dir) -> Dir {
    ORDER[(ORDER.iter().position(|x| *x == d).unwrap() + 1) & 3]
}

impl From<char> for State {
    fn from(value: char) -> Self {
        use Dir::*;
        use State::*;
        match value {
            '.' => Ground(false),
            '#' => Wall,
            '^' => Guard(Up),
            '>' => Guard(Right),
            'v' => Guard(Down),
            '<' => Guard(Left),
            e => unreachable!("found: {e}"),
        }
    }
}

pub fn parse_map(input: &str) -> Vec<Vec<State>> {
    let (_rest, map): (&str, Vec<Vec<State>>) = separated_list1(
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
            |c| Ok::<State, Error<&str>>(State::from(c)),
        )),
    )
    .parse(input)
    .unwrap();
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::part1::State::Guard;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = TEST_INPUT;
        assert_eq!(process(input)?, "41");
        Ok(())
    }

    #[test]
    fn test_grid() -> miette::Result<()> {
        let input = TEST_INPUT;
        assert_eq!(parse_map(input)[6][4], Guard(Dir::Up));
        Ok(())
    }

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
}
