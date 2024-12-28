#![feature(get_many_mut)]
use anyhow::{anyhow, bail, Error};
use std::collections::HashMap;
use std::io::BufRead;

const PART_TWO: bool = true;

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Gate {
    left_wire_id: usize,
    operator: Operator,
    right_wire_id: usize,
    output_wire_id: usize,
}

#[derive(Debug)]
struct Circuit {
    wire_to_name: Vec<String>,
    initial_state: Vec<Option<bool>>,
    gates: Vec<Gate>,
    x_wire_ids: Vec<usize>,
    y_wire_ids: Vec<usize>,
    z_wire_ids: Vec<usize>,
    wire_to_gate: Vec<Option<usize>>,
    max_adder_len: usize,
}

fn get_wire_id(wire_ids: &mut HashMap<String, usize>, name: &str) -> usize {
    match wire_ids.get(name) {
        Some(id) => *id,
        None => {
            let id = wire_ids.len();
            wire_ids.insert(name.to_owned(), id);
            id
        }
    }
}

fn get_wire_ids_by_prefix(wire_ids: &HashMap<String, usize>, prefix: &str) -> Vec<usize> {
    let mut z_names: Vec<_> = wire_ids
        .iter()
        .filter(|(name, _id)| name.starts_with(prefix))
        .collect();

    z_names.sort_by_key(|(name, _id)| *name);
    z_names.reverse();

    z_names.iter().map(|(_name, id)| **id).collect()
}

fn parse_gate(s: &str, wire_ids: &mut HashMap<String, usize>) -> Result<Gate, Error> {
    let tokens: Vec<_> = s.split_ascii_whitespace().collect();

    if tokens.len() != 5 {
        bail!("gate not enough tokens")
    }

    let left_input = get_wire_id(wire_ids, tokens[0]);
    let operator = match tokens[1] {
        "AND" => Operator::And,
        "OR" => Operator::Or,
        "XOR" => Operator::Xor,
        _ => bail!("gate invalid operator"),
    };

    let right_input = get_wire_id(wire_ids, tokens[2]);
    if tokens[3] != "->" {
        bail!("gate missing arrow");
    }

    let output = get_wire_id(wire_ids, tokens[4]);
    Ok(Gate {
        left_wire_id: left_input,
        operator,
        right_wire_id: right_input,
        output_wire_id: output,
    })
}

fn parse_initial_value(
    s: &str,
    wire_ids: &mut HashMap<String, usize>,
) -> Result<(usize, bool), Error> {
    let (k, v) = s
        .split_once(": ")
        .ok_or_else(|| anyhow!("bad initial value"))?;

    let k = get_wire_id(wire_ids, k);

    let v = match v {
        "0" => false,
        "1" => true,
        _ => bail!("bad initial value"),
    };

    Ok((k, v))
}

fn read_circuit(rd: impl BufRead) -> Result<Circuit, Error> {
    let mut wire_ids = HashMap::new();
    let mut initial_values = HashMap::new();
    let mut gates = Vec::new();
    let mut in_gates = false;

    for ln in rd.lines() {
        let ln = ln?;

        if ln.is_empty() {
            in_gates = true;
            continue;
        }

        if in_gates {
            gates.push(parse_gate(&ln, &mut wire_ids)?);
        } else {
            let (k, v) = parse_initial_value(&ln, &mut wire_ids)?;
            initial_values.insert(k, v);
        }
    }

    let x_wire_ids = get_wire_ids_by_prefix(&wire_ids, "x");
    let y_wire_ids = get_wire_ids_by_prefix(&wire_ids, "y");
    let z_wire_ids = get_wire_ids_by_prefix(&wire_ids, "z");

    let mut initial_state = vec![None; wire_ids.len()];
    for (id, val) in &initial_values {
        initial_state[*id] = Some(*val);
    }

    let mut wire_names = Vec::new();
    wire_names.resize_with(wire_ids.len(), Default::default);

    let mut wire_to_gate = Vec::new();
    wire_to_gate.resize_with(wire_ids.len(), Default::default);

    // fill in wire_names
    for (name, id) in wire_ids.into_iter() {
        wire_names[id] = name;
    }

    // fill in wire_to_gate
    for wire_id in 0..wire_names.len() {
        // don't try going upstream from input gates
        if x_wire_ids.contains(&wire_id) || y_wire_ids.contains(&wire_id) {
            continue;
        }

        let mut found = false;
        for gate_id in 0..gates.len() {
            if gates[gate_id].output_wire_id == wire_id {
                wire_to_gate[wire_id] = Some(gate_id);
                found = true;
                break;
            }
        }

        if !found {
            panic!("no upstream gate for wire {}", wire_names[wire_id]);
        }
    }

    Ok(Circuit {
        wire_to_name: wire_names,
        gates,
        x_wire_ids,
        y_wire_ids,
        z_wire_ids,
        initial_state,
        wire_to_gate,
        max_adder_len: 0,
    })
}

fn propagate(wire_values: &[Option<bool>], gate: &Gate) -> Option<bool> {
    let left = wire_values[gate.left_wire_id]?;
    let right = wire_values[gate.right_wire_id]?;

    let val = match gate.operator {
        Operator::And => left && right,
        Operator::Or => left || right,
        Operator::Xor => left ^ right,
    };

    Some(val)
}

fn solve_circuit(circuit: &Circuit, wire_values: &mut [Option<bool>]) {
    let mut remaining = wire_values.iter().filter(|v| v.is_none()).count();
    while remaining > 0 {
        let mut new_remaining = remaining;
        for gate in &circuit.gates {
            if wire_values[gate.output_wire_id].is_none() {
                if let Some(val) = propagate(&wire_values, gate) {
                    wire_values[gate.output_wire_id] = Some(val);
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

fn get_value(wire_values: &[Option<bool>], wire_ids: &[usize]) -> Option<usize> {
    let mut value = 0;

    for wire_id in wire_ids {
        let val = wire_values[*wire_id]?;
        let val = usize::from(val);
        value = value * 2 + val;
    }

    Some(value)
}

fn format_wire_list(circuit: &Circuit, wire_ids: &[usize]) -> String {
    let mut s = String::new();

    for wire_id in wire_ids {
        if !s.is_empty() {
            s.push(',');
        }

        s.push_str(&circuit.wire_to_name[*wire_id]);
    }

    s
}

fn clear_wire_values(wire_values: &mut [Option<bool>]) {
    wire_values.fill(None);
}

fn set_wire_values(wire_ids: &[usize], mut val: usize, wire_values: &mut [Option<bool>]) {
    for wire_id in wire_ids.iter().rev() {
        let val_bool = (val & 1) != 0;
        val >>= 1;
        wire_values[*wire_id] = Some(val_bool);
    }
    assert_eq!(val, 0);
}

fn value_fits(val: usize, wire_ids: &[usize]) -> bool {
    let biggest_val = (1 << wire_ids.len()) - 1;

    val <= biggest_val
}

fn is_adder_one(
    circuit: &Circuit,
    wire_values: &mut [Option<bool>],
    x: usize,
    y: usize,
    x_wires: &[usize],
    y_wires: &[usize],
    z_wires: &[usize],
) -> bool {
    if !value_fits(x, x_wires) {
        return true;
    }

    if !value_fits(y, y_wires) {
        return true;
    }

    clear_wire_values(wire_values);
    set_wire_values(x_wires, x, wire_values);
    set_wire_values(y_wires, y, wire_values);

    solve_circuit(circuit, wire_values);
    let z = get_value(wire_values, z_wires);

    // println!("is_adder_one(x = {x}, y = {y}, z = {z:?})");

    let Some(z) = z else {
        return false;
    };

    x + y == z
}
fn last_n_wires(wire_ids: &[usize], n: usize) -> &[usize] {
    if n < wire_ids.len() {
        &wire_ids[(wire_ids.len() - 1 - n)..]
    } else {
        wire_ids
    }
}

// verify that an adder, that has been verified to be an adder for num_bits-1 bits, is an adder as well
fn calc_adder_size(circuit: &Circuit, wire_values: &mut [Option<bool>]) -> usize {
    // every circuit is a 0 bit adder :d
    for num_bits in 1..circuit.z_wire_ids.len() {
        let x_wires = last_n_wires(&circuit.x_wire_ids, num_bits);
        let y_wires = last_n_wires(&circuit.y_wire_ids, num_bits);
        let z_wires = last_n_wires(&circuit.z_wire_ids, num_bits);

        let test_mid = (1 << num_bits) - 1;
        let test_lo = test_mid - 1;
        let test_hi = test_mid + 1;

        let test_vals = [test_lo, test_mid, test_hi];

        for test_i in 0..test_vals.len() {
            for test_j in test_i + 1..test_vals.len() {
                if !is_adder_one(
                    circuit,
                    wire_values,
                    test_vals[test_i],
                    test_vals[test_j],
                    x_wires,
                    y_wires,
                    z_wires,
                ) {
                    return num_bits;
                }

                if !is_adder_one(
                    circuit,
                    wire_values,
                    test_vals[test_j],
                    test_vals[test_i],
                    x_wires,
                    y_wires,
                    z_wires,
                ) {
                    return num_bits;
                }
            }
        }
    }

    circuit.z_wire_ids.len()
}

fn swap_outputs(circuit: &mut Circuit, i: usize, j: usize) {
    let [i, j] = circuit.gates.get_many_mut([i, j]).unwrap();
    std::mem::swap(&mut i.output_wire_id, &mut j.output_wire_id);
}

// try to fix the circuit by swapping gates
// upon entry, you've everified that the circuit functions as a num_bits adder and performed the swaps in 'swaps'
// if it returns false, the circuit can't be fixed the current swaps, and the last swap should be undone and another tried
fn fix_circuit(
    circuit: &mut Circuit,
    expected_bits: usize,
    swaps: &mut Vec<(usize, usize)>,
    wire_values: &mut [Option<bool>],
) -> bool {
    // println!("on {expected_bits}");
    let actual_bits = calc_adder_size(circuit, wire_values);

    if actual_bits > circuit.max_adder_len {
        println!("found addr of len {actual_bits}");
        circuit.max_adder_len = actual_bits;
    }

    if actual_bits < expected_bits {
        // we've been give a circuit that doesn't work as much as it should!
        return false;
    }

    if actual_bits >= circuit.z_wire_ids.len() {
        // win condition
        return true;
    }

    for gate_i in 0..circuit.gates.len() {
        for gate_j in gate_i + 1..circuit.gates.len() {
            swap_outputs(circuit, gate_i, gate_j);
            swaps.push((gate_i, gate_j));

            if fix_circuit(circuit, actual_bits + 1, swaps, wire_values) {
                // undo swaps before returning
                // for (gate_i, gate_j) in swaps {
                //     swap_outputs(circuit, *gate_i, *gate_j);
                // }
                return true;
            }
            // swap didn't work out somewhere down the line, undo it and try another
            swap_outputs(circuit, gate_i, gate_j);
            swaps.pop();
        }
    }

    false
}

fn format_swaps(circuit: &Circuit, swaps: &[(usize, usize)]) -> String {
    let mut names: Vec<&str> = Vec::new();

    for (gate_i, gate_j) in swaps {
        names.push(&circuit.wire_to_name[circuit.gates[*gate_i].output_wire_id]);
        names.push(&circuit.wire_to_name[circuit.gates[*gate_j].output_wire_id]);
    }

    names.sort();

    names.join(",")
}

fn main() -> Result<(), Error> {
    let mut circuit = read_circuit(std::io::stdin().lock())?;

    println!(
        "{} {} {}",
        circuit.x_wire_ids.len(),
        circuit.y_wire_ids.len(),
        circuit.z_wire_ids.len(),
    );

    if PART_TWO {
        let mut swaps = Vec::new();
        let mut wire_values: Vec<Option<bool>> = vec![None; circuit.initial_state.len()];
        fix_circuit(&mut circuit, 0, &mut swaps, &mut wire_values);
        println!("{}", format_swaps(&circuit, &swaps));
    } else {
        let mut wire_values = circuit.initial_state.clone();
        solve_circuit(&circuit, &mut wire_values);
        let x = get_value(&wire_values, &circuit.x_wire_ids);
        let y = get_value(&wire_values, &circuit.y_wire_ids);
        let z = get_value(&wire_values, &circuit.z_wire_ids);
        println!("x = {x:?}, y = {y:?}, z = {z:?}");
    }
    Ok(())
}
