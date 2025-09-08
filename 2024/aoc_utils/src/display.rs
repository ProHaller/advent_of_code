use std::fmt::{self, Display, Formatter};

/// Generic grid printer over arbitrary "key -> slice of values" data.
pub struct Grid<'a, K, P, G, H, C> {
    keys: Vec<K>,
    get: G,
    header: H,
    cell: C,
    _phantom: std::marker::PhantomData<&'a [P]>,
}

impl<'a, K, P, G, H, C> Grid<'a, K, P, G, H, C>
where
    // The getter must accept &K with ANY lifetime `'k`
    G: for<'k> Fn(&'k K) -> Option<&'a [P]>,
    // Same for the header closure
    H: for<'k> Fn(&'k K) -> String,
    // Cell doesn’t take references to K, so no HRTB needed here
    C: Fn(usize, &P) -> String,
{
    pub fn new(keys: Vec<K>, get: G, header: H, cell: C) -> Self {
        Self {
            keys,
            get,
            header,
            cell,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, K, P, G, H, C> Display for Grid<'a, K, P, G, H, C>
where
    K: Clone,
    G: for<'k> Fn(&'k K) -> Option<&'a [P]>,
    H: for<'k> Fn(&'k K) -> String,
    C: Fn(usize, &P) -> String,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // 1) Max column height
        let max_len = self
            .keys
            .iter()
            .map(|k| (self.get)(k).map_or(0, |col| col.len()))
            .max()
            .unwrap_or(0);

        // 2) Compute per-column widths from headers and cells
        let mut col_widths = Vec::with_capacity(self.keys.len());
        for k in &self.keys {
            let mut w = (self.header)(k).len();
            if let Some(col) = (self.get)(k) {
                for (i, p) in col.iter().enumerate() {
                    w = w.max((self.cell)(i, p).len());
                }
            }
            col_widths.push(w);
        }

        let pad = |s: &str, w: usize| format!("{:>width$}", s, width = w);

        // 3) Header row
        for (j, k) in self.keys.iter().enumerate() {
            let h = (self.header)(k);
            write!(f, "{}  ", pad(&h, col_widths[j]))?;
        }
        writeln!(f)?;

        // 4) Rows
        for i in 0..max_len {
            for (j, k) in self.keys.iter().enumerate() {
                let cell = (self.get)(k)
                    .and_then(|col| col.get(i))
                    .map(|p| (self.cell)(i, p))
                    .unwrap_or_default();
                write!(f, "{} │", pad(&cell, col_widths[j]))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
