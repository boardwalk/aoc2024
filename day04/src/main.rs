use anyhow::{bail, Error};
use std::io::BufRead;

struct Grid {
    rows: Vec<Vec<char>>,
    num_cols: usize,
}

impl Grid {
    fn read(r: impl BufRead) -> Result<Self, Error> {
        let mut num_cols = None;
        let mut rows = Vec::new();
        for ln in r.lines() {
            let ln = ln?;
            if ln.contains('.') {
                bail!("reserved char used");
            }

            let row: Vec<char> = ln.chars().collect();

            if let Some(num_cols) = num_cols {
                if num_cols != row.len() {
                    bail!("uneven columns");
                }
            } else {
                num_cols = Some(row.len());
            }

            rows.push(row);
        }

        Ok(Grid {
            rows,
            num_cols: num_cols.unwrap(),
        })
    }

    fn num_rows(&self) -> usize {
        self.rows.len()
    }

    fn num_cols(&self) -> usize {
        self.num_cols
    }

    fn get(&self, row: usize, col: usize) -> Option<char> {
        let ch = self.rows.get(row)?.get(col)?;

        Some(*ch)
    }
}

fn main() -> Result<(), Error> {
    let grid = Grid::read(std::io::stdin().lock())?;
    Ok(())
}
