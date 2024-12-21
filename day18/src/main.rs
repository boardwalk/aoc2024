use anyhow::{anyhow, bail, Error};

use crate::heap::*;
use ndarray::Array2;
use std::collections::HashSet;
use std::str::FromStr as _;

mod heap;

// const WIDTH: usize = 7;
// const HEIGHT: usize = 7;
const WIDTH: usize = 71;
const HEIGHT: usize = 71;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Node {
    steps: usize,
    pos: (usize, usize),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps.cmp(&other.steps)
    }
}

impl Node {
    fn is_end(&self) -> bool {
        self.pos.0 == WIDTH - 1 && self.pos.1 == HEIGHT - 1
    }
}

fn find_neighbors(grid: &Array2<bool>, node: Node) -> Vec<Node> {
    let mut result = Vec::new();

    for (dr, dc) in tools::DELTAS {
        let Some(new_pos) = tools::shift(grid, node.pos, *dr, *dc) else {
            continue;
        };

        if grid[new_pos] {
            continue;
        }

        result.push(Node {
            steps: node.steps + 1,
            pos: new_pos,
        });
    }

    result
}

fn main() -> Result<(), Error> {
    let mut grid = Array2::from_elem((WIDTH, HEIGHT), false);

    let mut drops = Vec::new();

    for ln in std::io::stdin().lines() {
        let ln = ln?;
        let (x, y) = ln
            .split_once(',')
            .ok_or_else(|| anyhow!("bad coordinate"))?;
        let x = usize::from_str(x)?;
        let y = usize::from_str(y)?;

        drops.push((x, y));
    }

    for (x, y) in drops.iter().copied().take(1024) {
        grid[(x, y)] = true;
    }

    let mut frontier = Vec::new();
    let mut expanded: HashSet<(usize, usize)> = HashSet::new();

    heap_push(
        &mut frontier,
        Node {
            steps: 0,
            pos: (0, 0),
        },
    );

    loop {
        let Some(node) = heap_pop(&mut frontier) else {
            bail!("did not reach goal");
        };

        if node.is_end() {
            println!("reached end in {} steps", node.steps);
            break Ok(());
        }

        expanded.insert(node.pos);

        for n in find_neighbors(&grid, node) {
            // frontier lookup here is o(n)
            let mut frontier_idx = frontier.iter().position(|f| f.pos == n.pos);

            if !expanded.contains(&n.pos) && frontier_idx.is_none() {
                frontier_idx = Some(heap_push(&mut frontier, n));
            }

            if let Some(frontier_idx) = frontier_idx {
                if frontier[frontier_idx].steps > n.steps {
                    // decreasing number of steps in frontier
                    frontier[frontier_idx] = n;
                    heap_decrease(&mut frontier, frontier_idx);
                }
            }
            //
        }
    }
}
