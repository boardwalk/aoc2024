#![feature(iterator_try_collect)]
#![feature(coroutines)]
#![feature(coroutine_trait)]

const PART_TWO: bool = true;

use anyhow::Error;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::ops::Coroutine;
use std::pin::Pin;
use std::str::FromStr as _;

fn mix(secret_num: usize, value: usize) -> usize {
    // To mix a value into the secret number, calculate the bitwise XOR of the given value and the secret number. Then, the secret number becomes the result of that operation. (If the secret number is 42 and you were to mix 15 into the secret number, the secret number would become 37.)
    secret_num ^ value
}

fn prune(secret_num: usize) -> usize {
    // To prune the secret number, calculate the value of the secret number modulo 16777216. Then, the secret number becomes the result of that operation. (If the secret number is 100000000 and you were to prune the secret number, the secret number would become 16113920.)
    secret_num % 16777216
}

fn next_secret_num(mut secret_num: usize) -> usize {
    // Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
    secret_num = prune(mix(secret_num, secret_num * 64));
    // Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
    secret_num = mix(secret_num, secret_num / 32);
    secret_num = prune(secret_num);
    // Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
    secret_num = mix(secret_num, secret_num * 2048);
    secret_num = prune(secret_num);
    secret_num
}

fn gen_many_vec(mut secret_num: usize, count: usize) -> Vec<usize> {
    let mut res = Vec::with_capacity(count);

    for _i in 0..count {
        secret_num = next_secret_num(secret_num);
        res.push(secret_num);
    }

    res
}

struct IterCoro<T> {
    coro: Option<T>,
}

impl<T> Iterator for IterCoro<T>
where
    T: Coroutine + Unpin,
{
    type Item = T::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        let coro = self.coro.as_mut()?;

        match Pin::new(coro).resume(()) {
            std::ops::CoroutineState::Yielded(val) => Some(val),
            std::ops::CoroutineState::Complete(_) => {
                self.coro = None;
                None
            }
        }
    }
}

fn iter_coro<T>(coro: T) -> IterCoro<T>
where
    T: Coroutine,
{
    IterCoro { coro: Some(coro) }
}

fn iter_change_sequences() -> impl Iterator<Item = [i64; 4]> {
    // some of these aren't possible, but that's fine, evaling them is just a waste of time
    iter_coro(
        #[coroutine]
        || {
            for c1 in -9..=9 {
                for c2 in -9..=9 {
                    for c3 in -9..=9 {
                        for c4 in -9..=9 {
                            yield [c1, c2, c3, c4];
                        }
                    }
                }
            }
        },
    )
}

fn eval_change_sequence(expected_seq: [i64; 4], mut secret_num: usize) -> Option<i64> {
    let mut actual_seq: [i64; 4] = [0, 0, 0, 0];
    let mut begin_i = 0;
    let mut end_i = 0;

    for _i in 0..2000 {
        let prev_price = i64::try_from(secret_num % 10).unwrap();
        secret_num = next_secret_num(secret_num);
        let this_price = i64::try_from(secret_num % 10).unwrap();
        let price_delta = this_price - prev_price;

        // pop delta
        if end_i - begin_i >= 4 {
            begin_i += 1;
        }

        // push_delta
        actual_seq[end_i % 4] = price_delta;
        end_i += 1;

        // println!("i = {i} secret_num = {secret_num}, prev_price = {prev_price}, this_price = {this_price}, actual_seq = {actual_seq:?}");

        // compare if we have 4 things to compare
        if end_i - begin_i >= 4 {
            let mut matched = true;
            for j in 0..4 {
                let actual_price = actual_seq[(begin_i + j) % 4];
                let expected_price = expected_seq[j];
                if actual_price != expected_price {
                    matched = false;
                    break;
                }
            }

            if matched {
                return Some(this_price);
            }
            //todo
        }
    }

    None
}

fn eval_change_sequence_all(expected_seq: [i64; 4], buyers: &[usize]) -> i64 {
    let mut tot = 0;

    for buyer in buyers {
        tot += eval_change_sequence(expected_seq, *buyer).unwrap_or_default();
    }

    tot
}

fn main() -> Result<(), Error> {
    assert_eq!(prune(100000000), 16113920);
    assert_eq!(mix(42, 15), 37);
    assert_eq!(
        gen_many_vec(123, 10),
        &[
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254
        ]
    );

    assert_eq!(eval_change_sequence([-2, 1, -1, 3], 1), Some(7));

    assert_eq!(eval_change_sequence([-2, 1, -1, 3], 2), Some(7));

    assert_eq!(eval_change_sequence([-2, 1, -1, 3], 3), None);

    assert_eq!(eval_change_sequence([-2, 1, -1, 3], 2024), Some(9));
    assert_eq!(
        eval_change_sequence_all([-2, 1, -1, 3], &[1, 2, 3, 2024]),
        23
    );

    let buyers: Vec<_> = std::io::stdin().lines().try_collect()?;
    let buyers: Vec<_> = buyers.iter().map(|ln| usize::from_str(ln)).try_collect()?;

    if PART_TWO {
        let seauences: Vec<_> = iter_change_sequences().collect();
        let val = seauences
            .into_par_iter()
            .map(|seq| eval_change_sequence_all(seq, &buyers))
            .progress()
            .max();
        println!("{val:?}");
    } else {
        let mut sum = 0;

        for buyer in &buyers {
            let mut secret_num = *buyer;

            for _i in 0..2000 {
                secret_num = next_secret_num(secret_num);
            }

            sum += secret_num;
        }

        println!("{sum}");
    }

    Ok(())
}
