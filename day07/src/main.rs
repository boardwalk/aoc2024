#![feature(iterator_try_collect)]
use anyhow::{bail, Error};
use indicatif::ProgressIterator as _;
use num_bigint::BigUint;
use rayon::prelude::*;
use std::str::FromStr as _;

struct Equation {
    test_value: BigUint,
    args: Vec<BigUint>,
}

fn do_append_slow(res: BigUint, arg_val: &BigUint) -> BigUint {
    let mut res_tmp = res.to_string();
    let arg_tmp = arg_val.to_string();
    res_tmp.push_str(&arg_tmp);

    BigUint::from_str(&res_tmp).unwrap()
}

fn eval_equation(eqn: &Equation, mut permute_num: usize) -> Option<BigUint> {
    let mut res = eqn.args[0].clone();

    for arg_val in eqn.args.iter().skip(1) {
        let op_num = permute_num & 0b11;
        permute_num >>= 2;

        match op_num {
            0b00 => {
                res = res + arg_val;
            }
            0b01 => {
                res = res * arg_val;
            }
            0b10 => {
                res = do_append_slow(res, arg_val);
            }
            0b11 => {
                return None;
            }
            _ => {
                panic!("ack!");
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
    let mut eqns = Vec::new();

    for ln in std::io::stdin().lines() {
        let ln = ln?;
        let tokens: Vec<_> = ln.split(':').collect();

        let [test_value, args] = &tokens.as_slice() else {
            bail!("wrong number of tokens");
        };

        let test_value = BigUint::from_str(test_value)?;

        let args: Vec<_> = args
            .split_ascii_whitespace()
            .map(BigUint::from_str)
            .try_collect()?;

        eqns.push(Equation {
            test_value: test_value,
            args,
        });
    }

    let mut sum = BigUint::ZERO;

    for eqn in eqns.iter().progress() {
        if equation_satisfiable(eqn) {
            sum += &eqn.test_value;
        }
    }

    println!("{sum}");

    Ok(())
}
