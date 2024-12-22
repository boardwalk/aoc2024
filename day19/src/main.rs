#![feature(iterator_try_collect)]

use anyhow::Error;

#[derive(Debug)]
struct Towels {
    patterns: Vec<String>,
    designs: Vec<String>,
}

fn design_possible(patterns: &[String], design: &str) -> bool {
    false
}

fn parse_towels() -> Result<Towels, Error> {
    let mut lines: Vec<String> = std::io::stdin().lines().try_collect()?;

    let patterns: Vec<String> = lines.remove(0).split(", ").map(|s| s.to_owned()).collect();
    assert!(lines.remove(0).is_empty());

    Ok(Towels {
        patterns,
        designs: lines,
    })
}

fn main() -> Result<(), Error> {
    let towels = parse_towels()?;
    println!("{towels:?}");

    let num_possible: usize = towels
        .designs
        .iter()
        .filter(|design| design_possible(&towels.patterns, design))
        .count();
    println!("{num_possible}");
    Ok(())
}
