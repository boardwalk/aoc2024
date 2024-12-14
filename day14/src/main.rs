use anyhow::{anyhow, Error};
use std::str::FromStr as _;

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Robot {
    pos: Vec2,
    vel: Vec2,
}

// const WIDTH: i64 = 11;
// const HEIGHT: i64 = 7;
const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

fn read_robots() -> Result<Vec<Robot>, Error> {
    let re_bot = regex::Regex::new(r"^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$")?;

    let mut robots = Vec::new();

    for ln in std::io::stdin().lines() {
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

fn main() -> Result<(), Error> {
    let mut robots = read_robots()?;

    let mut num_bots: [usize; 4] = [0; 4];

    for robot in &mut robots {
        for _sec in 0..100 {
            let mut final_x = robot.pos.x;
            let mut final_y = robot.pos.y;

            final_x += robot.vel.x;
            final_y += robot.vel.y;

            while final_x >= WIDTH {
                final_x -= WIDTH;
            }

            while final_y >= HEIGHT {
                final_y -= HEIGHT;
            }

            while final_x < 0 {
                final_x += WIDTH;
            }

            while final_y < 0 {
                final_y += HEIGHT;
            }

            let pos = Vec2 {
                x: final_x,
                y: final_y,
            };

            robot.pos = pos;
        }

        if let Some(quadrant) = which_quadrant(robot.pos) {
            println!("bot goes in {}", quadrant);
            num_bots[quadrant] += 1;
        } else {
            println!("bot goes nowhere");
        }
    }

    let safety_factor = num_bots[0] * num_bots[1] * num_bots[2] * num_bots[3];

    println!("{safety_factor}");
    println!("{num_bots:?}");

    // println!("{robots:#?}");
    Ok(())
}
