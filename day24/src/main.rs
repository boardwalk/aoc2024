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
    initial_state: Vec<Option<bool>>,
    gates: Vec<Gate>,
    x_wire_ids: Vec<usize>,
    y_wire_ids: Vec<usize>,
    z_wire_ids: Vec<usize>,
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

    Ok(Circuit {
        gates,
        x_wire_ids,
        y_wire_ids,
        z_wire_ids,
        initial_state,
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

fn solve_circuit(circuit: &Circuit) -> Result<Vec<Option<bool>>, Error> {
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
            bail!("did not make progress");
        }
        remaining = new_remaining;
    }

    Ok(wire_values)
}

fn get_value(wire_values: &[Option<bool>], wire_ids: &[usize]) -> usize {
    let mut value = 0;

    for wire_id in wire_ids {
        let val = wire_values[*wire_id].unwrap();
        let val = usize::from(val);
        value = value * 2 + val;
    }

    value
}

fn set_value(wire_values: &mut [Option<bool>], wire_ids: &[usize], mut value: usize) {
    for wire_id in wire_ids {
        assert!(wire_values[*wire_id].is_none());
        wire_values[*wire_id] = Some((value % 2) != 0);
        value /= 2;
    }

    assert!(value == 0);
}

fn main() -> Result<(), Error> {
    let circuit = read_circuit(std::io::stdin().lock())?;
    let wire_values = solve_circuit(&circuit)?;
    let x = get_value(&wire_values, &circuit.x_wire_ids);
    let y = get_value(&wire_values, &circuit.y_wire_ids);
    let z = get_value(&wire_values, &circuit.z_wire_ids);

    println!("x = {x}, y = {y}, z = {z}");
    Ok(())
}
