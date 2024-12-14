#![feature(iterator_try_collect)]
#![feature(array_chunks)]

use anyhow::Error;
use std::io::BufRead;
use std::str::FromStr as _;

#[derive(Debug)]
struct Vec2 {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Prize {
    a_value: Vec2,
    b_value: Vec2,
    goal: Vec2,
}

fn load_prizes(rd: impl BufRead) -> Result<Vec<Prize>, Error> {
    let re_btn_a = regex::Regex::new(r"^Button A: X\+(\d+), Y\+(\d+)$")?;
    let re_btn_b = regex::Regex::new(r"^Button B: X\+(\d+), Y\+(\d+)$")?;
    let re_prize = regex::Regex::new(r"^Prize: X=(\d+), Y=(\d+)$")?;
    let mut prizes = Vec::new();
    let mut lines = Vec::new();

    for ln in rd.lines() {
        let ln = ln?;
        if ln.is_empty() {
            continue;
        }

        lines.push(ln);
    }

    for [btn_a, btn_b, prize] in lines.array_chunks() {
        let btn_a = re_btn_a.captures(btn_a).unwrap();
        let btn_b = re_btn_b.captures(btn_b).unwrap();
        let prize = re_prize.captures(&prize).unwrap();

        prizes.push(Prize {
            a_value: Vec2 {
                x: usize::from_str(&btn_a[1])?,
                y: usize::from_str(&btn_a[2])?,
            },
            b_value: Vec2 {
                x: usize::from_str(&btn_b[1])?,
                y: usize::from_str(&btn_b[2])?,
            },
            goal: Vec2 {
                x: usize::from_str(&prize[1])?,
                y: usize::from_str(&prize[2])?,
            },
        });
    }

    Ok(prizes)
}

fn main() -> Result<(), Error> {
    let prizes = load_prizes(std::io::stdin().lock())?;

    println!("{prizes:#?}");
    println!("num prizes = {}", prizes.len());

    let mut total_cost = 0;

    for prize in &prizes {
        let mut best_cost: Option<usize> = None;
        for num_a in 0..200 {
            for num_b in 0..200 {
                let x_val = num_a * prize.a_value.x + num_b * prize.b_value.x;
                let y_val = num_a * prize.a_value.y + num_b * prize.b_value.y;
                if x_val == prize.goal.x && y_val == prize.goal.y {
                    let this_cost = num_a * 3 + num_b;
                    if let Some(best_cost) = &mut best_cost {
                        *best_cost = std::cmp::min(*best_cost, this_cost);
                    } else {
                        best_cost = Some(this_cost);
                    }
                }
            }
        }

        total_cost += best_cost.unwrap_or_default();
        // println!("{prize:?}, {best_cost:?}");
    }

    println!("total_cost = {total_cost}");

    Ok(())
}
