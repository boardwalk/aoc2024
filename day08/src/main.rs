#![feature(array_windows)]

use anyhow::Error;
use ndarray::Array2;
use std::collections::{HashMap, HashSet};

type Antennas = HashMap<char, Vec<(usize, usize)>>;
type AntiNodes = HashSet<(usize, usize)>;

const PART_TWO: bool = true;

fn find_antennas(grid: &ndarray::Array2<char>) -> Antennas {
    let mut antennas: Antennas = HashMap::new();
    for ((row, col), freq) in grid.indexed_iter() {
        if freq.is_ascii_lowercase() || freq.is_ascii_uppercase() || freq.is_ascii_digit() {
            antennas.entry(*freq).or_default().push((row, col));
        }
    }

    antennas
}

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

fn find_anti_nodes_p1(
    a1: &(usize, usize),
    a2: &(usize, usize),
    anti_nodes: &mut AntiNodes,
    grid: &Array2<char>,
) {
    let a1 = coord_to_signed(a1);
    let a2 = coord_to_signed(a2);
    let n = find_anti_node(a1, a2);

    if let Some(n) = bound_check_coord(n, grid) {
        anti_nodes.insert(n);
    }
}

fn find_anti_nodes_p2(
    a1: &(usize, usize),
    a2: &(usize, usize),
    anti_nodes: &mut AntiNodes,
    grid: &Array2<char>,
) {
    let a1 = coord_to_signed(a1);
    let a2 = coord_to_signed(a2);

    let dr = a2.0 - a1.0;
    let dc = a2.1 - a1.1;

    let mut r = a1.0;
    let mut c = a1.1;

    loop {
        let Some(n) = bound_check_coord((r, c), grid) else {
            break;
        };
        anti_nodes.insert(n);

        r += dr;
        c += dc;
    }
}

fn main() -> Result<(), Error> {
    let (mut grid, _extra) = tools::load_grid(std::io::stdin().lock())?;
    // println!("{grid:?}");

    let antennas = find_antennas(&grid);
    println!("{antennas:?}");

    let mut anti_nodes = AntiNodes::new();

    for (_freq, antennae) in &antennas {
        for i in 0..antennae.len() {
            for j in i + 1..antennae.len() {
                if PART_TWO {
                    find_anti_nodes_p2(&antennae[i], &antennae[j], &mut anti_nodes, &grid);
                    find_anti_nodes_p2(&antennae[j], &antennae[i], &mut anti_nodes, &grid);
                } else {
                    find_anti_nodes_p1(&antennae[i], &antennae[j], &mut anti_nodes, &grid);
                    find_anti_nodes_p1(&antennae[j], &antennae[i], &mut anti_nodes, &grid);
                }
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
