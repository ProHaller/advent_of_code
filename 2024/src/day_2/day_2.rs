fn parse_report(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(parse_report)
        .filter(|report| safecheck_with_tolerance(report))
        .count()
}

fn is_monotone(line: &[u64]) -> bool {
    // check if on or less numbers
    if line.len() <= 1 {
        return true;
    };
    let ordering = line[0].cmp(&line[1]);
    for window in line.windows(2) {
        let current = window[0];
        let next = window[1];
        if !(1..=3).contains(&current.abs_diff(next)) {
            return false;
        }
        if current.cmp(&next) != ordering {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn bruteforced_safecheck(line: &[u64]) -> bool {
    for n in 0..line.len() {
        let mut test_line = Vec::from(line);
        test_line.remove(n);
        if is_monotone(&test_line) {
            println!("{:<40} {t:>5}", format!("{:?}", line), t = "true");
            return true;
        }
    }
    println!("{:<40} {f:>5}", format!("{:?}", line), f = "false");
    false
}

// FIX: This does not work with [1, 2, 5, 3, 4, 7, 8]
//                                     ^  ^  ^
// 3 is the first out of order but 5 is the one to eliminate
fn safecheck_with_tolerance(line: &[u64]) -> bool {
    // check if the line is monotone after the first element
    if is_monotone(&line[1..]) {
        println!("{:<40} {:>5}", format!("{:?}", line), "true");
        return true;
    }
    let mut misses = 0;
    let mut previous = line[0];

    // if the first element is wrong and the rest is ok, the is_monotone guard caught it.
    // if the second element is wrong  then the order should be correct 1-3
    // check baseline order between the first and third element
    let ordering = line[0].cmp(&line[2]);

    // loop the elements after the first since previous is set to the first
    for &current in line.iter().skip(1) {
        // check if the absolute difference is between 1 and 3
        if current == previous || current.abs_diff(previous) > 3 {
            misses += 1;
            continue;
        }

        // check if order is the same as between 1 and 3
        if previous.cmp(&current) != ordering {
            misses += 1;
            continue;
        }
        // update pointers
        previous = current;
    }
    // if more than 1 miss, false else true
    println!("{:<40} {:>5}", format!("{:?}", line), misses < 2);
    misses <= 1
}
