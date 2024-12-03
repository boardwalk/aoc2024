#![feature(array_windows)]

use anyhow::Error;
use std::str::FromStr;

const PART_TWO: bool = true;

struct Report {
    levels: Vec<usize>,
}

fn map_index(idx: usize, excluded_idx: Option<usize>) -> usize {
    if let Some(excluded_idx) = excluded_idx {
        // if we're pretending an index was deleted, indices >= the deleted index are at idx + 1 in the physical array
        if idx < excluded_idx {
            idx
        } else {
            idx + 1
        }
    } else {
        // if we're not pretending anything was deleted, no mapping is needed
        idx
    }
}

fn all_pairs_excluding<T>(
    items: &[T],
    pred: impl Fn(&T, &T) -> bool,
    excluded_idx: Option<usize>,
) -> bool {
    for idx in 0..items.len() - 1 {
        let idx_1 = map_index(idx, excluded_idx);
        let idx_2 = map_index(idx + 1, excluded_idx);

        let Some(item_1) = items.get(idx_1) else {
            break;
        };

        let Some(item_2) = items.get(idx_2) else {
            break;
        };

        if !pred(item_1, item_2) {
            return false;
        }
    }
    true
}

impl Report {
    fn is_safe_excluding(&self, excluded_idx: Option<usize>) -> bool {
        let all_incr = all_pairs_excluding(&self.levels, |a, b| *a > *b, excluded_idx);
        let all_decr = all_pairs_excluding(&self.levels, |a, b| *a < *b, excluded_idx);

        if !(all_incr || all_decr) {
            return false;
        }

        all_pairs_excluding(
            &self.levels,
            |a, b| matches!(a.abs_diff(*b), 1 | 2 | 3),
            excluded_idx,
        )
    }
}

impl FromStr for Report {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut levels = Vec::new();
        for token in s.split_ascii_whitespace() {
            let level = usize::from_str(token)?;
            levels.push(level)
        }

        Ok(Self { levels })
    }
}

fn main() -> Result<(), Error> {
    let mut reports = Vec::new();

    for ln in std::io::stdin().lines() {
        let ln = ln?;
        reports.push(Report::from_str(&ln)?);
    }

    let num_safe = reports
        .iter()
        .map(|report| {
            if PART_TWO {
                (0..report.levels.len())
                    .any(|excluded_idx| report.is_safe_excluding(Some(excluded_idx)))
            } else {
                report.is_safe_excluding(None)
            }
        })
        .filter(|x| *x)
        .count();

    println!("{num_safe}");

    Ok(())
}
