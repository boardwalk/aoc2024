use anyhow::{anyhow, bail, Error};
use ndarray::Array2;

fn find_bot(grid: &Array2<char>) -> Option<(usize, usize)> {
    for (pos, ch) in grid.indexed_iter() {
        if *ch == '@' {
            return Some(pos);
        }
    }

    None
}

fn instr_to_delta(instr: char) -> Result<(i64, i64), Error> {
    match instr {
        '<' => Ok((-0, -1)),
        '>' => Ok((0, 1)),
        '^' => Ok((-1, 0)),
        'v' => Ok((1, 0)),
        _ => Err(anyhow!("bad instr")),
    }
}

fn push_box(
    grid: &mut Array2<char>,
    pos: (usize, usize),
    (dr, dc): (i64, i64),
) -> Result<bool, Error> {
    let mut test_pos = pos;
    loop {
        match grid[test_pos] {
            '#' => {
                // can't push into wall
                return Ok(false);
            }
            '.' => {
                // found an empty space to push into
                break;
            }
            'O' => {
                // found a box to push
                let Some(new_pos) = tools::shift(grid, test_pos, dr, dc) else {
                    return Ok(false);
                };
                test_pos = new_pos;
            }
            _ => {
                bail!("bad grid char");
            }
        }
    }
    // clear start
    grid[pos] = '.';
    // fill end
    grid[test_pos] = 'O';
    Ok(true)
}

fn do_delta(
    grid: &mut Array2<char>,
    cur_pos: &mut (usize, usize),
    (dr, dc): (i64, i64),
) -> Result<(), Error> {
    let Some(new_pos) = tools::shift(grid, *cur_pos, dr, dc) else {
        return Ok(());
    };
    grid[*cur_pos] = '.';

    match grid[new_pos] {
        '#' => {
            // if we're walking into a wall, do nothing
        }
        '.' => {
            // if we're moving onto a free space, do it
            *cur_pos = new_pos;
        }
        'O' => {
            // if we're pushing into a box, try that
            if push_box(grid, new_pos, (dr, dc))? {
                *cur_pos = new_pos;
            }
        }
        _ => {
            bail!("bad grid char");
        }
    }
    grid[*cur_pos] = '@';
    Ok(())
}

fn calc_gps_sum(grid: &Array2<char>) -> usize {
    let mut gps_sum = 0;
    for (pos, ch) in grid.indexed_iter() {
        if *ch == 'O' {
            gps_sum += 100 * pos.0 + pos.1;
        }
    }

    gps_sum
}

fn main() -> Result<(), Error> {
    let (mut grid, extra) = tools::load_grid(std::io::stdin().lock())?;
    let extra = extra.ok_or_else(|| anyhow!("missing extra"))?;
    println!("{grid:?}");
    let mut cur_pos = find_bot(&grid).ok_or_else(|| anyhow!("missing bot"))?;

    for instr in extra.chars() {
        let delta = instr_to_delta(instr)?;
        do_delta(&mut grid, &mut cur_pos, delta)?;
        // print!("\n\n\n\n\n\n\n");
        // println!("{instr}\n {grid:?}");
        // std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("gps_sum = {}", calc_gps_sum(&grid));

    Ok(())
}
