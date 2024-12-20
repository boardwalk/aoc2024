use anyhow::Error;

const OUTPUT: &[usize] = &[2, 4, 1, 3, 7, 5, 1, 5, 0, 3, 4, 1, 5, 5, 3, 0];

#[derive(Debug, Clone, Copy, Default)]
struct BitState {
    flipped: Option<()>,
    value: Option<bool>,
}

fn flip_flip(f: &mut BitState) {
    if f.flipped.is_some() {
        f.flipped = None;
    } else {
        f.flipped = Some(());
    }
}

const ZERO: BitState = BitState {
    flipped: None,
    value: Some(false),
};

struct State {
    a: [BitState; 64],
    b: [BitState; 64],
    c: [BitState; 64],
    ip: usize,
    // number of things printed so far
    num_prints: usize,
}

fn make_entrance() -> State {
    // a is unknown to start
    let a = [BitState::default(); 64];
    // b is 0 to start
    let b = [ZERO; 64];
    // c is 0 to start
    let c = [ZERO; 64];
    let ip = 0;
    let num_prints = 0;

    State {
        a,
        b,
        c,
        ip,
        num_prints,
    }
}

fn make_exit() -> State {
    // a is 0 on exit
    let a = [ZERO; 64];

    // b is unknown on exit
    let b = [BitState::default(); 64];

    // c is unknown on exit
    let c = [BitState::default(); 64];

    let ip = 7;
    let num_prints = OUTPUT.len();
    State {
        a,
        b,
        c,
        ip,
        num_prints,
    }
}

fn merge_bits(lhs: BitState, rhs: BitState) -> BitState {
    todo!()
}
impl State {
    fn score(&self) -> usize {
        let mut score = 0;
        score += self.a.iter().filter(|bs| bs.value.is_some()).count();
        score += self.b.iter().filter(|bs| bs.value.is_some()).count();
        score += self.c.iter().filter(|bs| bs.value.is_some()).count();
        score
    }

    fn merge_from(&mut self, other: &Self) -> bool {
        if other.ip != self.ip || other.num_prints != other.num_prints {
            return false;
        }

        for (lhs, rhs) in self.a.iter_mut().zip(other.a.iter()) {
            *lhs = merge_bits(*lhs, *rhs);
        }

        for (lhs, rhs) in self.b.iter_mut().zip(other.b.iter()) {
            *lhs = merge_bits(*lhs, *rhs);
        }

        for (lhs, rhs) in self.c.iter_mut().zip(other.c.iter()) {
            *lhs = merge_bits(*lhs, *rhs);
        }

        true
    }

    fn step_back(&mut self) -> bool {
        if self.num_prints == 0 && self.ip == 0 {
            return false;
        }

        match self.ip {
            0 => {
                // b = a & 7
            }
            1 => {
                // b ^= 3
                flip_flip(&mut self.b[0]);
                flip_flip(&mut self.b[1]);
            }
            2 => {
                // c >>= b
                // need to shift left and clear low bits
            }
            3 => {
                // 3
                // b ^= 5
                flip_flip(&mut self.b[0]);
                flip_flip(&mut self.b[2]);
            }
            4 => {
                // 4
                // a >>= 3
                for i in 0..64 {
                    let src_idx = i + 3;
                    let dst_idx = i;

                    self.a[dst_idx] = self.a.get(src_idx).copied().unwrap_or_else(|| BitState {
                        flipped: None, // i think?
                        value: Some(false),
                    });
                }
            }
            5 => {
                // 5
                // b ^= c
                for (b, c) in self.b.iter_mut().zip(self.c.iter()) {
                    if c.flipped.is_some() {
                        flip_flip(b);
                    }
                }
            }
            6 => {
                let val = OUTPUT[self.num_prints - 1];
                // this gets us the low 3 bits of b
                for bnum in 0..3 {
                    let value = (val >> bnum & 1) != 0;
                    self.b[bnum] = BitState {
                        flipped: None,
                        value: Some(value),
                    };
                }

                self.num_prints -= 1;
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

        true
    }

    fn reset_to_exit(&mut self) {
        self.ip = 6;
        self.num_prints = OUTPUT.len();

        for a in &mut self.a {
            a.flipped = None;
        }

        for b in &mut self.a {
            b.flipped = None;
        }

        for c in &mut self.a {
            c.flipped = None;
        }
    }
}

fn main() -> Result<(), Error> {
    let mut entrance = make_entrance();
    let mut exit = make_exit();

    loop {
        while exit.step_back() {}
        assert!(entrance.merge_from(&exit));
        println!("score is {}", exit.score());
        exit.reset_to_exit();
    }

    // Ok(())
}
