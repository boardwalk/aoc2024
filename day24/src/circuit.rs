use anyhow::{anyhow, bail, Error};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::io::BufRead;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    And,
    Or,
    Xor,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Operator::And => "AND",
            Operator::Or => "OR",
            Operator::Xor => "XOR",
        };

        f.write_str(s)
    }
}

#[derive(Debug)]
struct Gate {
    left_wire_id: usize,
    operator: Operator,
    right_wire_id: usize,
    output_wire_id: usize,
}

#[derive(Clone, Copy)]
pub struct GateRef<'a> {
    circuit: &'a Circuit,
    gate_id: usize,
}

#[derive(Clone, Copy)]
pub struct WireRef<'a> {
    circuit: &'a Circuit,
    wire_id: usize,
}

impl<'a> GateRef<'a> {
    pub fn left_input(self) -> WireRef<'a> {
        let wire_id = self.circuit.gates[self.gate_id].left_wire_id;
        WireRef {
            circuit: self.circuit,
            wire_id,
        }
    }

    pub fn right_input(self) -> WireRef<'a> {
        let wire_id = self.circuit.gates[self.gate_id].right_wire_id;
        WireRef {
            circuit: self.circuit,
            wire_id,
        }
    }

    pub fn output(self) -> WireRef<'a> {
        let wire_id = self.circuit.gates[self.gate_id].output_wire_id;
        WireRef {
            circuit: self.circuit,
            wire_id,
        }
    }

    pub fn operator(self) -> Operator {
        self.circuit.gates[self.gate_id].operator
    }
}

impl<'a> WireRef<'a> {
    #[allow(unused)]
    pub fn gate(self) -> Option<GateRef<'a>> {
        let gate_id = self.circuit.wire_to_gate[self.wire_id]?;
        Some(GateRef {
            circuit: &self.circuit,
            gate_id: gate_id,
        })
    }

    pub fn of_class(self, expected: char) -> bool {
        let Some(actual) = self.circuit.wire_to_name[self.wire_id].chars().next() else {
            return false;
        };

        actual == expected
    }

    pub fn name(self) -> &'a str {
        &self.circuit.wire_to_name[self.wire_id]
    }
}

impl Display for GateRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} -> {}",
            self.left_input(),
            self.operator(),
            self.right_input(),
            self.output()
        )
    }
}

impl Display for WireRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.circuit.wire_to_name[self.wire_id])
    }
}

#[derive(Debug)]
pub struct Circuit {
    wire_to_name: Vec<String>,
    initial_state: Vec<Option<bool>>,
    gates: Vec<Gate>,
    // for some wire, what gate has it as an output?
    // input wires will be None
    wire_to_gate: Vec<Option<usize>>,
}

impl Circuit {
    pub fn read(rd: impl BufRead) -> Result<Self, Error> {
        let mut wire_ids = BTreeMap::new();

        let mut initial_values = BTreeMap::new();
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
            if wire_names[wire_id].starts_with("x") || wire_names[wire_id].starts_with("y") {
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
            initial_state,
            wire_to_gate,
        })
    }

    pub fn initial_state(&self) -> CircuitState {
        CircuitState {
            state: self.initial_state.clone(),
            _circuit: PhantomData,
        }
    }

    #[allow(unused)]
    pub fn empty_state(&self) -> CircuitState {
        CircuitState {
            state: vec![None; self.wire_to_name.len()],
            _circuit: PhantomData,
        }
    }

    pub fn iter_gates(&self) -> impl Iterator<Item = GateRef> {
        tools::iter_coro(
            #[coroutine]
            || {
                for gate_id in 0..self.gates.len() {
                    yield GateRef {
                        circuit: self,
                        gate_id,
                    };
                }
            },
        )
    }

    pub fn iter_wires(&self) -> impl Iterator<Item = WireRef> {
        tools::iter_coro(
            #[coroutine]
            || {
                for wire_id in 0..self.wire_to_name.len() {
                    yield WireRef {
                        circuit: self,
                        wire_id,
                    };
                }
            },
        )
    }

    pub fn bus(&self, cls: char) -> Vec<WireRef> {
        let mut wires: Vec<_> = self.iter_wires().filter(|w| w.of_class(cls)).collect();
        wires.sort_by_key(|w| w.name());
        wires
    }
}

fn get_wire_id(wire_ids: &mut BTreeMap<String, usize>, name: &str) -> usize {
    match wire_ids.get(name) {
        Some(id) => *id,
        None => {
            let id = wire_ids.len();
            wire_ids.insert(name.to_owned(), id);
            id
        }
    }
}

fn parse_gate(s: &str, wire_ids: &mut BTreeMap<String, usize>) -> Result<Gate, Error> {
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
    wire_ids: &mut BTreeMap<String, usize>,
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

pub struct CircuitState<'circuit> {
    state: Vec<Option<bool>>,
    _circuit: PhantomData<&'circuit Circuit>,
}

impl<'circuit> CircuitState<'circuit> {
    pub fn get(&self, wire: WireRef) -> Option<bool> {
        self.state[wire.wire_id]
    }

    pub fn set(&mut self, wire: WireRef, val: bool) {
        self.state[wire.wire_id] = Some(val);
    }
}
