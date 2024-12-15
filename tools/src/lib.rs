use anyhow::{anyhow, bail, Error};
use ndarray::Array2;

#[derive(Default)]
struct GridShape {
    num_cols: Option<usize>,
    num_rows: usize,
}

impl GridShape {
    pub fn add_row(&mut self, col_count: usize) -> Result<(), Error> {
        if let Some(width) = &mut self.num_cols {
            if col_count != *width {
                bail!("inconsistent column count");
            }
        } else {
            self.num_cols = Some(col_count);
        }

        self.num_rows += 1;

        Ok(())
    }

    pub fn calc(self) -> Result<(usize, usize), Error> {
        let num_cols = self.num_cols.ok_or_else(|| anyhow!("no rows seen"))?;
        Ok((self.num_rows, num_cols))
    }
}

pub fn load_grid(rd: impl std::io::BufRead) -> Result<(Array2<char>, Option<String>), Error> {
    let mut data = Vec::new();
    let mut grid_shape = GridShape::default();
    let mut extra: Option<String> = None;

    for ln in rd.lines() {
        let ln = ln?;
        if let Some(extra) = &mut extra {
            extra.push_str(&ln);
            continue;
        }

        if ln.is_empty() {
            extra = Some(String::new());
            continue;
        }

        let mut col_count = 0;
        for ch in ln.chars() {
            data.push(ch);
            col_count += 1;
        }

        grid_shape.add_row(col_count)?;
    }

    let shape = grid_shape.calc()?;
    let grid = Array2::from_shape_vec(shape, data).map_err(|_| anyhow!("bad array shape"))?;
    Ok((grid, extra))
}

pub const DELTAS: &[(i64, i64)] = &[(0, 1), (0, -1), (-1, 0), (1, 0)];

pub fn shift(
    heights: &Array2<char>,
    pos: (usize, usize),
    dr: i64,
    dc: i64,
) -> Option<(usize, usize)> {
    let row = i64::try_from(pos.0).ok()?;
    let col = i64::try_from(pos.1).ok()?;
    let row = row.checked_add(dr)?;
    let col = col.checked_add(dc)?;
    let row = usize::try_from(row).ok()?;
    let col = usize::try_from(col).ok()?;

    if row < heights.dim().0 && col < heights.dim().1 {
        Some((row, col))
    } else {
        None
    }
}
