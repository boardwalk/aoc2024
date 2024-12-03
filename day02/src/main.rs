#![feature(array_windows)]

use anyhow::Error;
use std::str::FromStr;

const PART_TWO: bool = true;

struct Report {
    levels: Vec<usize>,
}

impl Report {
    fn is_safe_p1(&self) -> bool {
        let all_incr = self.levels.as_slice().array_windows().all(|[a, b]| *a > *b);

        let all_decr = self.levels.as_slice().array_windows().all(|[a, b]| *a < *b);

        if !(all_incr || all_decr) {
            return false;
        }

        self.levels
            .as_slice()
            .array_windows()
            .all(|[a, b]| matches!(a.abs_diff(*b), 1 | 2 | 3))
    }

    fn is_safe_p2(&mut self) -> bool {
        for i in 0..self.levels.len() {
            let val = self.levels.remove(i);
            let is_safe = self.is_safe_p1();
            self.levels.insert(i, val);
            if is_safe {
                return true;
            }
        }
        false
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
        .iter_mut()
        .map(|report| {
            if PART_TWO {
                report.is_safe_p2()
            } else {
                report.is_safe_p1()
            }
        })
        .filter(|x| *x)
        .count();

    println!("{num_safe}");

    Ok(())
}
