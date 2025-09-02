use std::fmt::Display;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let parsed = parse(input);
    let mut expanded = expand(parsed);
    expanded.retain(|b| !b.is_empty());
    let flat = compact(&mut expanded);
    let check_sum = check_sum(&flat);
    Ok(check_sum.to_string())
}

#[derive(Clone, Copy)]
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
pub fn expand(parsed: Vec<Block>) -> Vec<Vec<Option<usize>>> {
    parsed.iter().map(flatten_block).collect()
}

// TODO: make sure to continue after cursor cross.
pub fn compact(expanded: &mut [Vec<Option<usize>>]) -> Vec<Option<usize>> {
    // Make a flatten copy of expanded into a Vec<Option<usize>>, wasteful, but fuck it.
    let mut flat: Vec<Option<usize>> = line_up(expanded);
    // set right pointer to the lentgh of flat
    let mut right = flat.len();

    // loop over the NON-flatten vector of vectors to get the length of each elemente easily
    // might be sub-obtimal too...
    for e in expanded.iter_mut().rev() {
        // length of the memory block
        let len = e.len();
        // if it's not an empty block, process it
        if e.iter_mut().all(|element| element.is_some()) {
            // search from the left to the right if there is a big enough space
            // probably wasteful to recheck from the start, but it avoids false negatives.
            if let Some(target_start) = get_none_run(&flat[..right - len], len) {
                // get mutable subslices of the free space and memory block in the vector
                if let Ok([a, b]) =
                    flat.get_disjoint_mut([target_start..target_start + len, right - len..right])
                {
                    // swap the subslices
                    a.swap_with_slice(b);
                }
            }
        }
        // As long as the right pointer can be moved left do so, else it's finished.
        if let Some(new_right) = right.checked_sub(len) {
            right = new_right;
        } else {
            break;
        }
    }
    // returned the reordered flatten vector.
    flat
}

// Search for as many space as we need from the left
fn get_none_run(slice: &[Option<usize>], need: usize) -> Option<usize> {
    // Not needing anything is the source of happiness
    if need == 0 {
        return Some(0);
    }
    // initialize a running count of available spaces
    let mut run: usize = 0;
    // iter over the position i and value v of each element
    for (i, v) in slice.iter().enumerate() {
        // if it's an empty block, add 1 to the running count
        if v.is_none() {
            run += 1;
            // return the starting position of a valid need sized empty run
            if run == need {
                return Some(i + 1 - need);
            }
        } else {
            // if there is a some, restart the running count.
            run = 0;
        }
    }
    // we tried everything and couldn't find a fitting block
    None
}

// Make a copy of compacted vec of vec into as a single vec
pub fn line_up(compacted: &[Vec<Option<usize>>]) -> Vec<Option<usize>> {
    compacted.iter().flat_map(|v| v.iter().copied()).collect()
}

// Accumulate the products of the IDs and their positions.
pub fn check_sum(compacted: &[Option<usize>]) -> usize {
    // inner function called by the recursive accumulation
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

    // 0..111....22222
    // 02.111....2222.
    // 022111....222..
    // 0221112...22...
    // 02211122..2....
    // 022111222......
    const TEST1: &str = "2333133121414131402";
    const TEST1_EXPANDED: &str = "00...111...2...333.44.5555.6666.777.888899";
    const TEST1_COMPACTED: &str = "0099811188827773336446555566..............";
    const TEST1_CHECK_SUM: usize = 2858;
    #[test]
    fn test_count() -> miette::Result<()> {
        let input1 = TEST1;
        let mut expanded1 = expand(parse(input1));

        expanded1.retain(|b| !b.is_empty());
        let flat = compact(&mut expanded1);
        assert_eq!(TEST1_CHECK_SUM, check_sum(&flat));
        Ok(())
    }
}
