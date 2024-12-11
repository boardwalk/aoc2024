#![feature(iterator_try_collect)]

use anyhow::{anyhow, Error};
use std::collections::HashMap;
use std::io::Read as _;
use std::str::FromStr as _;

const PART_TWO: bool = true;

pub struct Pebble {
    num: usize,
    count: usize,
}

fn split_num(val: usize) -> Option<(usize, usize)> {
    let str_val = val.to_string();
    if str_val.len() % 2 == 0 {
        let (left, right) = str_val.split_at(str_val.len() / 2);
        let left = usize::from_str(left).unwrap();
        let right = usize::from_str(right).unwrap();
        Some((left, right))
    } else {
        None
    }
}

fn compress(pebbles: &[Pebble]) -> Vec<Pebble> {
    let mut num_to_count: HashMap<usize, usize> = HashMap::new();

    for pebble in pebbles {
        let count = num_to_count.entry(pebble.num).or_default();
        *count += pebble.count;
    }

    num_to_count
        .iter()
        .map(|(num, count)| Pebble {
            num: *num,
            count: *count,
        })
        .collect()
}

fn main() -> Result<(), Error> {
    let mut buf = String::new();

    std::io::stdin().read_to_string(&mut buf)?;

    let input: Vec<usize> = buf
        .split_ascii_whitespace()
        .map(usize::from_str)
        .try_collect()?;

    let mut pebbles: Vec<Pebble> = input
        .iter()
        .map(|num| Pebble {
            num: *num,
            count: 1,
        })
        .collect();

    let num_iters = if PART_TWO { 75 } else { 25 };
    for _i in 0..num_iters {
        let mut new_pebbles = Vec::new();

        for pebble in &pebbles {
            if pebble.num == 0 {
                // rule 0
                new_pebbles.push(Pebble {
                    num: 1,
                    count: pebble.count,
                });
            } else if let Some((left, right)) = split_num(pebble.num) {
                // rule 1
                new_pebbles.push(Pebble {
                    num: left,
                    count: pebble.count,
                });
                new_pebbles.push(Pebble {
                    num: right,
                    count: pebble.count,
                });
            } else {
                // rule 2
                let num = pebble
                    .num
                    .checked_mul(2024)
                    .ok_or_else(|| anyhow!("bad mul"))?;
                let count = pebble.count;
                new_pebbles.push(Pebble { num, count });
            }
        }

        pebbles = compress(&new_pebbles);
    }

    let mut total = 0usize;

    for pebble in &pebbles {
        total = total
            .checked_add(pebble.count)
            .ok_or_else(|| anyhow!("bad add"))?;
    }

    println!("total = {total}");

    Ok(())
}
