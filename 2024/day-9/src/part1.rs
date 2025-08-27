use std::fmt::Display;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let parsed = parse(input);
    let mut expanded = expand(parsed);
    compact(&mut expanded);
    let compacted = expanded;
    let check_sum = check_sum(&compacted);
    Ok(check_sum.to_string())
}

pub enum Block {
    B(usize, usize),
    Z(usize),
}

impl TryFrom<(usize, char)> for Block {
    type Error = &'static str;

    fn try_from((i, ch): (usize, char)) -> Result<Self, Self::Error> {
        let d: usize = ch.to_digit(10).ok_or("not a decimal digit")? as usize;

        Ok(if i % 2 == 0 {
            Block::B(i / 2, d)
        } else {
            Block::Z(d)
        })
    }
}
impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::B(i, n) => write!(f, "{}", i.to_string().repeat(*n)),
            Block::Z(n) => write!(f, "{}", ".".repeat(*n)),
        }
    }
}

pub fn parse(input: &str) -> Vec<Block> {
    input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            Block::try_from((i, c)).unwrap_or_else(|_| panic!("Not a decimal digit: {c}"))
        })
        .collect()
}

pub fn flatten_block(block: &Block) -> Vec<Option<usize>> {
    match block {
        Block::B(i, n) => {
            vec![Some(*i); *n]
        }
        Block::Z(n) => {
            vec![None; *n]
        }
    }
}
pub fn expand(parsed: Vec<Block>) -> Vec<Option<usize>> {
    parsed.iter().flat_map(flatten_block).collect()
}

#[allow(unused)]
fn wrong_compact(parsed: &mut [Option<usize>]) {
    let mut right: usize = parsed.len() - 1;
    for left in 0..parsed.len() - 1 {
        while parsed[right].is_none() {
            right -= 1;
        }
        if parsed[left].is_none() {
            parsed.swap(left, right);
            right -= 1;
        };
        if left == right {
            break;
        }
    }
}

pub fn compact(expanded: &mut [Option<usize>]) {
    let mut left = 0;
    let mut right = expanded.len().saturating_sub(1);

    while left < right {
        // Move left pointer to find next None (free space)
        while left < expanded.len() && expanded[left].is_some() {
            left += 1;
        }

        // Move right pointer to find next Some (file block)
        while right > 0 && expanded[right].is_none() {
            right -= 1;
        }

        // If both pointers are valid and left < right, swap
        if left < right {
            expanded.swap(left, right);
        }
    }
}

pub fn check_sum(compacted: &[Option<usize>]) -> usize {
    fn sum_n(acc: usize, (position, chiffre): (usize, &Option<usize>)) -> usize {
        match chiffre {
            Some(lechiffre) => acc + position * *lechiffre,
            None => acc,
        }
    }
    compacted
        .iter()
        .enumerate() // iteration
        .fold(0, sum_n)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "2333133121414131402";
    const TEST1_EXPANDED: &str = "00...111...2...333.44.5555.6666.777.888899";
    const TEST1_COMPACTED: &str = "0099811188827773336446555566..............";
    const TEST1_CHECK_SUM: usize = 1928;
    #[test]
    fn test_parse() -> miette::Result<()> {
        let expanaded = expand(parse(TEST1));
        let expanded_string = expanaded
            .iter()
            .map(|mn| match mn {
                Some(n) => n.to_string(),
                None => ".".to_string(),
            })
            .collect::<String>();
        assert_eq!(TEST1_EXPANDED, expanded_string);
        Ok(())
    }
    #[test]
    fn test_compacted() -> miette::Result<()> {
        let mut expanded1 = expand(parse(TEST1));
        compact(&mut expanded1);
        let compacted_string = expanded1
            .iter()
            .map(|mn| match mn {
                Some(n) => n.to_string(),
                None => ".".to_string(),
            })
            .collect::<String>();

        assert_eq!(TEST1_COMPACTED, dbg!(compacted_string));
        Ok(())
    }
    #[test]
    fn test_count() -> miette::Result<()> {
        let input1 = TEST1;
        let mut expanded1 = expand(parse(input1));
        compact(&mut expanded1);
        assert_eq!(TEST1_CHECK_SUM, check_sum(&expanded1));
        Ok(())
    }
}
