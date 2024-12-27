use anyhow::{anyhow, bail, Error};
use std::collections::HashMap;
use std::io::BufRead;

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

fn solve_circuit(circuit: &Circuit) -> Vec<Option<bool>> {
    let mut wire_values = circuit.initial_state.clone();
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

    wire_values
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

fn main() -> Result<(), Error> {
    let circuit = read_circuit(std::io::stdin().lock())?;
    let wire_values = solve_circuit(&circuit);
    let x = get_value(&wire_values, &circuit.x_wire_ids);
    let y = get_value(&wire_values, &circuit.y_wire_ids);
    let z = get_value(&wire_values, &circuit.z_wire_ids);
    println!("x = {x:?}, y = {y:?}, z = {z:?}");
    Ok(())
}
