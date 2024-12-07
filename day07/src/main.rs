#![feature(iterator_try_collect)]
use anyhow::{bail, Error};

#[derive(Debug)]
struct Equation {
    test_value: usize,
    args: Vec<usize>,
}

enum Op {
    Add,
    Mul,
}

fn get_op(arg_i: usize, permute_num: usize) -> Op {
    if (permute_num >> arg_i) % 2 == 0 {
        Op::Add
    } else {
        Op::Mul
    }
}

fn eval_equation(eqn: &Equation, permute_num: usize) -> usize {
    let mut res = 0;

    for (arg_i, op_val) in eqn.args.iter().enumerate() {
        match get_op(arg_i, permute_num) {
            Op::Add => {
                res += *op_val;
            }
            Op::Mul => {
                res *= *op_val;
            }
        }
    }

    res
}

fn equation_satisfiable(eqn: &Equation) -> bool {
    let num_operators = eqn.args.len();
    for permute_num in 0..(1 << num_operators) {
        if eval_equation(eqn, permute_num) == eqn.test_value {
            return true;
        }
    }

    false
}

fn main() -> Result<(), Error> {
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

    for eqn in &eqns {
        if equation_satisfiable(eqn) {
            sum += eqn.test_value;
        }
    }

    println!("{sum}");

    Ok(())
}
