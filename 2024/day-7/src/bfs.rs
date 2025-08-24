use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    iter::Sum,
    ops::{Add, Mul},
};

use crate::part1::Op;
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
            // add neighbors to q
            q.push_back(add_neigh);
            q.push_back(mul_neigh);
            // add neighbors to seen
            seen.insert(add_neigh);
            seen.insert(mul_neigh);
            // update neighbors parents
            parents.insert(add_neigh, Some((cur, Op::Add)));
            parents.insert(mul_neigh, Some((cur, Op::Mul)));
        }
    }
    // q is empty and goal not found
    println!("{:>20}: \t ❌", goal.res);
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::part1::*;

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
    fn test_bfs() {
        let (_, lines) = parse(INPUT_TEST).unwrap();
        assert!(check_line(lines[0].0, &lines[0].1));
        assert!(check_line(lines[1].0, &lines[1].1));
        assert!(!check_line(lines[2].0, &lines[2].1));
        assert!(!check_line(lines[3].0, &lines[3].1));
        assert!(!check_line(lines[4].0, &lines[4].1));
        assert!(!check_line(lines[5].0, &lines[5].1));
        assert!(!check_line(lines[6].0, &lines[6].1));
        assert!(!check_line(lines[7].0, &lines[7].1));
        assert!(check_line(lines[8].0, &lines[8].1));
        assert_eq!(chech_lines(lines), 3749);
    }

    #[test]
    fn test_line_setting() {
        let input = "21037: 21037
292: 0";
        let (_, lines) = parse(input).unwrap();
        assert!(check_line(lines[0].0, &lines[0].1));
        assert!(!check_line(lines[1].0, &lines[1].1));
    }
}
