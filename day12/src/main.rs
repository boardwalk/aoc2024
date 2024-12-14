use anyhow::Error;
use ndarray::Array2;

const PART_TWO: bool = false;

struct EvalResult {
    area: usize,
    perimeter: usize,
}

struct Work {
    pos: (usize, usize),
}

fn eval_region(plots: &Array2<char>, seen: &mut Array2<bool>, start: (usize, usize)) -> EvalResult {
    let region_plant = plots[start];
    let mut queue: Vec<Work> = Vec::new();
    queue.push(Work { pos: start });

    let mut area = 0;
    let mut perimeter = 0;

    while let Some(work) = queue.pop() {
        if seen[work.pos] {
            continue;
        }
        perimeter += 4;

        seen[work.pos] = true;
        area += 1;

        for (dr, dc) in tools::DELTAS {
            let Some(new_pos) = tools::shift(plots, work.pos, *dr, *dc) else {
                continue;
            };

            if plots[new_pos] == region_plant {
                perimeter -= 1;
                queue.push(Work { pos: new_pos });
            }
        }
    }

    EvalResult { area, perimeter }
}

fn main() -> Result<(), Error> {
    let plots = tools::load_grid(std::io::stdin().lock())?;

    let mut seen: Array2<bool> = Array2::default(plots.raw_dim());

    let mut total_cost = 0;

    for (start, _plant) in plots.indexed_iter() {
        if !seen[start] {
            let result = eval_region(&plots, &mut seen, start);

            total_cost += result.area * result.perimeter;
        }
    }

    println!("{total_cost}");
    Ok(())
}
