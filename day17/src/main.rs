#![feature(iterator_try_collect)]

use anyhow::{anyhow, bail, Error, Ok};
use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr as _;

#[derive(Debug)]
struct Interpreter {
    a: usize,
    b: usize,
    c: usize,
    program: Vec<usize>,
    ip: usize,
    output: Vec<usize>,
}

#[derive(Debug)]
enum Instr {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instr {
    fn from_opcode(op: usize) -> Result<Self, Error> {
        match op {
            0 => Ok(Self::Adv),
            1 => Ok(Self::Bxl),
            2 => Ok(Self::Bst),
            3 => Ok(Self::Jnz),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out),
            6 => Ok(Self::Bdv),
            7 => Ok(Self::Cdv),
            _ => Err(anyhow!("bad opcode")),
        }
    }
}

impl Interpreter {
    fn resolve_operand(&self, operand: usize) -> Result<usize, Error> {
        match operand {
            0 | 1 | 2 | 3 => Ok(operand),
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            _ => {
                bail!("invalid combo operand");
            }
        }
    }

    fn do_div(&self, operand: usize) -> Result<usize, Error> {
        let numer = self.a;

        let operand = self.resolve_operand(operand)?;
        let denom = 1 << operand;
        println!("numer = {numer}, denom = {denom} operand = {operand}");
        Ok(numer / denom)
    }

    // returns false if the program has terminated
    fn step(&mut self) -> Result<bool, Error> {
        let Some((instr, operand)) = self.get_val() else {
            return Ok(false);
        };

        let instr = Instr::from_opcode(instr)?;

        let mut skip_incr = false;
        println!("exec {instr:?}, {operand}");
        match instr {
            Instr::Adv => {
                println!("a was {}", self.a);
                self.a = self.do_div(operand)?;
                println!("a is now {}", self.a);
            }
            Instr::Bxl => {
                self.b = self.b ^ operand;
                println!("b is now {}", self.b);
            }
            Instr::Bst => {
                let operand = self.resolve_operand(operand)?;
                self.b = operand % 8;
                println!("b is now {}", self.b);
            }
            Instr::Jnz => {
                if self.a != 0 {
                    self.ip = operand;
                    skip_incr = true;
                    println!("ip is now {}", self.ip);
                }
            }
            Instr::Bxc => {
                self.b = self.b ^ self.c;
                println!("b is now {}", self.b);
            }
            Instr::Out => {
                let operand = self.resolve_operand(operand)?;
                self.output.push(operand % 8);
                println!("output was {}", self.output.last().unwrap());
            }
            Instr::Bdv => {
                self.b = self.do_div(operand)?;
            }
            Instr::Cdv => {
                self.c = self.do_div(operand)?;
            }
        }

        if !skip_incr {
            self.ip += 2;
        }

        Ok(true)
    }

    fn get_val(&mut self) -> Option<(usize, usize)> {
        let instr = self.program.get(self.ip)?;
        let operand = self.program.get(self.ip + 1)?;
        Some((*instr, *operand))
    }
}

fn read_input(rd: impl BufRead) -> Result<Interpreter, Error> {
    let mut data: HashMap<String, String> = HashMap::new();

    for ln in rd.lines() {
        let ln = ln?;
        if ln.is_empty() {
            continue;
        }

        let (k, v) = ln.split_once(':').unwrap();
        let k = k.trim();
        let v = v.trim();
        let prev = data.insert(k.to_owned(), v.to_owned());
        assert!(prev.is_none());
    }

    let a = usize::from_str(data.get("Register A").unwrap())?;
    let b = usize::from_str(data.get("Register B").unwrap())?;
    let c = usize::from_str(data.get("Register C").unwrap())?;
    let program = data.get("Program").unwrap();
    let program = program.split(',').map(usize::from_str).try_collect()?;

    Ok(Interpreter {
        a,
        b,
        c,
        program,
        ip: 0,
        output: Vec::new(),
    })
}

fn test_1() -> Result<(), Error> {
    let mut i = Interpreter {
        a: 0,
        b: 0,
        c: 9,
        program: vec![2, 6],
        ip: 0,
        output: Vec::new(),
    };

    while i.step()? {}
    assert_eq!(i.b, 1);
    Ok(())
}

fn test_2() -> Result<(), Error> {
    let mut i = Interpreter {
        a: 10,
        b: 0,
        c: 0,
        program: vec![5, 0, 5, 1, 5, 4],
        ip: 0,
        output: Vec::new(),
    };

    while i.step()? {}
    assert_eq!(i.output, &[0, 1, 2]);

    Ok(())
}

fn test_3() -> Result<(), Error> {
    let mut i = Interpreter {
        a: 2024,
        b: 0,
        c: 0,
        program: vec![0, 1, 5, 4, 3, 0],
        ip: 0,
        output: Vec::new(),
    };

    while i.step()? {}
    assert_eq!(i.output, &[4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    assert_eq!(i.a, 0);

    Ok(())
}

fn test_4() -> Result<(), Error> {
    let mut i = Interpreter {
        a: 0,
        b: 29,
        c: 0,
        program: vec![1, 7],
        ip: 0,
        output: Vec::new(),
    };

    while i.step()? {}
    assert_eq!(i.b, 26);

    Ok(())
}

fn test_5() -> Result<(), Error> {
    let mut i = Interpreter {
        a: 0,
        b: 2024,
        c: 43690,
        program: vec![4, 0],
        ip: 0,
        output: Vec::new(),
    };

    while i.step()? {}
    assert_eq!(i.b, 44354);

    Ok(())
}

fn main() -> Result<(), Error> {
    test_1()?;
    test_2()?;
    test_3()?;
    test_4()?;
    test_5()?;

    let mut interp = read_input(std::io::stdin().lock())?;
    println!("before = {interp:?}");

    while interp.step()? {}

    println!("after = {interp:?}");

    let out: Vec<String> = interp.output.iter().map(usize::to_string).collect();
    let out = out.as_slice().join(",");
    println!("{out}");

    Ok(())
}
