#![feature(array_windows)]

use anyhow::{anyhow, Error};
use std::io::Read as _;

#[derive(Debug, Hash)]
struct Block {
    file_id: usize,
    file_block: usize,
    disk_block: usize,
}

fn parse_disk_map(disk_map: &str) -> Result<Vec<Block>, Error> {
    let mut blocks = Vec::new();
    let mut disk_block = 0;
    let mut file_id = 0;
    let mut map_it = disk_map.chars();
    loop {
        let Some(length) = map_it.next() else {
            break;
        };

        let length = length.to_digit(10).ok_or_else(|| anyhow!("invalid used"))?;
        let length = usize::try_from(length)?;

        for file_block in 0..length {
            blocks.push(Block {
                file_id,
                file_block,
                disk_block,
            });

            disk_block += 1;
        }

        file_id += 1;
        let Some(free) = map_it.next() else {
            break;
        };

        let length = free.to_digit(10).ok_or_else(|| anyhow!("invalid free"))?;
        let length = usize::try_from(length)?;
        disk_block += length;
    }

    Ok(blocks)
}

fn find_free_block(blocks: &[Block]) -> Option<usize> {
    for [a, b] in blocks.array_windows() {
        let dist = b.disk_block - a.disk_block;
        if dist > 1 {
            return Some(a.disk_block + 1);
        }
    }

    None
}

fn main() -> Result<(), Error> {
    let mut disk_map = String::new();
    std::io::stdin().read_to_string(&mut disk_map)?;

    let mut blocks = parse_disk_map(&disk_map)?;
    // kept in disk_block order

    while let Some(free_block) = find_free_block(&blocks) {
        let last_block = blocks.last_mut().unwrap();
        // println!("moving {} to {}", last_block.disk_block, free_block);
        last_block.disk_block = free_block;
        blocks.sort_by_key(|block| block.disk_block);
    }

    let mut sum = 0;

    for block in &blocks {
        sum += block.disk_block * block.file_id
    }

    println!("{sum}");

    // println!("{blocks:#?}");

    Ok(())
}
