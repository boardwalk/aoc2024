use anyhow::{anyhow, bail, Error};
use std::io::BufRead;

struct Grid {
    rows: Vec<Vec<char>>,
    num_cols: i64,
}

impl Grid {
    fn read(r: impl BufRead) -> Result<Self, Error> {
        let mut num_cols = None;
        let mut rows = Vec::new();
        for ln in r.lines() {
            let ln = ln?;

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

        let num_cols = num_cols.ok_or_else(|| anyhow!("no rows"))?;
        let num_cols = i64::try_from(num_cols)?;

        Ok(Grid { rows, num_cols })
    }

    fn num_rows(&self) -> i64 {
        i64::try_from(self.rows.len()).unwrap()
    }

    fn num_cols(&self) -> i64 {
        self.num_cols
    }

    fn get(&self, row: i64, col: i64) -> Option<char> {
        let row = usize::try_from(row).ok()?;
        let col = usize::try_from(col).ok()?;

        let ch = self.rows.get(row)?.get(col)?;

        Some(*ch)
    }
}

fn eval_x_2(grid: &Grid, r: i64, c: i64) -> bool {
    // center must be an A
    if grid.get(r, c).unwrap() != 'A' {
        return false;
    }

    let ul = grid.get(r - 1, c - 1).unwrap();
    let ur = grid.get(r - 1, c + 1).unwrap();
    let dr = grid.get(r + 1, c + 1).unwrap();
    let dl = grid.get(r + 1, c - 1).unwrap();

    // corners must be S or M
    if ul != 'S' && ul != 'M' {
        return false;
    }

    if ur != 'S' && ur != 'M' {
        return false;
    }

    if dr != 'S' && dr != 'M' {
        return false;
    }

    if dl != 'S' && dl != 'M' {
        return false;
    }

    // opposite corners must not match (e.g. MAM or SAS)
    if ul == dr {
        return false;
    }

    if ur == dl {
        return false;
    }

    true
}

fn main() -> Result<(), Error> {
    let grid = Grid::read(std::io::stdin().lock())?;

    let mut num_found = 0;

    for r in 1..grid.num_rows() - 1 {
        for c in 1..grid.num_cols() - 1 {
            if eval_x_2(&grid, r, c) {
                num_found += 1;
            }
        }
    }

    println!("num_found = {num_found}");

    Ok(())
}
