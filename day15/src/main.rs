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
        '<' => Ok((-1, 0)),
        '>' => Ok((1, 0)),
        '^' => Ok((0, -1)),
        'v' => Ok((0, 1)),
        _ => Err(anyhow!("bad instr")),
    }
}

fn push_box(grid: &Array2<char>, pos: (usize, usize), (dr, dc): (i64, i64)) -> bool {
    false
}

fn do_delta(
    grid: &mut Array2<char>,
    cur_pos: &mut (usize, usize),
    (dr, dc): (i64, i64),
) -> Result<(), Error> {
    let Some(new_pos) = tools::shift(grid, *cur_pos, dr, dc) else {
        return Ok(());
    };

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
            if push_box(grid, new_pos, (dr, dc)) {
                *cur_pos = new_pos;
            }
        }
        _ => {
            bail!("bad grid char");
        }
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let (mut grid, extra) = tools::load_grid(std::io::stdin().lock())?;
    let extra = extra.ok_or_else(|| anyhow!("missing extra"))?;
    println!("{grid:?}, {extra:?}");
    let mut cur_pos = find_bot(&grid).ok_or_else(|| anyhow!("missing bot"))?;
    // we know where we are!
    grid[cur_pos] = '.';

    for instr in extra.chars() {
        let delta = instr_to_delta(instr)?;
        do_delta(&mut grid, &mut cur_pos, delta)?;
    }

    Ok(())
}
