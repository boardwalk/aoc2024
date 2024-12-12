use anyhow::Error;
use ndarray::Array2;

struct EvalResult {
    area: usize,
    perimeter: usize,
}

fn eval_region(plots: &Array2<char>, seen: &mut Array2<bool>, start: (usize, usize)) -> EvalResult {
    let region_plant = plots[start];
    let mut work: Vec<(usize, usize)> = Vec::new();
    work.push(start);

    let mut area = 0;
    let mut perimeter = 0;

    while let Some(pos) = work.pop() {
        if seen[pos] {
            continue;
        }
        perimeter += 4;

        seen[pos] = true;
        area += 1;

        for (dr, dc) in tools::DELTAS {
            let Some(pos) = tools::shift(plots, pos, *dr, *dc) else {
                continue;
            };

            if plots[pos] == region_plant {
                perimeter -= 1;
                work.push(pos);
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
