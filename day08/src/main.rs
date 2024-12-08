#![feature(array_windows)]

use anyhow::Error;
use ndarray::Array2;
use std::collections::{HashMap, HashSet};

type Antennae = HashMap<char, Vec<(usize, usize)>>;
type AntiNodes = HashSet<(usize, usize)>;

fn find_antennae(grid: &ndarray::Array2<char>) -> Antennae {
    let mut antennae: Antennae = HashMap::new();
    for ((row, col), freq) in grid.indexed_iter() {
        if freq.is_ascii_lowercase() || freq.is_ascii_uppercase() || freq.is_ascii_digit() {
            antennae.entry(*freq).or_default().push((row, col));
        }
    }

    antennae
}

// a, b, c

fn coord_to_signed(x: &(usize, usize)) -> (i64, i64) {
    let r = i64::try_from(x.0).unwrap();
    let c = i64::try_from(x.1).unwrap();
    (r, c)
}

fn find_anti_node(a1: (i64, i64), a2: (i64, i64)) -> (i64, i64) {
    let r = a1.0 + (a2.0 - a1.0) * 2;
    let c = a1.1 + (a2.1 - a1.1) * 2;
    (r, c)
}

fn bound_check_coord(c: (i64, i64), grid: &Array2<char>) -> Option<(usize, usize)> {
    let Ok(r) = usize::try_from(c.0) else {
        return None;
    };

    let Ok(c) = usize::try_from(c.1) else {
        return None;
    };

    if r >= grid.shape()[0] {
        return None;
    }

    if c >= grid.shape()[1] {
        return None;
    }

    Some((r, c))
}

fn find_anti_nodes(
    a1: &(usize, usize),
    a2: &(usize, usize),
    anti_nodes: &mut AntiNodes,
    grid: &Array2<char>,
) {
    let a1 = coord_to_signed(a1);
    let a2 = coord_to_signed(a2);
    let n1 = find_anti_node(a1, a2);
    let n2 = find_anti_node(a2, a1);

    if let Some(n1) = bound_check_coord(n1, grid) {
        anti_nodes.insert(n1);
    }

    if let Some(n1) = bound_check_coord(n2, grid) {
        anti_nodes.insert(n1);
    }
}

fn main() -> Result<(), Error> {
    let mut grid = tools::load_grid(std::io::stdin().lock())?;
    // println!("{grid:?}");

    let antennae = find_antennae(&grid);
    println!("{antennae:?}");

    let mut anti_nodes = AntiNodes::new();

    for (_freq, antennae) in &antennae {
        for i in 0..antennae.len() {
            for j in i + 1..antennae.len() {
                find_anti_nodes(&antennae[i], &antennae[j], &mut anti_nodes, &grid);
            }
        }
    }

    for anti_node in &anti_nodes {
        grid[*anti_node] = '#';
    }

    println!("antinode count = {}", anti_nodes.len());

    println!("{grid:?}");

    Ok(())
}

// 1 2 3 4 5
