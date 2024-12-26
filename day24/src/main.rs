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
    left_input: usize,
    operator: Operator,
    right_input: usize,
    output: usize,
}

#[derive(Debug)]
struct Circuit {
    initial_state: Vec<Option<bool>>,
    gates: Vec<Gate>,
    x_wires: Vec<usize>,
    y_wires: Vec<usize>,
    z_wires: Vec<usize>,
}

fn get_wire_id(wires: &mut HashMap<String, usize>, name: &str) -> usize {
    match wires.get(name) {
        Some(id) => *id,
        None => {
            let id = wires.len();
            wires.insert(name.to_owned(), id);
            id
        }
    }
}

fn get_io_wires(wires: &HashMap<String, usize>, prefix: &str) -> Vec<usize> {
    let mut z_names: Vec<_> = wires
        .iter()
        .filter(|(name, _id)| name.starts_with(prefix))
        .collect();

    z_names.sort_by_key(|(name, _id)| *name);
    z_names.reverse();

    z_names.iter().map(|(_name, id)| **id).collect()
}

fn parse_gate(s: &str, wires: &mut HashMap<String, usize>) -> Result<Gate, Error> {
    let tokens: Vec<_> = s.split_ascii_whitespace().collect();

    if tokens.len() != 5 {
        bail!("gate not enough tokens")
    }

    let left_input = get_wire_id(wires, tokens[0]);
    let operator = match tokens[1] {
        "AND" => Operator::And,
        "OR" => Operator::Or,
        "XOR" => Operator::Xor,
        _ => bail!("gate invalid operator"),
    };

    let right_input = get_wire_id(wires, tokens[2]);
    if tokens[3] != "->" {
        bail!("gate missing arrow");
    }

    let output = get_wire_id(wires, tokens[4]);
    Ok(Gate {
        left_input,
        operator,
        right_input,
        output,
    })
}

fn parse_initial_value(
    s: &str,
    wires: &mut HashMap<String, usize>,
) -> Result<(usize, bool), Error> {
    let (k, v) = s
        .split_once(": ")
        .ok_or_else(|| anyhow!("bad initial value"))?;

    let k = get_wire_id(wires, k);

    let v = match v {
        "0" => false,
        "1" => true,
        _ => bail!("bad initial value"),
    };

    Ok((k, v))
}

fn read_circuit(rd: impl BufRead) -> Result<Circuit, Error> {
    let mut initial_values = HashMap::new();
    let mut gates = Vec::new();
    let mut in_gates = false;
    let mut wires = HashMap::new();

    for ln in rd.lines() {
        let ln = ln?;

        if ln.is_empty() {
            in_gates = true;
            continue;
        }

        if in_gates {
            gates.push(parse_gate(&ln, &mut wires)?);
        } else {
            let (k, v) = parse_initial_value(&ln, &mut wires)?;
            initial_values.insert(k, v);
        }
    }

    let x_wires = get_io_wires(&wires, "x");
    let y_wires = get_io_wires(&wires, "y");
    let z_wires = get_io_wires(&wires, "z");

    let mut initial_state = vec![None; wires.len()];
    for (id, val) in &initial_values {
        initial_state[*id] = Some(*val);
    }

    Ok(Circuit {
        gates,
        x_wires,
        y_wires,
        z_wires,
        initial_state,
    })
}

fn propagate(wires: &[Option<bool>], gate: &Gate) -> Option<bool> {
    let left = wires[gate.left_input]?;
    let right = wires[gate.right_input]?;

    let val = match gate.operator {
        Operator::And => left && right,
        Operator::Or => left || right,
        Operator::Xor => left ^ right,
    };

    Some(val)
}

fn solve_circuit(circuit: &Circuit) -> Result<Vec<bool>, Error> {
    let mut wires = circuit.initial_state.clone();
    let mut remaining = wires.iter().filter(|v| v.is_none()).count();
    while remaining > 0 {
        let mut new_remaining = remaining;
        for gate in &circuit.gates {
            if wires[gate.output].is_none() {
                if let Some(val) = propagate(&wires, gate) {
                    wires[gate.output] = Some(val);
                    new_remaining -= 1;
                }
            }
        }

        if new_remaining == remaining {
            bail!("did not make progress");
        }
        remaining = new_remaining;
    }

    let wires = wires.into_iter().map(Option::unwrap).collect();

    Ok(wires)
}

fn calculate(wires: &[bool], circuit: &Circuit) -> usize {
    let mut output = 0;

    for io_wire in &circuit.z_wires {
        let val = wires[*io_wire];
        let val = usize::from(val);
        output = output * 2 + val;
    }

    output
}

fn main() -> Result<(), Error> {
    let circuit = read_circuit(std::io::stdin().lock())?;
    let wires = solve_circuit(&circuit)?;
    let output = calculate(&wires, &circuit);
    println!("{output}");
    Ok(())
}
