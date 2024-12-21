#![feature(new_range_api)]
use anyhow::Error;
use rand::Rng;

// const FINAL_I: usize = 0b11101101001100011000101001101111011001100010;
// const SAMPLE_I: usize = 0b111001101011001010101111110;

// known output from part a given 21539243
const PART_1_ANSWER_VEC: &[usize] = &[6, 7, 5, 2, 1, 3, 5, 1, 7];

const PART_2_TARGET_VEC: &[usize] = &[2, 4, 1, 3, 7, 5, 1, 5, 0, 3, 4, 1, 5, 5, 3, 0];

fn vec_to_int(prog: &[usize]) -> usize {
    let mut ret = 0;

    for val in prog.iter() {
        ret = ret * 8 + val;
    }
    ret
}

fn score(actual: usize, expected: usize) -> usize {
    let mut dist = 0;

    for actual_idx in 0..64 {
        for expected_idx in actual_idx + 1..64 {
            let actual_val = (actual >> actual_idx) & 1 != 0;
            let expected_val = (expected >> expected_idx) & 1 != 0;
            if actual_val == expected_val {
                continue;
            }

            // println!("diff at {actual_idx}, {expected_idx}");
            // dist += 1 << (std::cmp::min(expected_idx - actual_idx, 8));
            dist += 1;
        }
    }

    // println!("score is {dist}");

    dist
}

fn eval(mut in_val: usize, with_print: bool) -> (usize, usize) {
    let mut actual = 0;
    let mut len = 0;
    loop {
        len += 1;
        // println!("in_val = {in_val:048b}, out_val={out_val:048b}");
        // calculate p for this iter
        let shift_val = (in_val ^ 0b011) & 0b111;
        let p = in_val ^ (in_val >> shift_val);
        let p = p ^ 0b110;

        if with_print {
            println!("blat, {}", p & 0b111);
        }

        actual = actual << 3 | (p & 0b111);

        in_val >>= 3;

        if in_val == 0 {
            break;
        }
    }

    (actual, len)

    // println!("in_val = {in_val}, expected = {expected}, actual = {actual}");
}

enum DoOp {
    Add,
    Sub,
    None,
}

impl DoOp {
    fn do_it(&self, guess: usize) -> Option<usize> {
        match self {
            DoOp::Add => guess.checked_add(1),
            DoOp::Sub => guess.checked_sub(1),
            DoOp::None => Some(guess),
        }
    }
}

fn main() -> Result<(), Error> {
    let part_1_answer_i = vec_to_int(PART_1_ANSWER_VEC);
    let (part_1_i_eval, _) = eval(21539243, true);
    let part_2_target_i = vec_to_int(PART_2_TARGET_VEC);
    assert_eq!(part_1_answer_i, part_1_i_eval);

    let mut rng = rand::thread_rng();

    let mut guess: usize = 0;
    let mut variations: Vec<usize> = Vec::new();

    loop {
        variations.clear();
        // push current best guess
        variations.push(guess);

        for not_val in [usize::MIN, usize::MAX] {
            let guess_1 = guess ^ not_val;
            // try variations of shifting
            for shift_out_val in 0..8 {
                let guess_2 = guess_1 >> shift_out_val;

                for shift_in_val in 0..8 {
                    let guess_3 = guess_2 << shift_in_val;
                    // try flipping bits
                    for _i in 0..64 {
                        let bit_index: i32 = rng.gen_range(0..64);
                        let bit_val: bool = rng.gen();
                        let bit_val = usize::from(bit_val);
                        let guess_4 = guess_3 ^ (bit_val << bit_index);
                        // try adding and subtracting
                        for op in &[DoOp::Add, DoOp::Sub, DoOp::None] {
                            let Some(guess_5) = op.do_it(guess_4) else {
                                continue;
                            };

                            // try xoring
                            for val in 0..8 {
                                let guess_6 = guess_5 ^ val;
                                variations.push(guess_6);

                                // try anding
                                for val in 0..8 {
                                    let guess_7 = guess_6 & val;
                                    variations.push(guess_7);
                                }
                            }
                        }
                    }
                }
            }
        }

        // println!("len {}", variations.len());

        guess = variations
            .iter()
            .copied()
            .min_by_key(|a_val| {
                let (actual, len) = eval(*a_val, false);

                if len != 16 {
                    return usize::MAX;
                }
                let expected = part_2_target_i;
                score(actual, expected)
            })
            .unwrap();

        let (guess_i, _len) = eval(guess, true);

        let guess_score = score(guess_i, part_2_target_i);

        println!("guess is {guess}, guess score is = {guess_score}");
    }
}
