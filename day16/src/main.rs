#![allow(dead_code)]
use anyhow::Error;
use ndarray::Array2;
use std::collections::{BTreeSet, BinaryHeap};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

// this is kinda dumb and could be some basic math
fn valid_dir_change(src: Direction, dst: Direction) -> bool {
    match (src, dst) {
        (Direction::Up, Direction::Left) => true,
        (Direction::Up, Direction::Right) => true,
        (Direction::Down, Direction::Left) => true,
        (Direction::Down, Direction::Right) => true,
        (Direction::Left, Direction::Up) => true,
        (Direction::Left, Direction::Down) => true,
        (Direction::Right, Direction::Up) => true,
        (Direction::Right, Direction::Down) => true,
        _ => false,
    }
}

#[derive(Debug)]
struct Graph {
    nrows: usize,
    ncols: usize,
    mat: Array2<bool>,
}

fn vert_to_dir(vert: usize) -> Direction {
    match vert % 4 {
        0 => Direction::Up,
        1 => Direction::Right,
        2 => Direction::Down,
        3 => Direction::Left,
        _ => unreachable!(),
    }
}

fn vert_to_pos(grid: &Array2<char>, vert: usize) -> (usize, usize) {
    let (_nrows, ncols) = grid.dim();
    // shift out dir first
    let vert = vert / 4;
    let col = vert % ncols;
    let row = vert / ncols;
    (row, col)
}

fn shift_pos(pos: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
    match dir {
        Direction::Up => {
            if pos.0 != 0 {
                Some((pos.0 - 1, pos.1))
            } else {
                None
            }
        }
        Direction::Down => {
            Some((pos.0 + 1, pos.1))
            //
        }
        Direction::Left => {
            if pos.1 != 0 {
                Some((pos.0, pos.1 - 1))
            } else {
                None
            }
        }
        Direction::Right => Some((pos.0, pos.1 + 1)),
    }
}

fn has_edge(grid: &Array2<char>, src_vert: usize, dst_vert: usize) -> bool {
    let src_pos = vert_to_pos(grid, src_vert);
    let dst_pos = vert_to_pos(grid, dst_vert);
    let src_dir = vert_to_dir(src_vert);
    let dst_dir = vert_to_dir(dst_vert);

    // must start and end on a valid spot
    if grid[src_pos] != '.' {
        return false;
    }

    if grid[dst_pos] != '.' {
        return false;
    }

    let moved = src_pos != dst_pos;
    let turned = src_dir != dst_dir;

    match (moved, turned) {
        (true, true) => {
            // two changes, no edge
            return false;
        }
        (true, false) => {
            // moved
            // validate pos change
            if shift_pos(src_pos, src_dir) != Some(dst_pos) {
                return false;
            }
        }
        (false, true) => {
            // turned
            // validate dir change
            // can only turn left or right one notch
            if !valid_dir_change(src_dir, dst_dir) {
                return false;
            }
        }
        (false, false) => {
            // no change, no edge
            return false;
        }
    }
    println!("edge at {src_pos:?}, {dst_pos:?}, {src_dir:?}, {dst_dir:?}");
    true
}

impl Graph {
    fn new(grid: &Array2<char>) -> Self {
        let (nrows, ncols) = grid.dim();

        let num_verts = nrows * ncols * 4;

        println!("num_verts = {num_verts}");

        let shape = (num_verts, num_verts);

        let mat = Array2::from_shape_fn(shape, |(src_vert, dst_vert)| {
            has_edge(grid, src_vert, dst_vert)
        });

        Self { nrows, ncols, mat }
    }
}

#[derive(Clone, Copy, Debug)]
struct Node {
    pos: (usize, usize),
    dir: Direction,
    cost: usize,
    num_turn: usize,
    num_step: usize,
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.dir == other.dir && self.cost == other.cost
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost).reverse())
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reverse order for min heap
        self.cost.cmp(&other.cost).reverse()
    }
}

fn find_neighbors(grid: &Array2<char>, node: Node) -> Vec<Node> {
    let mut neighbors = Vec::new();

    // turn left neighbor
    neighbors.push(Node {
        pos: node.pos,
        dir: node.dir.turn_left(),
        cost: node.cost + 1000,
        num_turn: node.num_turn + 1,
        num_step: node.num_step,
    });

    // turn right neighbor
    neighbors.push(Node {
        pos: node.pos,
        dir: node.dir.turn_right(),
        cost: node.cost + 1000,
        num_turn: node.num_turn + 1,
        num_step: node.num_step,
    });

    // continue neighbor
    if let Some(new_pos) = shift_pos(node.pos, node.dir) {
        // println!(
        //     "shift res for {:?} in {:?} is {:?}",
        //     node.pos, node.dir, new_pos
        // );
        if let Some(ch) = grid.get(new_pos) {
            if *ch == '.' {
                neighbors.push(Node {
                    pos: new_pos,
                    dir: node.dir,
                    cost: node.cost + 1,
                    num_turn: node.num_turn,
                    num_step: node.num_step + 1,
                });
            }
        }
    }

    // println!("found neighbors {neighbors:?} of {node:?}");

    neighbors
}

fn ucs(grid: &mut Array2<char>) {
    let start = grid
        .indexed_iter()
        .find_map(|(pos, ch)| if *ch == 'S' { Some(pos) } else { None })
        .unwrap();

    let end = grid
        .indexed_iter()
        .find_map(|(pos, ch)| if *ch == 'E' { Some(pos) } else { None })
        .unwrap();

    println!("starting at {start:?}");
    println!("ending at {end:?}");

    grid[start] = '.';
    grid[end] = '.';

    let mut node = Node {
        pos: start,
        dir: Direction::Right,
        cost: 0,
        num_turn: 0,
        num_step: 0,
    };
    let mut frontier: BinaryHeap<Node> = BinaryHeap::new();
    let mut expanded: BTreeSet<Node> = BTreeSet::new();

    frontier.push(node);

    loop {
        // grid[node.pos] = 'X';
        // println!("{grid:?}");
        // grid[node.pos] = '.';
        node = frontier.pop().unwrap();

        println!(
            "on node {:?}, frontier len is {}, expanded len is {}",
            node,
            frontier.len(),
            expanded.len()
        );

        // std::thread::sleep(std::time::Duration::from_millis(1000));

        if node.pos == end {
            panic!("aw yeah: {node:?}");
        }

        expanded.insert(node);

        for n in find_neighbors(&grid, node) {
            if !expanded
                .iter()
                .any(|exp| (exp.pos, exp.dir) == (n.pos, n.dir))
                && !frontier.iter().any(|f| (f.pos, f.dir) == (n.pos, n.dir))
            {
                frontier.push(n);
            }
            let mut frontier_tmp: Vec<_> = frontier.iter().copied().collect();

            if let Some(idx) = frontier_tmp
                .iter()
                .position(|f| (f.pos, f.dir) == (n.pos, n.dir) && f.cost > n.cost)
            {
                frontier_tmp[idx] = n;

                frontier = frontier_tmp.iter().copied().collect();
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let (mut grid, _extra) = tools::load_grid(std::io::stdin().lock())?;

    // let graph = Graph::new(&grid);

    // println!("{grid:?}");
    // println!("{graph:?}");

    ucs(&mut grid);
    Ok(())
}
