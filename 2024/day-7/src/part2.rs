use core::fmt;
use std::collections::{HashMap, HashSet, VecDeque};

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

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Key {
    depth: usize,
    res: u64,
}

pub fn chech_lines(input: Vec<(u64, Vec<u64>)>) -> u64 {
    dbg!(
        input
            .iter()
            .filter_map(|(goal, nums)| check_line(*goal, nums).then_some(goal))
            .sum()
    )
}

// Remontada
pub fn remontada(start: &Key, parents: HashMap<Key, Option<(Key, Op)>>) -> Vec<Op> {
    // 1. New Vec<node>
    let mut path = Vec::with_capacity(start.depth);
    // 2. Add solution node
    let mut next = start.to_owned();
    // 3. While has parent, add parent and follow
    while let Some(Some((parent, op))) = parents.get(&next).copied() {
        next = parent;
        path.push(op);
    }

    path.reverse();
    println!(
        "{:>20}:\t {}",
        start.res,
        path.iter().map(|op| op.to_string()).collect::<String>()
    );
    path
}

pub fn check_line(goal: u64, nums: &[u64]) -> bool {
    if let Some((first, rest)) = nums.split_first() {
        let start = Key {
            depth: 0,
            res: *first,
        };
        let goal_key = Key {
            depth: nums.len() - 1,
            res: goal,
        };

        // HACK: These early return create false negatives.
        // match (&goal, first, rest) {
        //     // first is alone and goal
        //     (g, f, r) if g == f && r.is_empty() => return true,
        //     // first too small
        //     (g, f, _) if g < f => return false,
        //     //     // This is not true! 1+1 > 1*1
        //     //     (g, f, r) if g < &f.add(r.iter().sum::<u64>()) => return false,
        //     //     // This is not true! 2 > 1*1
        //     //     (g, f, r) if g > &f.mul(r.iter().product::<u64>()) => return false,
        //     (_, _, _) => (),
        // };

        // if there is at least one solution, true
        bfs(start, rest, goal_key).is_some()
    } else {
        // if nums is empty returns false
        unreachable!("There shouldn't be an empty list of nums: {nums:?}");
    }
}

pub fn bfs(start: Key, rest: &[u64], goal: Key) -> Option<Vec<Op>> {
    let mut q = VecDeque::<Key>::new();
    let mut seen = HashSet::new();
    let mut parents = HashMap::new();

    // initial state
    let cur = start;
    q.push_back(cur);
    seen.insert(cur);
    // Batman
    parents.insert(cur, None);

    //as long as q yields
    while let Some(cur) = q.pop_front() {
        if cur == goal {
            return Some(remontada(&cur, parents));
        }
        if let Some(next_i) = rest.get(cur.depth) {
            let add_neigh = Key {
                depth: cur.depth + 1,
                res: cur.res + next_i,
            };
            let mul_neigh = Key {
                depth: cur.depth + 1,
                res: cur.res * next_i,
            };
            let con_neigh = Key {
                depth: cur.depth + 1,
                res: (format!("{}{}", cur.res, next_i)
                    .parse::<u64>()
                    .expect("coulnd not parse concatenated string")),
            };
            // add neighbors to q
            q.push_back(add_neigh);
            q.push_back(mul_neigh);
            q.push_back(con_neigh);
            // add neighbors to seen
            seen.insert(add_neigh);
            seen.insert(mul_neigh);
            seen.insert(con_neigh);
            // update neighbors parents
            parents.insert(add_neigh, Some((cur, Op::Add)));
            parents.insert(mul_neigh, Some((cur, Op::Mul)));
            parents.insert(con_neigh, Some((cur, Op::Con)));
        }
    }
    // q is empty and goal not found
    println!("{:>20}: \t ❌", goal.res);
    None
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub enum Op {
    Add,
    Mul,
    Con,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "➕"),
            Op::Mul => write!(f, "✖️"),
            Op::Con => write!(f, "➰"),
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
        assert_eq!(process(INPUT_TEST)?, "11387");
        Ok(())
    }

    #[test]
    fn test_parse() -> miette::Result<()> {
        let input = "3267: 81 40 27";
        assert_eq!(parse(input), Ok(("", vec![(3267, vec![81, 40, 27])])));
        Ok(())
    }
}
