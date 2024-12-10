use anyhow::{anyhow, Error};
use ndarray::Array2;

fn get_height(heights: &Array2<char>, pos: (usize, usize)) -> Result<usize, Error> {
    let height = heights[pos];
    let height = height
        .to_digit(10)
        .ok_or_else(|| anyhow!("invalid height on map"))?;
    let height = usize::try_from(height)?;
    Ok(height)
}

fn shift(heights: &Array2<char>, pos: (usize, usize), dr: i64, dc: i64) -> Option<(usize, usize)> {
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

const DELTAS: &[(i64, i64)] = &[(0, 1), (0, -1), (-1, 0), (1, 0)];

fn eval_trailhead(heights: &Array2<char>, start: (usize, usize)) -> Result<usize, Error> {
    let mut seen: Array2<bool> = Array2::default(heights.raw_dim());
    let mut work = Vec::new();
    work.push(start);

    while let Some(pos) = work.pop() {
        seen[pos] = true;
        let height = get_height(heights, pos)?;
        for (dr, dc) in DELTAS {
            let Some(new_pos) = shift(heights, pos, *dr, *dc) else {
                continue;
            };

            let new_height = get_height(heights, new_pos)?;

            if !seen[new_pos] && new_height == height + 1 {
                work.push(new_pos);
            }
        }
    }

    let mut score = 0;
    for (pos, val) in heights.indexed_iter() {
        if *val == '9' && seen[pos] {
            score += 1;
        }
    }

    Ok(score)
}

fn main() -> Result<(), Error> {
    let heights = tools::load_grid(std::io::stdin().lock())?;
    let mut total_score = 0;
    for (pos, val) in heights.indexed_iter() {
        if *val == '0' {
            total_score += eval_trailhead(&heights, pos)?;
        }
    }

    println!("{total_score}");

    Ok(())
}
