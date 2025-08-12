use std::iter::Enumerate;

static INPUT_D4: &str =
    include_str!("/Volumes/Dock/Dev/Rust/projects/advent_of_code/2024/assets/day_4/input.txt");

pub fn day_4() {
    day_4_part_1(INPUT_D4);

    day_4_part_2(INPUT_D4);
}

fn day_4_part_2(input: &str) -> u64 {
    let (letter_grid, first_letter) = get_first_letter_from_input(input, &'M');
    find_word_2(&letter_grid, &first_letter, &vec!['A', 'S'])
}

fn day_4_part_1(input: &str) -> u64 {
    // for each position map surrounding_letters
    //
    //       M S A    1 2 3
    //  MSA   \|/      \|/     (-1,-1), (-1,0), (-1,1)
    //  AMX  A-M-X    4-M-5    ( 0,-1), (0, 0), ( 0,1)
    //  MSA   /|\      /|\     ( 1,-1), ( 1,0), ( 1,1)
    //       M S A    6 7 8
    //
    let (letter_grid, first_letter) = get_first_letter_from_input(input, &'X');
    let result = find_word_1(letter_grid, first_letter);
    result
}

fn find_word_1(letter_grid: Vec<Letter>, x: Vec<Letter>) -> u64 {
    let xmas: Vec<Letter> = x
        .iter()
        .flat_map(|lx| {
            lx.get(&'M')
                .iter()
                .filter_map(|lxm| {
                    let dir = lxm;
                    lxm.cell().map(|cell| (dir, cell))
                })
                .filter_map(|(dir, cell)| {
                    letter_grid
                        .iter()
                        .find(|l| l.letter.position == cell.position)
                        .copied()
                        .map(|letter| (dir.to_owned(), letter))
                })
                .collect::<Vec<(Direction, Letter)>>()
        })
        .flat_map(|(dir, lxm)| {
            lxm.check_in_direction(&dir, 'A')
                .iter()
                .map(|cell| (dir, cell))
                .filter_map(|(dir, cell)| {
                    letter_grid
                        .iter()
                        .find(|l| l.letter.position == cell.position)
                        .copied()
                        .map(|letter| (dir.to_owned(), letter))
                })
                .collect::<Vec<(Direction, Letter)>>()
        })
        .flat_map(|(dir, lxma)| {
            lxma.check_in_direction(&dir, 'S')
                .iter()
                .map(|cell| (dir, cell))
                .filter_map(|(_dir, cell)| {
                    letter_grid
                        .iter()
                        .find(|l| l.letter.position == cell.position)
                        .copied()
                })
                .collect::<Vec<Letter>>()
        })
        .collect();
    xmas.len() as u64
}
fn find_word_2(letter_grid: &[Letter], first_letters: &[Letter], letters: &[char]) -> u64 {
    assert!(letters.len() >= 2, "need at least two letters");

    // TODO: Il faut eliminer les croix verticales,
    let word: Vec<Mas> = first_letters
        .iter()
        .flat_map(|&lm| {
            lm.get_cross(&letters[0])
                .into_iter()
                .filter_map(|dir| dir.cell().map(|cell| (dir, cell)))
                .filter_map(move |(dir, cell)| {
                    letter_grid
                        .iter()
                        .find(|l| l.letter.position == cell.position)
                        .copied()
                        .map(|a_letter| {
                            (
                                dir,
                                Mas {
                                    m: lm,
                                    a: Some(a_letter),
                                    s: None,
                                },
                            )
                        })
                })
        })
        .flat_map(|(dir, lma)| {
            // If we have an A, look in the same `dir` for the S (letters[1])
            lma.a
                .into_iter() // Option<Letter> -> iterator of 0 or 1 item
                .filter_map(move |a_letter| {
                    a_letter
                        .check_in_direction(&dir, letters[1]) // Option<Cell>
                        .and_then(|cell| {
                            letter_grid
                                .iter()
                                .find(|l| l.letter.position == cell.position) // Option<&Letter>
                                .copied()
                        })
                        .map(|s_letter| Mas {
                            m: lma.m,
                            a: Some(a_letter),
                            s: Some(s_letter),
                        })
                })
        })
        .collect();
    let result = keep_with_common_a(&word);
    dbg!(result.len() as u64 / 2)
}
use std::collections::BTreeMap;

fn keep_with_common_a<'a>(items: &'a [Mas]) -> Vec<&'a Mas> {
    // 1) Count how many times each `a` appears (key = a.letter.position)
    let mut counts: BTreeMap<Position, usize> = BTreeMap::new();
    for m in items {
        if let Some(a) = m.a {
            *counts.entry(a.letter.position).or_default() += 1;
        }
    }

    // 2) Keep only those with a present AND count >= 2
    items
        .iter()
        .filter(|m| {
            m.a.map(|a| counts.get(&a.letter.position).copied().unwrap_or(0) == 2)
                .unwrap_or(false)
        })
        .collect()
}

fn get_first_letter_from_input(input: &str, char: &char) -> (Vec<Letter>, Vec<Letter>) {
    let grid = map_grid(input);
    let letter_grid: Vec<Letter> = cells_to_letters(&grid);
    let letter: Vec<Letter> = letter_grid
        .iter()
        .filter(|l| &l.letter.char == char)
        .copied()
        .collect();
    (letter_grid, letter)
}

fn from_position(grid: &Vec<Cell>, position: Position) -> Option<Cell> {
    grid.into_iter().find(|c| c.position == position).copied()
}

fn cell_to_letter(cell: &Cell, grid: Vec<&Letter>) -> Letter {
    grid.iter()
        .find(|l| l.letter.position == cell.position)
        .copied()
        .copied()
        .unwrap()
}
fn cells_to_letters(grid: &Vec<Cell>) -> Vec<Letter> {
    grid.iter()
        .map(|cell| Letter {
            letter: cell.to_owned(),
            up_left: Direction::UpLeft(from_position(
                grid,
                Position {
                    line: cell.position.line - 1,
                    column: cell.position.column - 1,
                },
            )),
            up: Direction::Up(from_position(
                grid,
                Position {
                    line: cell.position.line - 1,
                    column: cell.position.column,
                },
            )),
            up_right: Direction::UpRight(from_position(
                grid,
                Position {
                    line: cell.position.line - 1,
                    column: cell.position.column + 1,
                },
            )),
            left: Direction::Left(from_position(
                grid,
                Position {
                    line: cell.position.line,
                    column: cell.position.column - 1,
                },
            )),
            right: Direction::Right(from_position(
                grid,
                Position {
                    line: cell.position.line,
                    column: cell.position.column + 1,
                },
            )),
            down_left: Direction::DownLeft(from_position(
                grid,
                Position {
                    line: cell.position.line + 1,
                    column: cell.position.column - 1,
                },
            )),
            down: Direction::Down(from_position(
                grid,
                Position {
                    line: cell.position.line + 1,
                    column: cell.position.column,
                },
            )),
            down_right: Direction::DownRight(from_position(
                grid,
                Position {
                    line: cell.position.line + 1,
                    column: cell.position.column + 1,
                },
            )),
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Letter {
    letter: Cell,
    up_left: Direction,
    up: Direction,
    up_right: Direction,
    left: Direction,
    right: Direction,
    down_left: Direction,
    down: Direction,
    down_right: Direction,
}

impl Letter {
    pub fn get(&self, ch: &char) -> Vec<Direction> {
        [
            self.up_left,
            self.up,
            self.up_right,
            self.left,
            self.right,
            self.down_left,
            self.down,
            self.down_right,
        ]
        .into_iter()
        .filter_map(|dir| dir.cell().filter(|c| &c.char == ch).map(|_| dir))
        .collect()
    }
    pub fn get_cross(&self, ch: &char) -> Vec<Direction> {
        [self.up_left, self.up_right, self.down_left, self.down_right]
            .into_iter()
            .filter_map(|dir| dir.cell().filter(|c| &c.char == ch).map(|_| dir))
            .collect()
    }
    pub fn check_in_direction(&self, dir: &Direction, target: char) -> Option<Cell> {
        match dir {
            Direction::UpLeft(_) => self.up_left.cell().filter(|c| c.char == target),
            Direction::Up(_) => self.up.cell().filter(|c| c.char == target),
            Direction::UpRight(_) => self.up_right.cell().filter(|c| c.char == target),
            Direction::Left(_) => self.left.cell().filter(|c| c.char == target),
            Direction::Right(_) => self.right.cell().filter(|c| c.char == target),
            Direction::DownLeft(_) => self.down_left.cell().filter(|c| c.char == target),
            Direction::Down(_) => self.down.cell().filter(|c| c.char == target),
            Direction::DownRight(_) => self.down_right.cell().filter(|c| c.char == target),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    line: i32,
    column: i32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Cell {
    position: Position,
    char: char,
}

#[derive(Debug, PartialEq, Eq)]
struct Mas {
    m: Letter,
    a: Option<Letter>,
    s: Option<Letter>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    UpLeft(Option<Cell>),
    Up(Option<Cell>),
    UpRight(Option<Cell>),
    Left(Option<Cell>),
    Right(Option<Cell>),
    DownLeft(Option<Cell>),
    Down(Option<Cell>),
    DownRight(Option<Cell>),
}
impl Direction {
    // Borrowing version: Option<&Cell>
    fn cell_ref(&self) -> Option<&Cell> {
        match self {
            Direction::UpLeft(o)
            | Direction::Up(o)
            | Direction::UpRight(o)
            | Direction::Left(o)
            | Direction::Right(o)
            | Direction::DownLeft(o)
            | Direction::Down(o)
            | Direction::DownRight(o) => o.as_ref(),
        }
    }

    // By-value version: Option<Cell> (works because Cell: Copy)
    fn cell(self) -> Option<Cell> {
        match self {
            Direction::UpLeft(o)
            | Direction::Up(o)
            | Direction::UpRight(o)
            | Direction::Left(o)
            | Direction::Right(o)
            | Direction::DownLeft(o)
            | Direction::Down(o)
            | Direction::DownRight(o) => o,
        }
    }
}

fn map_grid(input: &str) -> Vec<Cell> {
    let grid: Vec<Cell> = input
        .lines()
        .enumerate()
        .take(140)
        .flat_map(|(ln, l)| {
            l.chars().enumerate().map(move |(cn, c)| Cell {
                position: Position {
                    line: ln as i32,
                    column: cn as i32,
                },
                char: c,
            })
        })
        .collect::<Vec<Cell>>();
    grid
}

mod test {
    use super::*;

    const INPUT_D4_TEST: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_xmas() {
        assert_eq!(day_4_part_1(INPUT_D4), 2654);
    }
    #[test]
    fn test_xmas_2() {
        assert_eq!(day_4_part_2(INPUT_D4_TEST), 9);
    }
    #[test]
    fn test_map_grid() {
        let grid = dbg!(map_grid(INPUT_D4_TEST));
        assert_eq!(
            grid.first().unwrap(),
            &Cell {
                position: Position { line: 0, column: 0 },
                char: 'M'
            }
        );
        assert_eq!(
            grid.last().unwrap(),
            &Cell {
                position: Position { line: 9, column: 9 },
                char: 'X'
            }
        );
    }
}
