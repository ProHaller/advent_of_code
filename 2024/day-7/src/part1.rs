use crate::bfs::*;
use core::fmt;

use miette::miette;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, digit1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, parsed_input) = parse(input).map_err(|e| miette!("Failed to parse input: {}", e))?;
    let result = chech_lines(parsed_input);
    Ok(result.to_string())
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub enum Op {
    Add,
    Mul,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "➕"),
            Op::Mul => write!(f, "✖️"),
        }
    }
}

fn is_valid_line(line: (u64, Vec<u64>)) -> Option<Vec<Op>> {
    let (target, numbers) = line;
    dbg!(match (target, &numbers) {
        (t, n) if n.iter().sum::<u64>() == t => Some(vec![Op::Add; numbers.len()]),
        (t, n) if n.iter().product::<u64>() == t => Some(vec![Op::Mul; numbers.len()]),
        (t, n) if n.iter().sum::<u64>() < t => None,
        (t, n) if n.iter().product::<u64>() > t => None,
        (t, n) => try_ops(t, n),
    })
}

fn try_ops(t: u64, n: &[u64]) -> Option<Vec<Op>> {
    dbg!(
        list_res(n)
            .iter()
            .find(|(r, _ops)| r == &t)
            .map(|ops| ops.1.clone())
    )
}

fn list_res(input: &[u64]) -> Vec<(u64, Vec<Op>)> {
    let res = input
        .windows(2)
        .fold(Vec::<(u64, Vec<Op>)>::new(), |mut acc, window| {
            if let [a, b] = window {
                acc.extend(add_and_mul(*a, *b));
            }
            acc
        });
    println!("{:?}", res);
    res
}

fn add_and_mul(a: u64, b: u64) -> Vec<(u64, Vec<Op>)> {
    let res = vec![(a + b, vec![Op::Add]), (a * b, vec![Op::Mul])];
    println!("{:?}", &res);
    res
}

pub fn parse(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    let parsed = separated_list1(
        newline,
        separated_pair(
            map_res(digit1, |s: &str| s.parse::<u64>()),
            tag(": "),
            separated_list1(char(' '), map_res(digit1, |s: &str| s.parse::<u64>())),
        ),
    )
    .parse(input)?;
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!(process(INPUT_TEST)?, "3749");
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let input = "3267: 81 40 27";
        assert_eq!(parse(input), Ok(("", vec![(3267, vec![81, 40, 27])])));
        Ok(())
    }
}
