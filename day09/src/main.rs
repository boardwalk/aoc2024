#![feature(array_windows)]

use anyhow::{anyhow, Error};
use std::io::Read as _;

#[derive(Debug, Hash)]
struct File {
    file_id: usize,
    length: usize,
    disk_block: usize,
}

fn parse_disk_map(disk_map: &str) -> Result<Vec<File>, Error> {
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

        blocks.push(File {
            file_id,
            length,
            disk_block,
        });

        disk_block += length;
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

fn find_contig_free_of_size(files: &[File], len: usize) -> Option<usize> {
    for [a, b] in files.array_windows() {
        let a_end = a.disk_block + a.length;
        let b_begin = b.disk_block;
        let num_free = b_begin - a_end;

        if num_free >= len {
            return Some(a_end);
        }
    }

    None
}

fn main() -> Result<(), Error> {
    let mut disk_map = String::new();
    std::io::stdin().read_to_string(&mut disk_map)?;

    let mut files = parse_disk_map(&disk_map)?;
    // kept in disk_block order
    for file_id in (0..files.len()).rev() {
        let idx = files
            .iter()
            .position(|file| file.file_id == file_id)
            .unwrap();
        let Some(free_block) = find_contig_free_of_size(&files, files[idx].length) else {
            continue;
        };

        if free_block >= files[idx].disk_block {
            // we only move files left
            continue;
        }

        println!(
            "moving {} from {} to {}",
            file_id, files[idx].disk_block, free_block
        );
        files[idx].disk_block = free_block;
        files.sort_by_key(|block| block.disk_block);
    }

    let mut sum = 0;

    for file in &files {
        for file_block in 0..file.length {
            sum += (file.disk_block + file_block) * file.file_id
        }
    }

    println!("{sum}");
    Ok(())
}
