use anyhow::{anyhow, bail, Error};
use ndarray::Array2;
use std::fmt::{Display, Write as _};

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

pub fn print_grid(grid: &Array2<impl Display>) {
    let mut row_buf = String::new();

    for (pos, val) in grid.indexed_iter() {
        if pos.1 < row_buf.len() {
            println!("{row_buf}");
            row_buf.clear();
        }

        write!(&mut row_buf, "{val}").unwrap();
    }

    if !row_buf.is_empty() {
        println!("{row_buf}");
    }
}
