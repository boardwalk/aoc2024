#![feature(coroutines)]
#![feature(get_many_mut)]
use crate::circuit::{Circuit, CircuitState, GateRef, Operator};
use anyhow::Error;

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
                    println!("solved {gate}");
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

        println!("solved {gate}");
        state.set(gate.output(), val);
    }
}

fn get_value(circuit: &Circuit, state: &CircuitState, cls: char) -> Option<usize> {
    let mut value = 0;

    let wires = circuit.bus(cls);

    for wire in wires.iter().rev() {
        let val = state.get(*wire)?;
        let val = usize::from(val);
        value = value * 2 + val;
    }

    Some(value)
}

fn main() -> Result<(), Error> {
    let circuit = Circuit::read(std::io::stdin().lock())?;
    if PART_TWO {
        // num_and = 89
        // num_or = 44
        // num_xor = 89
        // https://en.m.wikipedia.org/wiki/Adder_(electronics)#/media/File%3AFull-adder_logic_diagram.svg

        // "Schematic of full adder implemented with two XOR gates, two AND gates, one OR gate."
        // so 44 full adders worth of ors
        // 44.5 full adders worh of and and xors
    } else {
        let mut wire_values = circuit.initial_state();
        solve_circuit_fast(&circuit, &mut wire_values);
        let x = get_value(&circuit, &wire_values, 'x');
        let y = get_value(&circuit, &wire_values, 'y');
        let z = get_value(&circuit, &wire_values, 'z');
        println!("x = {x:?}, y = {y:?}, z = {z:?}");
    }
    Ok(())
}
