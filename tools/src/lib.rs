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

pub fn load_grid(rd: impl std::io::BufRead) -> Result<Array2<char>, Error> {
    let mut data = Vec::new();
    let mut grid_shape = GridShape::default();

    for ln in rd.lines() {
        let ln = ln?;
        grid_shape.add_row(ln.len())?;
        data.extend(ln.chars());
    }

    let shape = grid_shape.calc()?;
    Array2::from_shape_vec(shape, data).map_err(|_| anyhow!("bad array shape"))
}
