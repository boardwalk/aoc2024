use anyhow::Error;
use ndarray::Array2;

const PART_TWO: bool = false;

#[derive(Debug)]
struct EvalResult {
    area: usize,
    perimeter: usize,
    total_run: usize,
}

struct Work {
    pos: (usize, usize),
    last_delta: Option<(i64, i64)>,
    run_len: usize,
}

fn eval_region(plots: &Array2<char>, seen: &mut Array2<bool>, start: (usize, usize)) -> EvalResult {
    let region_plant = plots[start];
    let mut queue: Vec<Work> = Vec::new();
    queue.push(Work {
        pos: start,
        last_delta: None,
        run_len: 1,
    });

    let mut area = 0;
    let mut perimeter = 0;

    let mut total_run = 0;

    while let Some(work) = queue.pop() {
        if seen[work.pos] {
            continue;
        }

        // println!("run len = {}", work.run_len);

        // if work.run_len != 1 {
        perimeter += 4;
        // }

        seen[work.pos] = true;
        area += 1;

        // total_run += work.run_len;
        if work.run_len != 1 {
            total_run += 1;
        }

        for (dr, dc) in tools::DELTAS {
            let Some(new_pos) = tools::shift(plots, work.pos, *dr, *dc) else {
                continue;
            };

            if plots[new_pos] == region_plant {
                perimeter -= 1;

                if work.last_delta == Some((*dr, *dc)) {
                    queue.push(Work {
                        pos: new_pos,
                        last_delta: Some((*dr, *dc)),
                        run_len: work.run_len + 1,
                    });
                } else {
                    queue.push(Work {
                        pos: new_pos,
                        last_delta: Some((*dr, *dc)),
                        run_len: 1,
                    });
                }
            }
        }
    }

    // println!("run_len = {total_run}");

    EvalResult {
        area,
        perimeter,
        total_run,
    }
}

fn main() -> Result<(), Error> {
    let (plots, _extra) = tools::load_grid(std::io::stdin().lock())?;

    let mut seen: Array2<bool> = Array2::default(plots.raw_dim());

    let mut total_cost = 0;

    for (start, plant) in plots.indexed_iter() {
        if !seen[start] {
            let result = eval_region(&plots, &mut seen, start);

            println!("got result {:?} for region = {}", result, plant);

            total_cost += result.area * result.perimeter;
        }
    }

    println!("{total_cost}");
    Ok(())
}
