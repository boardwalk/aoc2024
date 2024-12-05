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

        if *ch == '.' {
            None
        } else {
            Some(*ch)
        }
    }

    fn take(&mut self, row: i64, col: i64) -> Option<char> {
        let row = usize::try_from(row).ok()?;
        let col = usize::try_from(col).ok()?;

        let ch = self.rows.get_mut(row)?.get_mut(col)?;
        if *ch == '.' {
            None
        } else {
            Some(std::mem::replace(ch, '.'))
        }
    }
}

fn try_word(grid: &mut Grid, mut start_r: i64, mut start_c: i64, dr: i64, dc: i64) -> bool {
    const WORD: &[char] = &['X', 'M', 'A', 'S'];
    let mut r = start_r;
    let mut c = start_c;

    for word_char in WORD {
        let Some(ch) = grid.get(r, c) else {
            return false;
        };

        if ch != *word_char {
            return false;
        }
        r += dr;
        c += dc;
    }

    let mut r = start_r;
    let mut c = start_c;

    for _word_char in WORD {
        // grid.take(r, c).unwrap();
        r += dr;
        c += dc;
    }

    true
}

fn main() -> Result<(), Error> {
    let mut grid = Grid::read(std::io::stdin().lock())?;

    let mut num_found = 0;

    for r in 0..grid.num_rows() {
        for c in 0..grid.num_cols() {
            for dr in [-1, 0, 1] {
                for dc in [-1, 0, 1] {
                    if dr == 0 && dc == 0 {
                        continue;
                    }

                    if try_word(&mut grid, r, c, dr, dc) {
                        num_found += 1;
                    }
                }
            }
        }
    }

    println!("num_found = {num_found}");

    Ok(())
}
