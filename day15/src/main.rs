use anyhow::{anyhow, bail, Error};
use ndarray::Array2;

const PART_TWO: bool = true;

fn find_bot(grid: &Array2<char>) -> Option<(usize, usize)> {
    grid.indexed_iter()
        .find_map(|(pos, ch)| if *ch == '@' { Some(pos) } else { None })
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

fn do_delta_p1(
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

fn do_delta_p2(
    grid: &mut Array2<char>,
    cur_pos: &mut (usize, usize),
    (dr, dc): (i64, i64),
) -> Result<(), Error> {
    // todo!()
    Ok(())
}

fn calc_gps_sum_p1(grid: &Array2<char>) -> usize {
    let mut gps_sum = 0;
    for (pos, ch) in grid.indexed_iter() {
        if *ch == 'O' {
            gps_sum += 100 * pos.0 + pos.1;
        }
    }

    gps_sum
}

// given position of right edge of box
fn dist_from_top(_grid: &Array2<char>, pos: (usize, usize)) -> usize {
    pos.0
}

// given position of right edge of box
fn dist_from_right(grid: &Array2<char>, pos: (usize, usize)) -> usize {
    grid.dim().1 - pos.1
}

fn calc_gps_sum_p2(grid: &Array2<char>) -> usize {
    let mut in_box = false;
    let mut gps_sum = 0;
    for (pos, ch) in grid.indexed_iter() {
        if *ch == '[' {
            assert!(!in_box);
            in_box = true;
        } else if *ch == ']' {
            assert!(in_box);
            in_box = false;
            gps_sum += 100 * dist_from_top(&grid, pos) + dist_from_right(&grid, pos);
        } else {
            // ignore
        }
    }
    gps_sum
}

fn widen_grid(old_grid: &Array2<char>) -> Result<Array2<char>, Error> {
    let old_shape = old_grid.raw_dim();
    let new_shape = [old_shape[0], old_shape[1] * 2];

    let mut new_grid = Array2::<char>::from_elem(new_shape, ' ');

    for (old_pos, old_ch) in old_grid.indexed_iter() {
        let new_pos_1 = (old_pos.0, old_pos.1 * 2);
        let new_pos_2 = (old_pos.0, old_pos.1 * 2 + 1);
        let (new_ch_1, new_ch_2) = match *old_ch {
            '#' => ('#', '#'),
            'O' => ('[', ']'),
            '.' => ('.', '.'),
            '@' => ('@', '.'),
            _ => bail!("bad grid char"),
        };

        new_grid[new_pos_1] = new_ch_1;
        new_grid[new_pos_2] = new_ch_2;
    }

    Ok(new_grid)
}

fn main() -> Result<(), Error> {
    let (mut grid, extra) = tools::load_grid(std::io::stdin().lock())?;
    let extra = extra.ok_or_else(|| anyhow!("missing extra"))?;
    if PART_TWO {
        grid = widen_grid(&grid)?;
    }

    println!("{grid:?}");

    let mut cur_pos = find_bot(&grid).ok_or_else(|| anyhow!("missing bot"))?;

    for instr in extra.chars() {
        let delta = instr_to_delta(instr)?;
        if PART_TWO {
            do_delta_p2(&mut grid, &mut cur_pos, delta)?;
        } else {
            do_delta_p1(&mut grid, &mut cur_pos, delta)?;
        }
        print!("\n\n\n\n\n\n\n");
        println!("{instr}\n {grid:?}");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    if PART_TWO {
        println!("gps_sum = {}", calc_gps_sum_p2(&grid));
    } else {
        println!("gps_sum = {}", calc_gps_sum_p1(&grid));
    }

    Ok(())
}
