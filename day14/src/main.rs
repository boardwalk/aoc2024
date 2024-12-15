use anyhow::{anyhow, Error};
use ndarray::Array2;
use std::io::BufRead;
use std::str::FromStr as _;

const PART_TWO: bool = false;

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: Vec2,
    vel: Vec2,
}

// const WIDTH: i64 = 11;
// const HEIGHT: i64 = 7;
const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

fn read_robots(rd: impl BufRead) -> Result<Vec<Robot>, Error> {
    let re_bot = regex::Regex::new(r"^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$")?;

    let mut robots = Vec::new();

    for ln in rd.lines() {
        let ln = ln?;

        let bot = re_bot
            .captures(&ln)
            .ok_or_else(|| anyhow!("failed to parse bot"))?;

        let px = i64::from_str(&bot[1])?;
        let py = i64::from_str(&bot[2])?;
        let vx = i64::from_str(&bot[3])?;
        let vy = i64::from_str(&bot[4])?;

        robots.push(Robot {
            pos: Vec2 { x: px, y: py },
            vel: Vec2 { x: vx, y: vy },
        });
    }

    Ok(robots)
}

fn which_quadrant(pos: Vec2) -> Option<usize> {
    let half_x = (WIDTH - 1) / 2;
    let half_y = (HEIGHT - 1) / 2;

    if pos.x < half_x && pos.y < half_y {
        Some(0)
    } else if pos.x > half_x && pos.y > half_y {
        Some(1)
    } else if pos.x > half_x && pos.y < half_y {
        Some(2)
    } else if pos.x < half_x && pos.y > half_y {
        Some(3)
    } else {
        None
    }
}

// fn gcd(a: usize, b: usize) -> usize {
//     if b != 0 {
//         gcd(b, a % b)
//     } else {
//         a
//     }
// }

// fn period(k: usize, m: usize) -> usize {
//     m / gcd(k, m)
// }

fn print_bots(robots: &[Robot]) {
    let mut arr: Array2<bool> = Array2::default((WIDTH as usize, HEIGHT as usize));

    for robot in robots {
        arr[(robot.pos.x as usize, robot.pos.y as usize)] = true;
    }

    let mut out = String::new();

    for y in 0..HEIGHT as usize {
        for x in 0..WIDTH as usize {
            if arr[(x, y)] {
                out.push('#')
            } else {
                out.push(' ');
            }
        }
        out.push('\n');
    }
    println!("{out}");
}

fn score_bots(robots: &[Robot]) -> usize {
    let mut num_bots: [usize; 4] = [0; 4];

    for robot in robots {
        if let Some(quadrant) = which_quadrant(robot.pos) {
            num_bots[quadrant] += 1;
        }
    }

    num_bots[0] * num_bots[1] * num_bots[2] * num_bots[3]
}

fn step_all_bots(robots: &mut [Robot], num_steps: usize) {
    for robot in robots {
        step_bot(robot, num_steps);
    }
}

fn step_bot(robot: &mut Robot, num_steps: usize) {
    let num_steps = i64::try_from(num_steps).unwrap();
    robot.pos.x = (robot.pos.x + robot.vel.x * num_steps).rem_euclid(WIDTH);
    robot.pos.y = (robot.pos.y + robot.vel.y * num_steps).rem_euclid(HEIGHT);
}

fn sqr_dist(a: &Vec2, b: &Vec2) -> i64 {
    let x_diff = a.x - b.x;
    let y_diff = a.y - b.y;

    x_diff * x_diff + y_diff * y_diff
}

fn score_bot_dist(robots: &[Robot]) -> i64 {
    let mut total_dist = 0;
    for i in 0..robots.len() {
        for j in i + 1..robots.len() {
            total_dist += sqr_dist(&robots[i].pos, &robots[j].pos);
        }
    }

    total_dist
}

fn main() -> Result<(), Error> {
    let robots = read_robots(std::io::BufReader::new(std::fs::File::open("input02.txt")?))?;

    if PART_TWO {
        let mut bots_copy = robots.clone();
        let mut best_score: Option<(usize, i64)> = None;
        for this_step in 0..10403 {
            let this_score = score_bot_dist(&bots_copy);
            if let Some((best_step, best_score)) = &mut best_score {
                if this_score < *best_score {
                    *best_step = this_step;
                    *best_score = this_score;
                }
            } else {
                best_score = Some((this_step, this_score));
            }
            step_all_bots(&mut bots_copy, 1);
        }

        let (step, _score) = best_score.unwrap();

        let mut bots_copy = robots.clone();
        step_all_bots(&mut bots_copy, step);
        print_bots(&bots_copy);

        println!("best_score = {best_score:?}");
    } else {
        let mut bots_copy = robots.clone();

        step_all_bots(&mut bots_copy, 100);
        let score = score_bots(&bots_copy);
        println!("{score}");
    }

    Ok(())
}
