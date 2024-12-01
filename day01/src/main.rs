use anyhow::{bail, Error};
use std::collections::HashMap;

const PART_TWO: bool = true;

fn count_counts(nums: &[i64]) -> HashMap<i64, i64> {
    let mut counts = HashMap::new();

    for num in nums {
        let count = counts.entry(*num).or_default();
        *count += 1;
    }

    counts
}

fn main() -> Result<(), Error> {
    let mut left_lst = Vec::new();
    let mut right_lst = Vec::new();

    for ln in std::io::stdin().lines() {
        let ln = ln?;
        let tokens: Vec<_> = ln.split_ascii_whitespace().collect();

        if tokens.len() != 2 {
            bail!("invalid line in input");
        }

        let left_num: i64 = tokens[0].parse()?;
        let right_num: i64 = tokens[1].parse()?;

        left_lst.push(left_num);
        right_lst.push(right_num);
    }

    left_lst.sort();
    right_lst.sort();

    let mut total_dist: i64 = 0;

    for (left_num, right_num) in left_lst.iter().zip(right_lst.iter()) {
        let dist = (*left_num - *right_num).abs();

        total_dist += dist;
    }

    println!("part_1 = {total_dist}");

    if PART_TWO {
        let right_counts = count_counts(&right_lst);

        let mut sim_score: i64 = 0;

        for left_num in &left_lst {
            let right_count = right_counts.get(left_num).copied().unwrap_or_default();

            sim_score += *left_num * right_count;
        }

        println!("part_2 = {sim_score}");
    }

    Ok(())
}
