mod grid;
mod heap;

pub use grid::load_grid;
pub use grid::print_grid;
pub use heap::{heap_decrease, heap_pop, heap_push};

use ndarray::Array2;

pub const DELTAS: &[(i64, i64)] = &[(0, 1), (0, -1), (-1, 0), (1, 0)];

pub fn shift<T>(
    heights: &Array2<T>,
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
