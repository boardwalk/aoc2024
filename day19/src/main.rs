#![feature(iterator_try_collect)]
#![feature(hash_set_entry)]
use anyhow::Error;
use std::collections::HashMap;

const PART_TWO: bool = true;

#[derive(Debug)]
struct Towels {
    patterns: Vec<String>,
    goals: Vec<String>,
}

fn inner_thing(patterns: &[String], goal: &str) -> HashMap<usize, usize> {
    let mut res = HashMap::new();

    for pattern in patterns {
        if goal.starts_with(pattern) {
            let count = res.entry(pattern.len()).or_default();
            *count += 1;
        }
    }

    res
}

// len_to_count is goal len to number of solutions

fn count_designs2_inner(
    patterns: &[String],
    goal: &str,
    len_to_count: &mut HashMap<usize, usize>,
) -> usize {
    let mut res = 0;

    // if we already have an answer, just use it
    if let Some(count) = len_to_count.get(&goal.len()) {
        return *count;
    }

    for (len, count) in inner_thing(patterns, goal) {
        let sub_solutions = count_designs2_inner(patterns, &goal[len..], len_to_count);

        res = res + (sub_solutions * count);
    }

    len_to_count.insert(goal.len(), res);

    res
}

fn count_designs2(patterns: &[String], goal: &str) -> usize {
    let mut len_to_count = HashMap::new();
    len_to_count.insert(0, 1);

    count_designs2_inner(patterns, goal, &mut len_to_count)
}

fn parse_towels() -> Result<Towels, Error> {
    let mut lines: Vec<String> = std::io::stdin().lines().try_collect()?;

    let patterns: Vec<String> = lines.remove(0).split(", ").map(|s| s.to_owned()).collect();
    assert!(lines.remove(0).is_empty());

    Ok(Towels {
        patterns,
        goals: lines,
    })
}

fn main() -> Result<(), Error> {
    let towels = parse_towels()?;
    println!("{towels:?}");

    if PART_TWO {
        let num_designs: usize = towels
            .goals
            .iter()
            .map(|goal| count_designs2(&towels.patterns, goal))
            .sum();

        println!("num_designs = {num_designs}");
    } else {
        let num_possible: usize = towels
            .goals
            .iter()
            .filter(|goal| count_designs2(&towels.patterns, goal) != 0)
            .count();

        println!("num_possible = {num_possible}");
    }
    Ok(())
}
