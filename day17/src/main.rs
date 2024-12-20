#![feature(new_range_api)]
use anyhow::Error;
use std::collections::HashMap;

const OUTPUT: &[usize] = &[2, 4, 1, 3, 7, 5, 1, 5, 0, 3, 4, 1, 5, 5, 3, 0];
const CYCLES_PER_LOOP: usize = 8;
const CYCLES_PER_INVOKE: usize = CYCLES_PER_LOOP * OUTPUT.len();

const BIT_INDICES: std::ops::Range<usize> = 0usize..64usize;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Register {
    A,
    B,
    C,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Step(usize);

impl Register {
    fn get_bit_idx(self, b: usize) -> usize {
        assert!(BIT_INDICES.contains(&b));
        match self {
            Register::A => b,
            Register::B => b + 64,
            Register::C => b + 64 + 64,
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct BitKey {
    step: Step,
    reg_bit_idx: usize,
}

impl BitKey {
    fn get(step: Step, r: Register, b: usize) -> Self {
        Self {
            step,
            reg_bit_idx: r.get_bit_idx(b),
        }
    }
}

struct State {
    ip: usize,
    num_unprints: usize,
    step: Step,
    // step to bit to value
    a: Vec<Vec<Option<bool>>>,
    b: Vec<Vec<Option<bool>>>,
    c: Vec<Vec<Option<bool>>>,
    did_enter_init: bool,
}

impl State {
    fn new() -> Self {
        let ip = 7;

        // a is 0 on exit
        let a = vec![vec![Some(false)]];

        let b = vec![];
        let c = vec![];

        Self {
            ip,
            num_unprints: 0,
            step: Step(0),
            did_enter_init: false,
            a,
            b,
            c,
        }
    }

    fn step_back(&mut self) -> bool {
        println!("on step {}", self.step.0);
        let before_step = self.step;
        let after_step = Step(self.step.0 + 1);

        while self.a.len() < before_step.0 {
            self.a.push(Vec::new());
        }

        while self.b.len() < before_step.0 {
            self.b.push(Vec::new());
        }

        while self.c.len() < before_step.0 {
            self.c.push(Vec::new());
        }

        while self.a.len() < after_step.0 {
            self.a.push(Vec::new());
        }

        while self.b.len() < after_step.0 {
            self.b.push(Vec::new());
        }

        while self.c.len() < after_step.0 {
            self.c.push(Vec::new());
        }

        let a_1 = &self.a[before_step.0];
        let b_1 = &self.b[before_step.0];
        let c_1 = &self.c[before_step.0];

        let a_2 = &mut self.a[after_step.0];
        let b_2 = &mut self.b[after_step.0];
        let c_2 = &mut self.c[after_step.0];

        *a_1 = a_2.clone();
        *b_1 = b_2.clone();
        *c_1 = c_2.clone();

        let mut a = self.a.get_mut(before_step.0);
        match self.ip {
            0 => {
                // b = a & 7
                // undo the write to b
            }
            1 => {
                // b ^= 3
                // flip bits 0 and 1 of b
            }
            2 => {
                // c >>= b
                // need to shift left and clear low bits
            }
            3 => {
                // b ^= 5
                // flip bits 0 and 2 of b
            }
            4 => {
                // a >>= 3
                // to undo shifting low bits out, shift high bits in
            }
            5 => {
                // b ^= c
            }
            6 => {
                // out a & 8
                self.num_unprints += 1;
                let val = OUTPUT[OUTPUT.len() - self.num_unprints];
            }
            7 => {
                //
            }
            _ => {
                panic!("bad ip");
            }
        }

        if self.ip > 0 {
            self.ip -= 1;
        } else {
            self.ip = 7;
        }

        if self.num_unprints == 16 && self.ip == 0 {
            // b is 0 on entrance
            if !self.did_enter_init {
                for i in BIT_INDICES {
                    let k = BitKey::get(after_step, Register::B, i);
                    let v = false;
                    self.history.insert(k, v);
                }

                // c is 0 on entrance
                for i in BIT_INDICES {
                    let k = BitKey::get(after_step, Register::C, i);
                    let v = false;
                    self.history.insert(k, v);
                }

                self.did_enter_init = true;
            }

            self.num_unprints = 0;
            self.ip = 7;
            self.step = Step(0);
            false
        } else {
            self.step = after_step;
            true
        }
    }

    fn reset(&mut self) {
        if !self.did_enter_init {
            for i in BIT_INDICES {
                let k = BitKey::get(Step(CYCLES_PER_INVOKE), Register::B, i);
                let v = false;
                self.history.insert(k, v);
            }

            // c is 0 on entrance
            for i in BIT_INDICES {
                let k = BitKey::get(Step(CYCLES_PER_INVOKE), Register::C, i);
                let v = false;
                self.history.insert(k, v);
            }

            self.did_enter_init = true;
        }

        self.num_unprints = 0;
        self.ip = 7;
        self.step = Step(0);
    }
}

fn main() -> Result<(), Error> {
    let mut st = State::new();

    loop {
        while st.step_back() {}
        st.reset();
        println!(
            "reset, hist size is {}, val len size is {}",
            st.history.len(),
            st.val_len.len()
        );
    }
}
