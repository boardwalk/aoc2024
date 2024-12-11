#![feature(iterator_try_collect)]

use anyhow::Error;
use std::io::Read as _;
use std::str::FromStr as _;

fn split_num(num: usize) -> Option<(usize, usize)> {
    let num = num.to_string();

    if num.len() % 2 != 0 {
        return None;
    }
    let (left, right) = num.split_at(num.len() / 2);
    let left = usize::from_str(left).unwrap();
    let right = usize::from_str(right).unwrap();
    Some((left, right))
}

fn handle_stone(num: usize, out: &mut Vec<usize>) {
    if num == 0 {
        // rule 0
        out.push(1);
    } else if let Some((left, right)) = split_num(num) {
        out.push(left);
        out.push(right);
        // rule 1
    } else {
        // rule 2
        out.push(num * 2024);
    }
}

fn main() -> Result<(), Error> {
    let mut buf = String::new();

    std::io::stdin().read_to_string(&mut buf)?;

    let mut stones: Vec<usize> = buf
        .split_ascii_whitespace()
        .map(usize::from_str)
        .try_collect()?;

    for i in 0..25 {
        let mut new_stones = Vec::with_capacity(stones.len());

        for num in &stones {
            handle_stone(*num, &mut new_stones);
        }
        stones = new_stones;
        println!("{} after {}", stones.len(), i);
    }

    Ok(())
}
