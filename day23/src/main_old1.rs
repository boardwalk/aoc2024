#![feature(get_many_mut)]
use anyhow::{anyhow, Error};
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

const PART_TWO: bool = false;

#[derive(Debug)]
struct Network {
    host_names: Vec<String>,
    //  a sorted list of adjacent hosts, indexed by host id
    adjacent: Vec<Vec<usize>>,
}

impl Network {
    fn len(&self) -> usize {
        self.adjacent.len()
    }

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
    for host_a in 0..network.len() {
        for host_b in host_a + 1..network.len() {
            for host_c in host_b + 1..network.len() {
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

fn merge_clusters(
    network: &Network,
    src_cluster: &mut Vec<usize>,
    dst_cluster: &mut Vec<usize>,
) -> bool {
    if src_cluster.is_empty() || dst_cluster.is_empty() {
        return false;
    }

    if !PART_TWO && src_cluster.len() + dst_cluster.len() > 2 {
        return false;
    }

    for src_i in 0..src_cluster.len() {
        for dst_i in src_i + 1..dst_cluster.len() {
            if !network.is_adjacent(src_cluster[src_i], dst_cluster[dst_i]) {
                return false;
            }
        }
    }

    dst_cluster.extend(src_cluster.iter());
    src_cluster.clear();
    true
}

fn find_cluster(host_id: usize, network: &Network, max_len: usize) -> Vec<usize> {
    println!("find_cluster(start = {host_id}, max_len = {max_len}");
    let mut cluster = Vec::new();

    let mut seen: HashSet<usize> = HashSet::new();
    let mut work: Vec<usize> = Vec::new();

    work.extend(network.adjacent[host_id].iter());
    // work.push(host_id);

    while let Some(host) = work.pop() {
        // only process a host once
        if seen.contains(&host) {
            continue;
        }
        seen.insert(host);

        // only add hosts that are adjacent to the start
        if !network.is_adjacent(host, host_id) {
            // println!("{host} and {host_id} are not adjacent");
            continue;
        } else {
            // println!("ARE ADJACENT");
        }

        cluster.push(host);
        // if cluster.len() == max_len {
        //     break;
        // }

        for adj in &network.adjacent[host] {
            work.push(*adj);
        }
    }

    // println!("out of work");

    cluster
}

fn is_cluster_valid(cluster: &Vec<usize>, network: &Network) -> bool {
    for i in 0..cluster.len() {
        for j in i + 1..cluster.len() {
            if !network.is_adjacent(cluster[i], cluster[j]) {
                return false;
            }
        }
    }

    true
}

fn format_cluster(cluster: &[usize], network: &Network) -> String {
    let mut res = String::new();
    for host in cluster {
        if !res.is_empty() {
            res.push(',');
        }

        res.push_str(&network.host_names[*host]);
    }
    res
}

fn find_clusters3(network: &Network) {
    // create a cluster for each host
    for host_i in 0..network.len() {
        let cluster = find_cluster(host_i, network, 3);
        println!(
            "cluster for {} is {}",
            host_i,
            format_cluster(&cluster, network)
        )
    }
}

fn find_clusters2(network: &Network) -> Vec<Vec<usize>> {
    // we start with every host being in a cluster by itself, in a vec keyed by host id
    let mut clusters: Vec<Vec<usize>> = Vec::with_capacity(network.len());
    for host_i in 0..network.len() {
        clusters.push(vec![host_i]);
    }

    // a list of cluster pairs that we still need to try and merge
    let mut mergeable_clusters: Vec<(usize, usize)> =
        Vec::with_capacity(network.len() * network.len());
    for cluster_a in 0..network.len() {
        for cluster_b in cluster_a + 1..network.len() {
            mergeable_clusters.push((cluster_a, cluster_b));
            mergeable_clusters.push((cluster_b, cluster_a));
        }
    }

    while let Some((src_cluster_idx, dst_cluster_idx)) = mergeable_clusters.pop() {
        println!("queue size = {}", mergeable_clusters.len());
        let [src_cluster, dst_cluster] = clusters
            .get_many_mut([src_cluster_idx, dst_cluster_idx])
            .unwrap();
        if merge_clusters(network, src_cluster, dst_cluster) {
            mergeable_clusters.push((src_cluster_idx, dst_cluster_idx));
            println!("retry");
        } else {
            println!("no merge");
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
    println!("{network:?}");

    find_clusters3(&network);

    // let clusters = find_clusters2(&network);
    // for (host_0, cluster) in clusters.iter().enumerate() {
    //     for host_i in cluster {
    //         // println!()
    //     }
    //     //
    // }

    // // let clusters: Vec<_> = clusters.into_iter().filter(|c| !c.len() == 3).collect();
    // println!("{}", clusters.len());

    // println!("{clusters:?}");
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
