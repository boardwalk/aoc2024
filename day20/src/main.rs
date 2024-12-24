use anyhow::{anyhow, bail, Error};
use ndarray::Array2;
use std::collections::{HashMap, HashSet};
use tools::{heap_decrease, heap_pop, heap_push};

fn find_pos(grid: &Array2<char>, search_char: char) -> Option<(usize, usize)> {
    for (pos, ch) in grid.indexed_iter() {
        if *ch == search_char {
            return Some(pos);
        }
    }
    None
}

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

fn find_neighbors(grid: &Array2<char>, node: Node) -> Vec<Node> {
    let mut result = Vec::new();

    for (dr, dc) in tools::DELTAS {
        let Some(new_pos) = tools::shift(grid, node.pos, *dr, *dc) else {
            continue;
        };

        if grid[new_pos] != '.' {
            continue;
        }

        result.push(Node {
            steps: node.steps + 1,
            pos: new_pos,
        });
    }

    result
}

fn find_path(
    grid: &Array2<char>,
    start: (usize, usize),
    end: (usize, usize),
) -> Result<Vec<(usize, usize)>, Error> {
    let mut frontier = Vec::new();
    let mut expanded: HashSet<(usize, usize)> = HashSet::new();

    let mut path = Vec::new();

    heap_push(
        &mut frontier,
        Node {
            steps: 0,
            pos: start,
        },
    );

    loop {
        let Some(node) = heap_pop(&mut frontier) else {
            bail!("no path found");
        };

        path.push(node.pos);

        if node.pos == end {
            break Ok(path);
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
        }
    }
}

fn draw_path(grid: &Array2<char>, path: &[(usize, usize)]) {
    let mut grid_copy = grid.clone();

    for elem in path {
        tools::print_grid(&grid_copy);
        grid_copy[*elem] = '*';
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

// // returns the minimum distance from start to end given a cheat of cheat_len starting now
// fn try_cheat(
//     grid: &Array2<char>,
//     start: (usize, usize),
//     end: (usize, usize),
//     cheat_len: usize,
//     time_saved_to_count: &mut HashMap<usize, usize>,
// ) {
//     if cheat_len == 0 {
//         if grid[start] == '.' {
//             // ended up dqd
//             return;
//         } else {

//         }

//         return find_path(grid, start, end);
//     }

//     for (dr, dc) in tools::DELTAS {
//         let Some(cheat_dest_1) = tools::shift(&grid, start, *dr, *dc) else {
//             continue;
//         };
//     }

//     0
// }

fn main() -> Result<(), Error> {
    let (mut grid, _extra) = tools::load_grid(std::io::stdin().lock())?;

    let start = find_pos(&grid, 'S').ok_or_else(|| anyhow!("no start pos"))?;
    let end = find_pos(&grid, 'E').ok_or_else(|| anyhow!("no end pos"))?;

    grid[start] = '.';
    grid[end] = '.';

    let path = find_path(&grid, start, end)?;

    let time_clean = path.len() - 1;
    println!("time clean: {time_clean}");

    // try cheating at all possible times in all possible ways
    // what a hack.

    let mut saved_to_count: HashMap<usize, usize> = HashMap::new();

    for (cheat_time, cheat_src) in path.iter().enumerate() {
        for (dr1, dc1) in tools::DELTAS {
            let Some(cheat_dest_1) = tools::shift(&grid, *cheat_src, *dr1, *dc1) else {
                continue;
            };

            // if grid[cheat_dest_1] != '#' {
            //     // not cheating into a wall makes no sense
            //     continue;
            // }

            for (dr2, dc2) in tools::DELTAS {
                let Some(cheat_dest_2) = tools::shift(&grid, cheat_dest_1, *dr2, *dc2) else {
                    continue;
                };

                // if grid[cheat_dest_2] != '.' {
                //     // must cheat back out of the wall
                //     continue;
                // }

                let Ok(path_tail) = find_path(&grid, cheat_dest_2, end) else {
                    continue;
                };

                let time_dirty = cheat_time + 2 + (path_tail.len() - 1);
                if time_dirty < time_clean {
                    let count = saved_to_count.entry(time_clean - time_dirty).or_default();
                    *count += 1;
                }
            }
        }
    }

    println!("{saved_to_count:#?}");

    Ok(())
}
