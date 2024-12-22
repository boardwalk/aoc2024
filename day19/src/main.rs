#![feature(iterator_try_collect)]
#![feature(hash_set_entry)]

use anyhow::Error;
use std::collections::HashSet;

#[derive(Debug)]
struct Towels {
    patterns: Vec<String>,
    goals: Vec<String>,
}

fn design_possible(patterns: &[String], goal: &str) -> bool {
    println!("design_possible() with goal {goal:?}");

    let mut prototype_queue: Vec<String> = Vec::new();
    prototype_queue.push("".to_owned());
    let mut seen_prototypes: HashSet<String> = HashSet::new();
    while let Some(mut prototype) = prototype_queue.pop() {
        // don't follow prototypes multiple times
        let old_seen_len = seen_prototypes.len();
        seen_prototypes.get_or_insert_with(&prototype, String::clone);
        if seen_prototypes.len() == old_seen_len {
            continue;
        }

        for pattern in patterns {
            // skip pattern if not short enough to fit
            if prototype.len() + pattern.len() > goal.len() {
                continue;
            }

            // skip pattern if would not match
            prototype.push_str(pattern);
            let pattern_works = goal.starts_with(&prototype);
            for _i in 0..pattern.chars().count() {
                prototype.pop().unwrap();
            }

            if !pattern_works {
                continue;
            }

            let mut extended_prototype = prototype.to_owned();
            extended_prototype.push_str(pattern);
            if extended_prototype == goal {
                return true;
            }

            prototype_queue.push(extended_prototype);
        }
    }

    false
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

    let num_possible: usize = towels
        .goals
        .iter()
        .filter(|goal| design_possible(&towels.patterns, goal))
        .count();

    println!("{num_possible}");
    Ok(())
}
