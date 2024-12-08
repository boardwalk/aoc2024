#![feature(iterator_try_collect)]
use anyhow::{bail, Error};
use indicatif::ParallelProgressIterator as _;
use num_bigint::BigUint;
use rayon::prelude::*;
use std::fmt::Write as _;
use std::str::FromStr as _;

struct Equation {
    test_value: BigUint,
    args: Vec<BigUint>,
}

fn do_add(lhs: &BigUint, rhs: &BigUint) -> BigUint {
    lhs + rhs
}

fn do_mul(lhs: &BigUint, rhs: &BigUint) -> BigUint {
    lhs * rhs
}

fn do_append(lhs: &BigUint, rhs: &BigUint) -> BigUint {
    let mut res = lhs.to_string();
    write!(&mut res, "{}", rhs).unwrap();
    BigUint::from_str(&res).unwrap()
}

fn eval_equation(eqn: &Equation, cur_sum: BigUint, next_arg: usize) -> bool {
    if next_arg >= eqn.args.len() {
        // no more args to process, check sum
        return cur_sum == eqn.test_value;
    }

    eval_equation(eqn, do_add(&cur_sum, &eqn.args[next_arg]), next_arg + 1)
        || eval_equation(eqn, do_mul(&cur_sum, &eqn.args[next_arg]), next_arg + 1)
        || eval_equation(eqn, do_append(&cur_sum, &eqn.args[next_arg]), next_arg + 1)
}

fn equation_satisfiable(eqn: &Equation) -> bool {
    eval_equation(eqn, eqn.args[0].clone(), 1)
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

    let eqns: Vec<_> = eqns
        .into_par_iter()
        .progress()
        .filter(|eqn| equation_satisfiable(eqn))
        .collect();

    let mut sum = BigUint::ZERO;

    for eq in &eqns {
        sum = sum + &eq.test_value;
    }

    println!("sum: {}", sum);

    Ok(())
}
