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

    pub fn has_class(self, expected: char) -> bool {
        let Some(actual) = self.circuit.wire_to_name[self.wire_id].chars().next() else {
            return false;
        };

        actual == expected
    }

    pub fn name(self) -> &'a str {
        &self.circuit.wire_to_name[self.wire_id]
    }

    pub fn id(self) -> usize {
        self.wire_id
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

fn get_bus_wire_ids(wire_to_name: &[String], cls: char) -> Vec<usize> {
    let mut wires = Vec::new();

    for (id, name) in wire_to_name.iter().enumerate() {
        if name.chars().next().unwrap() == cls {
            wires.push(id)
        }
    }

    wires.sort_by_key(|wire_id| wire_to_name[*wire_id].as_str());
    wires
}

#[derive(Clone, Copy)]
pub enum Bus {
    X,
    Y,
    Z,
}

#[derive(Debug)]
pub struct Circuit {
    wire_to_name: Vec<String>,
    initial_state: Vec<Option<bool>>,
    gates: Vec<Gate>,
    // for some wire, what gate has it as an output?
    // input wires will be None
    wire_to_gate: Vec<Option<usize>>,
    gate_order: Vec<usize>,
    x_bus: Vec<usize>,
    y_bus: Vec<usize>,
    z_bus: Vec<usize>,
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

        let mut wire_to_name = Vec::new();
        wire_to_name.resize_with(wire_ids.len(), Default::default);

        let mut wire_to_gate = Vec::new();
        wire_to_gate.resize_with(wire_ids.len(), Default::default);

        // fill in wire_names
        for (name, id) in wire_ids.into_iter() {
            wire_to_name[id] = name;
        }

        // fill in wire_to_gate
        for wire_id in 0..wire_to_name.len() {
            if wire_to_name[wire_id].starts_with("x") || wire_to_name[wire_id].starts_with("y") {
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
                panic!("no upstream gate for wire {}", wire_to_name[wire_id]);
            }
        }

        // create a list of gates ordered by depth in circuit
        // iterating over the gates in this order will allow you to solve it in one pass
        let mut gate_order: Vec<usize> = (0..gates.len()).collect();

        gate_order.sort_by_cached_key(|gate_id| {
            let mut depth = 1;
            let mut wire_queue: Vec<usize> = Vec::new();
            let gate = &gates[*gate_id];
            wire_queue.push(gate.left_wire_id);
            wire_queue.push(gate.right_wire_id);

            while let Some(wire_id) = wire_queue.pop() {
                if let Some(gate_id) = wire_to_gate[wire_id] {
                    depth += 1;
                    let gate = &gates[gate_id];
                    wire_queue.push(gate.left_wire_id);
                    wire_queue.push(gate.right_wire_id);
                }
            }
            depth
        });

        let x_bus = get_bus_wire_ids(&wire_to_name, 'x');
        let y_bus = get_bus_wire_ids(&wire_to_name, 'y');
        let z_bus = get_bus_wire_ids(&wire_to_name, 'z');

        Ok(Circuit {
            wire_to_name,
            gates,
            initial_state,
            wire_to_gate,
            gate_order,
            x_bus,
            y_bus,
            z_bus,
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
                for gate_id in &self.gate_order {
                    yield GateRef {
                        circuit: self,
                        gate_id: *gate_id,
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

    pub fn iter_bus_lsb(&self, bus: Bus) -> impl Iterator<Item = WireRef> {
        tools::iter_coro(
            #[coroutine]
            move || {
                let bus = match bus {
                    Bus::X => &self.x_bus,
                    Bus::Y => &self.y_bus,
                    Bus::Z => &self.z_bus,
                };

                for wire_id in bus.iter() {
                    yield WireRef {
                        circuit: self,
                        wire_id: *wire_id,
                    };
                }
            },
        )
    }

    pub fn iter_bus_msb(&self, bus: Bus) -> impl Iterator<Item = WireRef> {
        tools::iter_coro(
            #[coroutine]
            move || {
                let bus = match bus {
                    Bus::X => &self.x_bus,
                    Bus::Y => &self.y_bus,
                    Bus::Z => &self.z_bus,
                };

                for wire_id in bus.iter().rev() {
                    yield WireRef {
                        circuit: self,
                        wire_id: *wire_id,
                    };
                }
            },
        )
    }

    pub fn len(&self) -> usize {
        self.gates.len()
    }

    pub fn swap_outputs(&mut self, gate_i: usize, gate_j: usize) {
        // preemptively fix wire_to_gate
        let wire_i = self.gates[gate_i].output_wire_id;
        let wire_j = self.gates[gate_j].output_wire_id;
        self.wire_to_gate.swap(wire_i, wire_j);
        let [gate_i, gate_j] = self.gates.get_many_mut([gate_i, gate_j]).unwrap();
        std::mem::swap(&mut gate_i.output_wire_id, &mut gate_j.output_wire_id);
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

    pub fn clear(&mut self) {
        self.state.fill(None);
    }
}
