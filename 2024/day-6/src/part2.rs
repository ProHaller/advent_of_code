#![allow(unused)]
use crate::part1::*;

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
        .filter(|(_vpos, state)| state.ground_is_true())
        .count();

    Ok(sum.to_string())
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
