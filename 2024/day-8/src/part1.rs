use std::collections::{HashMap, HashSet};

use miette::miette;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut freq_maps = parse(input);
    // TODO: for each key iterate over every pair of values.
    build_antinode_map(&mut freq_maps, 50);

    if let Some(antinodes_nb) = freq_maps.get(&'#').map(|v| v.len()) {
        Ok(antinodes_nb.to_string())
    } else {
        Err(miette!("Oops"))
    }
}

pub fn map_print(map: HashMap<char, Vec<Pos>>, size: i32) {
    for (k, vp) in &map {
        eprintln!();
        for l in 0..size {
            for n in 0..size {
                if vp.iter().any(|p| p == &Pos { y: l, x: n }) {
                    eprint!("{}", k);
                } else {
                    eprint!(".");
                }
            }
            eprintln!()
        }
    }
}

// 1. parse input into hashmaps of frequency
pub fn parse(input: &str) -> HashMap<char, Vec<Pos>> {
    let lines = input.lines();
    let mut freq_maps: HashMap<char, Vec<Pos>> = HashMap::new();
    for (ln, line) in lines.enumerate() {
        for (cn, char) in line.chars().enumerate() {
            match char {
                '.' => (),
                c => {
                    freq_maps
                        .entry(c)
                        .and_modify(|freq_map| {
                            freq_map.push(Pos {
                                y: ln as i32,
                                x: cn as i32,
                            })
                        })
                        .or_insert(vec![Pos {
                            y: ln as i32,
                            x: cn as i32,
                        }]);
                }
            }
        }
    }
    freq_maps
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
pub struct Pos {
    y: i32,
    x: i32,
}
// 2. Vector calculation a` = a->b *2  and b` = ((a->b)*2)  and b` = -(( a->b ) *2)
impl Pos {
    /// Returns true if the position is inside the square map.
    fn is_inside(&self, size: i32) -> bool {
        (0..size).contains(&self.x) && (0..size).contains(&self.y)
    }

    /// Vector difference between two positions.
    fn delta(self, other: Pos) -> Pos {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// Shift this position by a vector.
    fn shifted(self, v: Pos) -> Pos {
        Pos {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

pub fn set_antinode(a: Pos, b: Pos, map_size: i32) -> Vec<Pos> {
    eprintln!("a: y:{} x:{}, b:y:{} x:{}", a.y, a.x, b.y, b.x);
    // Vector from b to a
    let v_ab = a.delta(b);
    // Antinodes are two steps away in both directions
    let antinode_a = a.shifted(Pos {
        x: v_ab.x,
        y: v_ab.y,
    });
    let antinode_b = b.shifted(Pos {
        x: -v_ab.x,
        y: -v_ab.y,
    });
    let mut antinodes = Vec::new();
    if antinode_a.is_inside(map_size) {
        antinodes.push(antinode_a);
    }
    if antinode_b.is_inside(map_size) {
        antinodes.push(antinode_b);
    }
    antinodes
}

// 3. merge HashMaps and count
pub fn build_antinode_map(freq_maps: &mut HashMap<char, Vec<Pos>>, size: i32) {
    let mut antinodes: HashSet<Pos> = HashSet::new();
    for (freq, antennas) in &mut *freq_maps {
        eprintln!("freq:'{freq}'");
        let mut seen: HashSet<(Pos, Pos)> = HashSet::new();
        for i in 0..antennas.len() {
            for o in i + 1..antennas.len() {
                if seen.insert((antennas[i], antennas[o])) | seen.insert((antennas[o], antennas[i]))
                {
                    let res = set_antinode(antennas[i], antennas[o], size);
                    antinodes.extend(&res);
                    eprintln!("i={}, o={} => {:?}", i, o, &res);
                }
            }
        }
    }
    freq_maps.entry('#').or_default().extend(antinodes.iter());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let solution = "......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.";
        let mut map = parse(input);
        let mut sol_map = parse(solution);

        assert_eq!(
            HashMap::from([
                (
                    '0',
                    vec![
                        Pos { y: 1, x: 8 },
                        Pos { y: 2, x: 5 },
                        Pos { y: 3, x: 7 },
                        Pos { y: 4, x: 4 },
                    ]
                ),
                (
                    'A',
                    vec![Pos { y: 5, x: 6 }, Pos { y: 8, x: 8 }, Pos { y: 9, x: 9 }]
                )
            ]),
            map
        );
        if let Some(v_pos) = map.get(&'0') {
            assert_eq!(
                set_antinode(v_pos[0], v_pos[1], 12),
                vec![Pos { y: 0, x: 11 }, Pos { y: 3, x: 2 }],
            );
        }
        build_antinode_map(&mut map, 12);
        assert_eq!(
            map.get(&'#').map(|v| v.len()),
            sol_map.get(&'#').map(|v| v.len() + 1)
        );

        Ok(())
    }
}
