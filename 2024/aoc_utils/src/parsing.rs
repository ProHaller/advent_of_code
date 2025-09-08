use std::fmt::Display;
use std::hash::Hash;

use std::collections::{HashMap, HashSet};

use crate::display::Grid;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
pub struct Pos<V> {
    pub line: V,
    pub column: V,
}

impl<A, V> From<(A, A)> for Pos<V>
where
    A: Into<V>,
{
    fn from(value: (A, A)) -> Self {
        Self {
            line: value.0.into(),
            column: value.1.into(),
        }
    }
}

pub fn print_map_pos_columns(map: &HashMap<u8, HashSet<Pos<usize>>>) {
    let mut keys: Vec<u8> = map.keys().copied().collect();
    keys.sort_unstable();

    // Pre-process the data into owned Vecs
    let data: HashMap<u8, Vec<Pos<usize>>> = keys
        .iter()
        .filter_map(|&k| {
            map.get(&k).map(|set| {
                let mut vec: Vec<Pos<usize>> = set.iter().copied().collect();
                vec.sort(); // Optional: sort for consistent output
                (k, vec)
            })
        })
        .collect();

    let grid = Grid::new(
        keys,
        |k| data.get(k).map(|v| v.as_slice()),
        |k| format!("{k:>2}:   l c"),
        |i, p: &Pos<usize>| format!("{i:>2}  {}:{}", p.line, p.column),
    );

    println!("{grid}");
}

impl<V: Display> Display for Pos<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

pub fn into_pos_map_with<K, V, F>(input: &str, mut key_of: F) -> HashMap<K, HashSet<Pos<V>>>
where
    K: Eq + Hash,
    V: From<usize> + Copy + Eq + Hash,
    F: FnMut(char) -> Option<K>,
{
    let mut map: HashMap<K, HashSet<Pos<V>>> = HashMap::new();

    for (line_idx, line) in input.lines().enumerate() {
        for (col_idx, ch) in line.chars().enumerate() {
            if let Some(key) = key_of(ch) {
                map.entry(key).or_default().insert(Pos {
                    line: V::from(line_idx),
                    column: V::from(col_idx),
                });
            }
        }
    }

    map
}
