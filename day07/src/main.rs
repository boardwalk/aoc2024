#![feature(iterator_try_collect)]
use anyhow::{bail, Error};
use indicatif::ProgressIterator as _;
use rayon::prelude::*;

#[derive(Debug)]
struct Equation {
    test_value: usize,
    args: Vec<usize>,
}

fn do_append_fast(mut res: usize, mut arg_val: usize) -> usize {
    let mut digits = Vec::new();

    while arg_val != 0 {
        let digit = arg_val % 10;
        arg_val /= 10;

        digits.push(digit);
    }

    for digit in digits.iter().rev() {
        res = res * 10 + *digit;
    }

    res
}

fn do_append_slow(res: usize, arg_val: usize) -> usize {
    let mut res_tmp = res.to_string();
    let arg_tmp = arg_val.to_string();
    res_tmp.push_str(&arg_tmp);
    usize::from_str_radix(&res_tmp, 10).unwrap()
}

fn eval_equation(eqn: &Equation, mut permute_num: usize) -> Option<usize> {
    let mut res = eqn.args[0];

    for arg_val in eqn.args.iter().skip(1) {
        let op_num = permute_num & 3;
        permute_num >>= 2;

        match op_num {
            0 => {
                res = res + *arg_val;
            }
            1 => {
                res = res * *arg_val;
            }
            2 => {
                // res = do_append_fast(res, *arg_val);
            }
            _ => {
                return None;
            }
        }
    }

    Some(res)
}

fn equation_satisfiable(eqn: &Equation) -> bool {
    let num_permutes = 1 << (2 * (eqn.args.len() - 1));

    (0..num_permutes)
        .into_par_iter()
        .find_any(|permute_num| {
            let Some(val) = eval_equation(eqn, *permute_num) else {
                return false;
            };

            val == eqn.test_value
        })
        .is_some()
}

fn main() -> Result<(), Error> {
    assert_eq!(do_append_slow(12, 345), 12345);
    assert_eq!(do_append_fast(12, 345), 12345);
    let mut eqns = Vec::new();

    for ln in std::io::stdin().lines() {
        let ln = ln?;
        let tokens: Vec<_> = ln.split(':').collect();

        let [test_value, args] = &tokens.as_slice() else {
            bail!("wrong number of tokens");
        };

        let test_value = usize::from_str_radix(test_value, 10)?;

        let args: Vec<_> = args
            .split_ascii_whitespace()
            .map(|arg| usize::from_str_radix(arg, 10))
            .try_collect()?;

        eqns.push(Equation { test_value, args });
    }

    println!("{eqns:?}");

    let mut sum = 0;

    for eqn in eqns.iter().progress() {
        // println!("testing eqn {eqn:?}");
        if equation_satisfiable(eqn) {
            sum += eqn.test_value;
        }
    }

    println!("{sum}");

    Ok(())
}
