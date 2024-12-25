use anyhow::{anyhow, Error};
use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug)]
struct Network {
    host_names: Vec<String>,
    //  a list of adjacent hosts, indexed by host id
    adjacent: Vec<Vec<usize>>,
}

impl Network {
    fn is_adjacent(&self, host_a: usize, host_b: usize) -> bool {
        self.adjacent[host_a].binary_search(&host_b).is_ok()
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

fn find_clusters(network: &Network) -> Vec<[usize; 3]> {
    let mut clusters = Vec::new();
    // look through all 3 tuples of hosts an add those that are interconnected.
    for host_a in 0..network.host_names.len() {
        for host_b in host_a + 1..network.host_names.len() {
            for host_c in host_b + 1..network.host_names.len() {
                // filter out tuples that are not interconnected
                if !network.is_adjacent(host_a, host_b)
                    || !network.is_adjacent(host_b, host_c)
                    || !network.is_adjacent(host_c, host_a)
                {
                    continue;
                }

                // filter out tuples where none of the hosts starts with t
                if !network.host_names[host_a].starts_with('t')
                    && !network.host_names[host_b].starts_with('t')
                    && !network.host_names[host_c].starts_with('t')
                {
                    continue;
                }

                // these will always be in order
                clusters.push([host_a, host_b, host_c]);
            }
        }
    }

    clusters
}

fn read_network(rd: impl BufRead) -> Result<Network, Error> {
    let mut host_ids = HashMap::new();

    // keyed by host id
    let mut adjacent: Vec<Vec<usize>> = Vec::new();

    for ln in rd.lines() {
        let ln = ln?;
        let delim = ln
            .find('-')
            .ok_or_else(|| anyhow!("no delimiter in line"))?;
        let host_a = &ln[..delim];
        let host_b = &ln[delim + 1..];
        let host_a = get_host_id(&mut host_ids, host_a);
        let host_b = get_host_id(&mut host_ids, host_b);

        if host_a >= adjacent.len() {
            adjacent.push(Vec::new());
        }

        if host_b >= adjacent.len() {
            adjacent.push(Vec::new());
        }

        // mark hosts as adjacent in both directions
        adjacent[host_a].push(host_b);
        adjacent[host_b].push(host_a);
    }

    // convert host_ids to host_names
    let mut host_names = Vec::new();
    host_names.resize_with(host_ids.len(), String::default);

    for (name, id) in host_ids.into_iter() {
        host_names[id] = name;
    }

    // sort lists so we can use binary search
    for lst in &mut adjacent {
        lst.sort_unstable();
    }

    Ok(Network {
        host_names,
        adjacent,
    })
}

fn main() -> Result<(), Error> {
    let network = read_network(std::io::stdin().lock())?;
    // println!("{network:?}");

    let clusters = find_clusters(&network);
    println!("{}", clusters.len());
    Ok(())
}

// start with Vec<Vec<usize>>
// outer is all clusters indexed by host_id (smallest host id in the set maybe?)
// inner is a list of host ids for for hosts in the cluster (sorted if needed
// initial state is a cluster of size 1 for each host id
// iteratively combine clusters of size 1 into into clusters of size 2, and so on
// if a cluster is too small for and doesn't connect to a cluster of the larger size, that cluster goes away
// (the whole cluster is not part of the set of largest if any other cluster is larger)
// continue until trying to merge/grow clusters would mean removing all remaining cluster
// what is left before that are clusters of the largest size
