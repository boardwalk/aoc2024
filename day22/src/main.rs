#![feature(iterator_try_collect)]

use anyhow::Error;
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

    let buyers: Vec<_> = std::io::stdin().lines().try_collect()?;
    let buyers: Vec<_> = buyers.iter().map(|ln| usize::from_str(ln)).try_collect()?;

    let mut sum = 0;

    for buyer in &buyers {
        let mut secret_num = *buyer;

        for _i in 0..2000 {
            secret_num = next_secret_num(secret_num);
        }

        sum += secret_num;
    }

    println!("{sum}");

    Ok(())
}
