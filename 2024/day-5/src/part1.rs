#![allow(unused)]
use std::collections::{BTreeSet, HashMap};

use miette::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char, digit1, line_ending},
    combinator::map_res,
    multi::{many1, many_till, separated_list1},
    sequence::separated_pair,
    Err, IResult, Parser,
};

use crate::topo::{add_edge, Graph};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    // Parse rule pairs in BtreeMap
    // Parse list of pages
    // Consolidate rules into BTreeMap
    // Create Ord based on BTreeMap / simple checks?
    // find the middle number of the pages
    // TODO: Add results
    let (rules, mut pages_list) = input_to_rules_and_pages(input)?;
    pages_list.retain(|pages| check_pages_order(&rules, pages));
    let result: i32 = pages_list.iter().filter_map(mid_num).sum();

    Ok(result.to_string())
}

pub type Rules = BTreeSet<(i32, i32)>;
pub type Rule = (i32, i32);
pub type PagesList = Vec<Pages>;
pub type Pages = Vec<i32>;

pub fn input_to_rules_and_pages(input: &str) -> Result<(Graph<i32>, PagesList)> {
    let (_rest, (rules, pages)) = parse_rules_and_pages(input).unwrap();
    Ok((rules, pages))
}

pub fn parse_rules_and_pages(input: &str) -> IResult<&str, (Graph<i32>, PagesList)> {
    let (rest, rules_vec) =
        separated_list1(line_ending, separated_pair(num, char('|'), num)).parse(input)?;
    let mut rules = HashMap::new();
    rules_vec.iter().for_each(|(a, b)| {
        add_edge(&mut rules, a.to_owned(), b.to_owned());
    });
    let (rest, (_, pages)) = many_till(
        anychar,
        separated_list1(line_ending, separated_list1(char(','), num)),
    )
    .parse(rest)?;

    Ok((rest, (rules, pages)))
}

pub fn check_pages_order(rules: &Graph<i32>, pages: &Pages) -> bool {
    pages.is_sorted_by(|a, b| rules.get(a).is_some_and(|v| v.contains(b)))
}

pub fn check_rule(rule: &Rule, pages: &Pages) -> bool {
    let first = pages.iter().position(|&x| x == rule.0);
    let second = pages.iter().position(|&x| x == rule.1);
    match (first, second) {
        (Some(a), Some(b)) => a < b,
        _ => true,
    }
}

pub fn num(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse::<i32>).parse(input)
}

pub fn mid_num(pages: &Pages) -> Option<i32> {
    if pages.len() % 2 != 1 {
        return None;
    }
    Some(pages[pages.len() / 2])
}

#[cfg(test)]
mod tests {

    use super::*;
    pub const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_process() -> Result<()> {
        assert_eq!(process(INPUT).unwrap(), "143");
        Ok(())
    }

    #[test]
    fn test_check_order() -> Result<()> {
        let (_rest, (rules, pages)) = parse_rules_and_pages(INPUT).unwrap();

        assert!(check_pages_order(&rules, &pages[0]));
        assert!(!check_pages_order(&rules, &pages[3]));
        Ok(())
    }

    #[test]
    fn test_mid_num() -> Result<()> {
        assert_eq!(mid_num(&vec![75, 47, 61, 53, 29]), Some(61));
        assert_eq!(mid_num(&vec![75, 53, 29]), Some(53));
        assert_eq!(mid_num(&vec![75, 47, 53, 29]), None);
        assert_eq!(mid_num(&vec![]), None);
        Ok(())
    }
}
