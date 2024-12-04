use anyhow::Error;
use std::io::Read;

const PART_TWO: bool = true;

struct ParseResult<'a, T> {
    rest: &'a [char],
    val: T,
}

struct Mul {
    lhs: usize,
    rhs: usize,
}

fn expect<'a>(rest: &'a [char], prefix: &[char]) -> Option<ParseResult<'a, ()>> {
    if rest.starts_with(prefix) {
        Some(ParseResult {
            rest: &rest[prefix.len()..],
            val: (),
        })
    } else {
        None
    }
}

fn parse_num<'a>(mut rest: &'a [char]) -> Option<ParseResult<'a, usize>> {
    let mut val: usize = 0;

    let mut num_digits = 0;

    while !rest.is_empty() {
        if let Some(digit) = rest[0].to_digit(10) {
            rest = &rest[1..];
            val = val * 10 + digit as usize;
            num_digits += 1;
            // if hit max digits, all done
            if num_digits == 3 {
                return Some(ParseResult { rest, val });
            }
        } else {
            // no more digits, but have at least one, done
            if num_digits >= 1 {
                return Some(ParseResult { rest, val });
            } else {
                return None;
            }
        }
    }

    None
}

fn parse_mul<'a>(mut rest: &'a [char]) -> Option<ParseResult<'a, Mul>> {
    let parse_res = expect(rest, &['m', 'u', 'l', '('])?;
    rest = parse_res.rest;

    let parse_res = parse_num(rest)?;
    rest = parse_res.rest;
    let lhs = parse_res.val;

    let parse_res = expect(rest, &[','])?;
    rest = parse_res.rest;

    let parse_res = parse_num(rest)?;
    rest = parse_res.rest;
    let rhs = parse_res.val;

    let parse_res = expect(rest, &[')'])?;
    rest = parse_res.rest;

    Some(ParseResult {
        rest,
        val: Mul { lhs, rhs },
    })
}

fn parse_do<'a>(mut rest: &'a [char]) -> Option<ParseResult<'a, ()>> {
    let parse_res = expect(rest, &['d', 'o', '(', ')'])?;
    rest = parse_res.rest;

    Some(ParseResult { rest, val: () })
}

fn parse_dont<'a>(mut rest: &'a [char]) -> Option<ParseResult<'a, ()>> {
    let parse_res = expect(rest, &['d', 'o', 'n', '\'', 't', '(', ')'])?;
    rest = parse_res.rest;

    Some(ParseResult { rest, val: () })
}

fn main() -> Result<(), Error> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    let buf: Vec<char> = buf.chars().collect();
    let mut rest = buf.as_slice();
    let mut sum = 0;

    let mut mul_enabled = true;

    loop {
        if rest.is_empty() {
            break;
        }

        if PART_TWO {
            if let Some(parse_res) = parse_do(rest) {
                rest = parse_res.rest;
                mul_enabled = true;
                continue;
            }

            if let Some(parse_res) = parse_dont(rest) {
                rest = parse_res.rest;
                mul_enabled = false;
                continue;
            }
        }

        if mul_enabled {
            if let Some(parse_res) = parse_mul(rest) {
                sum += parse_res.val.lhs * parse_res.val.rhs;
                rest = parse_res.rest;
                continue;
            }
        }

        rest = &rest[1..];
    }

    println!("{sum}");

    Ok(())
}
