#![feature(iterator_try_collect)]
use anyhow::{bail, Error};
use std::fmt::Write as _;
use std::str::FromStr as _;

struct Equation {
    test_value: usize,
    args: Vec<usize>,
}

fn do_add(lhs: usize, rhs: usize) -> usize {
    lhs.checked_add(rhs).unwrap()
}

fn do_mul(lhs: usize, rhs: usize) -> usize {
    lhs.checked_mul(rhs).unwrap()
}

fn do_append(lhs: usize, rhs: usize) -> usize {
    let mut res = lhs.to_string();
    write!(&mut res, "{}", rhs).unwrap();
    usize::from_str(&res).unwrap()
}

fn eval_equation(eqn: &Equation, cur_sum: usize, next_arg: usize) -> bool {
    if next_arg >= eqn.args.len() {
        // no more args to process, check sum
        return cur_sum == eqn.test_value;
    }

    eval_equation(eqn, do_add(cur_sum, eqn.args[next_arg]), next_arg + 1)
        || eval_equation(eqn, do_mul(cur_sum, eqn.args[next_arg]), next_arg + 1)
        || eval_equation(eqn, do_append(cur_sum, eqn.args[next_arg]), next_arg + 1)
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

        let test_value = usize::from_str(test_value)?;

        let args: Vec<_> = args
            .split_ascii_whitespace()
            .map(usize::from_str)
            .try_collect()?;

        eqns.push(Equation {
            test_value: test_value,
            args,
        });
    }

    let eqns: Vec<_> = eqns
        .into_iter()
        .filter(|eqn| equation_satisfiable(eqn))
        .collect();

    let mut sum = 0usize;

    for eq in &eqns {
        sum = sum.checked_add(eq.test_value).unwrap();
    }

    println!("sum: {}", sum);

    Ok(())
}
