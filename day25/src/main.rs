#![feature(iterator_try_collect)]
#![feature(array_windows)]
use anyhow::{anyhow, bail, Error};
use std::io::BufRead;

#[derive(Debug, PartialEq)]
struct KeyLock {
    is_lock: bool,
    heights: [u8; 5],
}

const SAMPLE_KEY_LOCKS: [KeyLock; 5] = [
    KeyLock {
        is_lock: true,
        heights: [0, 5, 3, 4, 3],
    },
    KeyLock {
        is_lock: true,
        heights: [1, 2, 0, 5, 3],
    },
    KeyLock {
        is_lock: false,
        heights: [5, 0, 2, 1, 3],
    },
    KeyLock {
        is_lock: false,
        heights: [4, 3, 4, 0, 2],
    },
    KeyLock {
        is_lock: false,
        heights: [3, 0, 2, 0, 1],
    },
];
type KeyRow = [bool; 5];
type KeyGrid = [KeyRow; 7];

fn row_all_same(row: &KeyRow) -> bool {
    row.array_windows().all(|[a, b]| *a == *b)
}

fn parse_keylock(rows: &KeyGrid) -> Result<KeyLock, Error> {
    if !row_all_same(&rows[0]) {
        bail!("bad first row");
    }

    if !row_all_same(&rows[rows.len() - 1]) {
        bail!("bad last row");
    }

    let is_lock = rows[0][0];
    let is_key = rows[rows.len() - 1][0];

    let (is_lock, heights) = match (is_lock, is_key) {
        (true, false) => {
            // look down for locks
            let mut heights: [u8; 5] = [0, 0, 0, 0, 0];
            for col in 0..5 {
                for row in 1..7 {
                    if !rows[row][col] {
                        break;
                    }
                    heights[col] += 1;
                }
            }
            (true, heights)
        }
        (false, true) => {
            // look up for keys
            let mut heights: [u8; 5] = [0, 0, 0, 0, 0];

            for col in 0..5 {
                for row in (0..6).rev() {
                    if !rows[row][col] {
                        break;
                    }
                    heights[col] += 1;
                }
            }
            (false, heights)
        }
        _ => {
            bail!("invalid keylock first/last row");
        }
    };

    Ok(KeyLock { is_lock, heights })
}

fn parse_keylocks(rd: impl BufRead) -> Result<Vec<KeyLock>, Error> {
    let mut key_locks = Vec::new();

    let mut rows: Vec<[bool; 5]> = Vec::new();

    for ln in rd.lines() {
        let ln = ln?;

        if ln.is_empty() {
            let rows_arr: KeyGrid = rows.try_into().map_err(|_| anyhow!("bad row count"))?;
            key_locks.push(parse_keylock(&rows_arr)?);
            rows = Vec::new();
            continue;
        }

        let row: Vec<bool> = ln
            .chars()
            .map(|ch| match ch {
                '#' => Ok(true),
                '.' => Ok(false),
                _ => Err(anyhow!("invalid keylock char")),
            })
            .try_collect()?;

        let row: KeyRow = row.try_into().map_err(|_| anyhow!("bad row length"))?;

        rows.push(row);
    }

    if !rows.is_empty() {
        let rows_arr: KeyGrid = rows.try_into().map_err(|_| anyhow!("bad row count"))?;
        key_locks.push(parse_keylock(&rows_arr)?);
    }

    Ok(key_locks)
}

fn main() -> Result<(), Error> {
    let key_locks = parse_keylocks(std::io::stdin().lock())?;
    assert_eq!(key_locks, SAMPLE_KEY_LOCKS);
    Ok(())
}
