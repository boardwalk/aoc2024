use std::collections::HashSet;

type Grid = Vec<Vec<char>>;

const PART_TWO: bool = true;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

fn dir_from_char(ch: char) -> Option<Direction> {
    match ch {
        '^' => Some(Direction::Up),
        '<' => Some(Direction::Left),
        '>' => Some(Direction::Right),
        'v' => Some(Direction::Down),
        _ => None,
    }
}
fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
    }
}

fn get_delta(dir: Direction) -> (i64, i64) {
    match dir {
        Direction::Up => (-1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
        Direction::Down => (1, 0),
    }
}

fn find_start(grid: &Grid) -> (i64, i64, Direction) {
    for (row, r_vec) in grid.iter().enumerate() {
        for (col, cell_val) in r_vec.iter().enumerate() {
            if let Some(dir) = dir_from_char(*cell_val) {
                let row = i64::try_from(row).unwrap();
                let col = i64::try_from(col).unwrap();
                return (row, col, dir);
            }
        }
    }

    panic!("no start pos found");
}

fn get(grid: &Grid, row: i64, col: i64) -> Option<char> {
    let row = usize::try_from(row).ok()?;
    let col = usize::try_from(col).ok()?;
    let ch = grid.get(row)?.get(col)?;
    Some(*ch)
}

#[derive(Debug)]
enum EvalResult {
    Looped,
    Escaped(usize),
}

fn num_rows(grid: &Grid) -> i64 {
    i64::try_from(grid.len()).unwrap()
}

fn num_cols(grid: &Grid) -> i64 {
    i64::try_from(grid[0].len()).unwrap()
}

fn in_bounds(grid: &Grid, row: i64, col: i64) -> bool {
    row >= 0 && row < num_rows(grid) && col >= 0 && col < num_cols(grid)
}

fn eval_grid(grid: &Grid) -> EvalResult {
    let mut visited_pos_dir: HashSet<(i64, i64, Direction)> = HashSet::new();
    let mut visited_pos: HashSet<(i64, i64)> = HashSet::new();

    let (mut row, mut col, mut dir) = find_start(&grid);

    visited_pos.insert((row, col));

    visited_pos_dir.insert((row, col, dir));

    loop {
        let (dr, dc) = get_delta(dir);
        let possible_r = row + dr;
        let possible_c = col + dc;

        if !in_bounds(grid, possible_r, possible_c) {
            // out of bounds, escaped
            return EvalResult::Escaped(visited_pos.len());
        }

        if get(&grid, possible_r, possible_c) == Some('#') {
            // obstacle in way, turn
            dir = turn_right(dir);
        } else {
            // in bounds and no obstacle, move
            row = possible_r;
            col = possible_c;
            visited_pos.insert((row, col));
            if !visited_pos_dir.insert((row, col, dir)) {
                return EvalResult::Looped;
            }
        }
    }
}

fn grid_put(grid: &mut Grid, row: i64, col: i64, ch: char) {
    let row = usize::try_from(row).unwrap();
    let col = usize::try_from(col).unwrap();
    grid[row][col] = ch;
}

fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for ln in std::io::stdin().lines() {
        let ln = ln.unwrap();
        grid.push(ln.chars().collect());
    }

    if PART_TWO {
        let mut num_loops = 0;
        for row in 0..num_rows(&grid) {
            for col in 0..num_cols(&grid) {
                // println!("on {row}, {col}");
                let ch = get(&grid, row, col).unwrap();

                if ch == '#' {
                    // obstacle already here
                    continue;
                }

                if dir_from_char(ch).is_some() {
                    //start point already here
                    continue;
                }

                grid_put(&mut grid, row, col, '#');
                let res = eval_grid(&grid);

                match res {
                    EvalResult::Looped => {
                        num_loops += 1;
                    }
                    EvalResult::Escaped(_) => (),
                }

                grid_put(&mut grid, row, col, ch);
            }
        }

        println!("num_loops = {num_loops}");
    } else {
        let res = eval_grid(&grid);

        println!("{res:?}");
    }
}
