#![allow(dead_code, unused_variables)]
pub use nom::{IResult, bytes::complete::tag};
use nom::{
    Parser,
    branch::alt,
    bytes::complete::{take_until, take_while_m_n},
    character::complete::{anychar, char},
    combinator::{map_res, value},
    multi::{many_till, many0, many1},
    sequence::{preceded, separated_pair},
};
use std::error::Error;
static INPUT_D3: &str =
    include_str!("/Volumes/Dock/Dev/Rust/projects/advent_of_code/2024/assets/day_3/input.txt");

pub fn day_3() {
    let res_part_1 = part_1(INPUT_D3);
    println!("{res_part_1:?}");

    let res_part_2 = part_2(INPUT_D3);
    println!("{}", res_part_2.unwrap());
}

fn part_1<'a>(input: &'a str) -> Result<i64, Box<dyn Error + 'a>> {
    let (rest, output) = parse_all_mul(input)?;
    let result = output
        .iter()
        .fold((Instruction::Do, 0), |acc, ins| match ins {
            Instruction::Mul(a, b) if acc.0 != Instruction::Dont => {
                (Instruction::Do, acc.1 + (a * b))
            }
            Instruction::Mul(a, b) => (Instruction::Do, acc.1 + (a * b)),
            Instruction::Do => (Instruction::Do, 0),
            Instruction::Dont => (Instruction::Dont, 0),
        });
    println!("rest: {rest}");
    println!("result: {}", result.1);

    println!("Day 3 Part 1\n Result: {}", result.1);
    Ok(result.1 as i64)
}

fn parse_all_mul(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(find_mul)
        .map(|v| v.into_iter().flatten().collect())
        .parse(input)
}

fn find_mul(input: &str) -> IResult<&str, Option<Instruction>> {
    preceded(
        take_until("mul"),
        alt((parse_one_mul.map(Some), value(None, tag("mul")))),
    )
    .parse(input)
}

fn parse_one_mul(input: &str) -> IResult<&str, Instruction> {
    let (rest, (_mul, _p1, (d1, d2), _p2)) = (
        tag("mul"),
        char('('),
        separated_pair(number, char(','), number),
        char(')'),
    )
        .parse(input)?;
    Ok((rest, Instruction::Mul(d1, d2)))
}

fn number(input: &str) -> IResult<&str, i32> {
    map_res(
        take_while_m_n(1, 3, |c: char| c.is_ascii_digit()),
        |s: &str| s.parse::<i32>(),
    )
    .parse(input)
}

fn part_2<'a>(input: &'a str) -> Result<i64, Box<dyn Error + 'a>> {
    let (rest, output) = parse_instructions(input)?;
    let result = fold_isntructions(output);
    println!("rest: {rest}");
    println!("result: {result}");

    println!("Day 3 Part 1\n Result: {result}");
    Ok(result)
}
fn fold_isntructions(output: Vec<Instruction>) -> i64 {
    dbg!(&output);
    let (ins, result) = output.iter().fold((Instruction::Do, 0), |acc, ins| {
        dbg!(&acc, ins);
        match ins {
            Instruction::Dont => (Instruction::Dont, acc.1),
            Instruction::Do => (Instruction::Do, acc.1),
            Instruction::Mul(a, b) if acc.0 != Instruction::Dont => {
                (Instruction::Do, acc.1 + (a * b))
            }
            Instruction::Mul(a, b) => acc,
        }
    });
    result as i64
}

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (rest, ins) =
        many1(many_till(anychar, parse_one_instruction).map(|(_c, ins)| ins)).parse(input)?;
    Ok((rest, ins))
}

// fn skip_until_dont(input: &str) -> IResult<&str, ()> {
//     let mut rest_txt: &str = "";
//     while let (rest, (Some(mul), Some(false) | None)) = (find_mul, find_do).parse(input)? {
//         rest_txt = rest;
//     }
//     Ok((rest_txt, ()))
// }

fn parse_one_instruction(input: &str) -> IResult<&str, Instruction> {
    let (rest, ins) = alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        parse_one_mul, // already returns Instruction
    ))
    .parse(input)?;
    Ok((rest, ins))
}

fn find_do(input: &str) -> IResult<&str, Option<Instruction>> {
    dbg!();
    preceded(
        take_until("do"),
        alt((parse_one_instruction.map(Some), value(None, tag("do")))),
    )
    .parse(input)
}

#[cfg(test)] // This ensures the 'tests' module is only compiled in test builds
mod tests {
    use super::*;
    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT1).unwrap(), 161);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT2).unwrap(), 48);
    }
    #[test]
    fn test_parse_one_ins() {
        let do1 = "do()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let dont = "don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(
            parse_one_instruction(do1).unwrap(),
            (
                "_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
                Instruction::Do,
            )
        );
        assert_eq!(
            parse_one_instruction(dont).unwrap(),
            (
                "_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
                Instruction::Dont
            )
        );
    }
    #[test]
    fn test_find_mul() {
        let input = "?mul(8,5))";
        assert_eq!(
            find_mul(input).unwrap(),
            (")", Some(Instruction::Mul(8, 5)))
        );
    }
    #[test]
    fn test_parse_mul_or_do() {
        use Instruction::{Do, Dont, Mul};

        let do1 = "do()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let dont = "don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(
            parse_instructions(do1),
            Ok((")", vec![Do, Mul(5, 5), Mul(11, 8), Do, Mul(8, 5)]))
        );
        assert_eq!(
            parse_instructions(dont),
            Ok((")", vec![Dont, Mul(5, 5), Mul(11, 8), Do, Mul(8, 5)]))
        );
    }
    #[test]
    fn test_find_do() {
        assert_eq!(
            find_do(INPUT2).unwrap(),
            (
                "_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
                Some(Instruction::Dont)
            )
        );
    }
}
