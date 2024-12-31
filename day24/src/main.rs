#![feature(coroutines)]
#![feature(get_many_mut)]
use crate::circuit::{Bus, Circuit, CircuitState, GateRef, Operator};
use anyhow::Error;
use rand::Rng as _;
use rand::SeedableRng;
use std::collections::HashSet;

mod circuit;

const PART_TWO: bool = false;

fn propagate(state: &CircuitState, gate: GateRef) -> Option<bool> {
    let left = state.get(gate.left_input())?;
    let right = state.get(gate.right_input())?;
    let val = match gate.operator() {
        Operator::And => left && right,
        Operator::Or => left || right,
        Operator::Xor => left ^ right,
    };

    Some(val)
}

#[allow(unused)]
fn solve_circuit_slow(circuit: &Circuit, state: &mut CircuitState) {
    let mut remaining = circuit
        .iter_wires()
        .filter(|w| state.get(*w).is_none())
        .count();

    while remaining > 0 {
        let mut new_remaining = remaining;
        for gate in circuit.iter_gates() {
            if state.get(gate.output()).is_none() {
                if let Some(val) = propagate(state, gate) {
                    state.set(gate.output(), val);
                    new_remaining -= 1;
                }
            }
        }

        if new_remaining == remaining {
            break;
        }

        remaining = new_remaining;
    }
}

fn solve_circuit_fast(circuit: &Circuit, state: &mut CircuitState) {
    for gate in circuit.iter_gates() {
        let Some(val) = propagate(state, gate) else {
            return;
        };

        state.set(gate.output(), val);
    }
}

fn set_value(
    circuit: &Circuit,
    state: &mut CircuitState,
    bus: Bus,
    mut value: usize,
    mut num_bits: usize,
) {
    for wire in circuit.iter_bus_lsb(bus) {
        if num_bits == 0 {
            break;
        }

        let val = value % 2 == 0;
        state.set(wire, val);
        value >>= 1;
        num_bits -= 1;
        //
    }
}

fn get_value(
    circuit: &Circuit,
    state: &CircuitState,
    bus: Bus,
    mut num_bits: usize,
) -> Option<usize> {
    let mut value = 0;

    for wire in circuit.iter_bus_msb(bus) {
        if num_bits == 0 {
            break;
        }

        let val = state.get(wire)?;

        let val = usize::from(val);
        value = value * 2 + val;
        num_bits -= 1
    }

    Some(value)
}

#[derive(Default)]
struct DistanceChecker {
    seen: HashSet<usize>,
}

impl DistanceChecker {
    fn distance_up(&mut self, gate: GateRef) -> usize {
        self.seen.clear();

        let mut num_gates = 0;

        gate.output().id();

        let mut gate_queue = vec![gate];

        while let Some(gate) = gate_queue.pop() {
            if !self.seen.insert(gate.output().id()) {
                // already saw this gate
                break;
            }

            num_gates += 1;

            if let Some(left_gate) = gate.left_input().gate() {
                gate_queue.push(left_gate);
            }

            if let Some(right_gate) = gate.right_input().gate() {
                gate_queue.push(right_gate);
            }
        }

        num_gates
    }
}

fn fuzz_adder(circuit: &Circuit, num_bits: usize, mut rng: impl rand::RngCore) -> bool {
    // special case?
    if num_bits == 0 {
        return true;
    }

    let max_val = (1 << num_bits) - 1;
    let mut st = circuit.empty_state();

    for _i in 0..10 {
        let x: usize = rng.gen_range(0..=max_val);
        let y: usize = rng.gen_range(0..=max_val);
        st.clear();
        set_value(circuit, &mut st, Bus::X, x, num_bits);
        set_value(circuit, &mut st, Bus::Y, y, num_bits);
        // can't use fast, gate order may have changed
        solve_circuit_slow(circuit, &mut st);

        let Some(z) = get_value(circuit, &st, Bus::Z, num_bits + 1) else {
            return false;
        };

        println!("x={x}, y={y}, z={z}");

        if (x + y) & max_val != z {
            return false;
        }
    }

    true
}

fn expected_distance(bit_num: usize) -> Option<usize> {
    // match bit_num {
    //     0 => 1,
    //     1 => 3,
    //     45 => 222,
    //     _ => (bit_num - 1) * 4 + 3,
    // }

    // at bit 45, this is 222 (all gates in circuit)
    // at low gate counts, lets just not care for now
    match bit_num {
        0..5 => None,
        _ => Some((bit_num - 1) * 5 + 2),
    }
}

fn bits_match(circuit: &Circuit, checker: &mut DistanceChecker) -> usize {
    let mut n = 0;
    for (bit_num, wire) in circuit.iter_bus_lsb(Bus::Z).enumerate() {
        let gate = wire.gate().unwrap();
        let Some(expected_dist) = expected_distance(bit_num) else {
            n += 1;
            continue;
        };

        if checker.distance_up(gate) == expected_dist {
            n += 1;
        } else {
            break;
        }
    }
    n
}

fn print_expectations(circuit: &Circuit, checker: &mut DistanceChecker) {
    for (bit_num, wire) in circuit.iter_bus_lsb(Bus::Z).enumerate() {
        let gate = wire.gate().unwrap();
        let actual = checker.distance_up(gate);
        let Some(expected) = expected_distance(bit_num) else {
            continue;
        };

        let status = if actual != expected { "BAD" } else { "GOOD" };

        println!("{wire}, {actual}, {expected} {status}");
    }
}

const ENABLE_FUZZ: bool = true;

fn main() -> Result<(), Error> {
    let mut circuit = Circuit::read(std::io::stdin().lock())?;
    let mut rng = rand::rngs::StdRng::seed_from_u64(1337);

    if PART_TWO {
        // num_and = 89
        // num_or = 44
        // num_xor = 89
        // https://en.m.wikipedia.org/wiki/Adder_(electronics)#/media/File%3AFull-adder_logic_diagram.svg

        // "Schematic of full adder implemented with two XOR gates, two AND gates, one OR gate."
        // so 44 full adders worth of ors
        // 44.5 full adders worh of and and xors

        let mut checker = DistanceChecker::default();
        print_expectations(&circuit, &mut checker);

        let mut curr_bits_match = 0;
        let mut swaps: Vec<(usize, usize)> = Vec::new();

        loop {
            println!("top of loop");
            for gate_i in 0..circuit.len() {
                for gate_j in gate_i + 1..circuit.len() {
                    circuit.swap_outputs(gate_i, gate_j);
                    let new_bits_match = bits_match(&circuit, &mut checker);

                    match new_bits_match.cmp(&curr_bits_match) {
                        std::cmp::Ordering::Less => {
                            // made things worse, undo
                            circuit.swap_outputs(gate_i, gate_j);
                        }
                        std::cmp::Ordering::Equal => {
                            // no improvement, undo
                            circuit.swap_outputs(gate_i, gate_j);
                        }
                        std::cmp::Ordering::Greater => {
                            // improvement, fuzz to verify

                            let mut tmp_bits_match = curr_bits_match;
                            if ENABLE_FUZZ {
                                while tmp_bits_match < new_bits_match {
                                    if fuzz_adder(&circuit, tmp_bits_match, &mut rng) {
                                        tmp_bits_match += 1;
                                    } else {
                                        break;
                                    }
                                }
                            } else {
                                tmp_bits_match = new_bits_match;
                            }

                            if tmp_bits_match < new_bits_match {
                                // failed fuzz, undo
                                circuit.swap_outputs(gate_i, gate_j);
                                println!("fuzz fail");
                            } else {
                                println!("{curr_bits_match} improved to {new_bits_match}");
                                print_expectations(&circuit, &mut checker);
                                swaps.push((gate_i, gate_j));
                                println!("{swaps:?}");
                                curr_bits_match = new_bits_match;
                            }
                        }
                    }
                }
            }
        }
    } else {
        let mut wire_values = circuit.initial_state();
        solve_circuit_fast(&circuit, &mut wire_values);
        let x = get_value(&circuit, &wire_values, Bus::X, 45).unwrap();
        let y = get_value(&circuit, &wire_values, Bus::Y, 45).unwrap();
        let z = get_value(&circuit, &wire_values, Bus::Z, 46).unwrap();
        println!("x = {x:?}, y = {y:?}, z = {z:?}");
    }
    Ok(())
}
