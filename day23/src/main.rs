#![feature(get_many_mut)]
use anyhow::{anyhow, Error};
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

const PART_TWO: bool = false;

#[derive(Debug)]
struct Network {
    host_names: Vec<String>,
    adjacent: HashSet<(usize, usize)>,
}

fn get_key(mut host_a: usize, mut host_b: usize) -> (usize, usize) {
    if host_a > host_b {
        std::mem::swap(&mut host_a, &mut host_b);
    }

    (host_a, host_b)
}

impl Network {
    fn len(&self) -> usize {
        self.host_names.len()
    }

    fn is_adjacent(&self, host_a: usize, host_b: usize) -> bool {
        self.adjacent.contains(&get_key(host_a, host_b))
    }
}

fn get_host_id(host_ids: &mut HashMap<String, usize>, host: &str) -> usize {
    match host_ids.get(host) {
        Some(id) => *id,
        None => {
            let id = host_ids.len();
            host_ids.insert(host.to_owned(), id);
            id
        }
    }
}

fn find_cluster(host_id: usize, network: &Network) -> HashSet<usize> {
    let mut cluster = HashSet::new();

    cluster.insert(host_id);

    loop {
        let mut changed = false;
        println!("loop with {}", cluster.len());

        for host_id_2 in 0..network.len() {
            if cluster
                .iter()
                .all(|host_id| network.is_adjacent(host_id_2, *host_id))
            {
                cluster.insert(host_id_2);
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

    cluster
}

fn read_network(rd: impl BufRead) -> Result<Network, Error> {
    let mut host_ids = HashMap::new();

    // keyed by host id
    let mut adjacent = HashSet::new();

    for ln in rd.lines() {
        let ln = ln?;
        let delim = ln
            .find('-')
            .ok_or_else(|| anyhow!("no delimiter in line"))?;
        let host_a = &ln[..delim];
        let host_b = &ln[delim + 1..];
        let host_a = get_host_id(&mut host_ids, host_a);
        let host_b = get_host_id(&mut host_ids, host_b);

        adjacent.insert(get_key(host_a, host_b));
    }

    // convert host_ids to host_names
    let mut host_names = Vec::new();
    host_names.resize_with(host_ids.len(), String::default);

    for (name, id) in host_ids.into_iter() {
        host_names[id] = name;
    }

    Ok(Network {
        host_names,
        adjacent,
    })
}

fn main() -> Result<(), Error> {
    let network = read_network(std::io::stdin().lock())?;

    for host_id in 0..network.len() {
        let c = find_cluster(host_id, &network);
        println!("{c:?}");
    }

    Ok(())
}
