use anyhow::{anyhow, Error};
use ndarray::Array2;
use std::collections::HashSet;
use std::fmt::Write as _;

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

const PART_TWO: bool = true;

const DELTAS: &[(i64, i64)] = &[(0, 1), (0, -1), (-1, 0), (1, 0)];

struct Work {
    pos: (usize, usize),
    path: String,
}

fn amend_path(path: &str, dir_idx: usize) -> String {
    let mut path = path.to_owned();
    write!(&mut path, "{dir_idx}").unwrap();
    path
}

fn eval_trailhead(heights: &Array2<char>, pos: (usize, usize)) -> Result<usize, Error> {
    let mut seen: Array2<bool> = Array2::default(heights.raw_dim());
    let mut work_queue = Vec::new();
    let mut paths: HashSet<String> = HashSet::new();
    work_queue.push(Work {
        pos,
        path: String::new(),
    });

    while let Some(work) = work_queue.pop() {
        seen[work.pos] = true;
        let height = get_height(heights, work.pos)?;
        if height == 9 {
            paths.insert(work.path.clone());
        }

        for (dir_idx, (dr, dc)) in DELTAS.iter().enumerate() {
            let Some(pos) = shift(heights, work.pos, *dr, *dc) else {
                continue;
            };

            let new_height = get_height(heights, pos)?;

            if new_height == height + 1 {
                work_queue.push(Work {
                    pos,
                    path: amend_path(&work.path, dir_idx),
                });
            }
        }
    }

    if PART_TWO {
        Ok(paths.len())
    } else {
        let mut score = 0;
        for (pos, val) in heights.indexed_iter() {
            if *val == '9' && seen[pos] {
                score += 1;
            }
        }

        Ok(score)
    }
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
