#!/usr/bin/env python3
from typing import Tuple
import sys

class Network:
    def __init__(self):
        self.adjacent: set[Tuple[str, str]] = set()
        self.hosts: set[str] = set()

        for ln in sys.stdin:
            ln = ln.rstrip()
            a, b = ln.split('-')
            if a > b:
                a, b = b, a
            self.adjacent.add((a, b))
            self.hosts.add(a)
            self.hosts.add(b)

    def is_adjacent(self, a: str, b: str) -> bool:
        if a > b:
            a, b = b, a

        return (a, b) in self.adjacent

def find_cluster(net: Network, seed: str) -> set[str]:
    cluster = {seed}
    while True:
        len_before = len(cluster)
        for host in net.hosts:
            if all(map(lambda c: net.is_adjacent(c, host), cluster)):
                cluster.add(host)
        if len(cluster) == len_before:
            break

    return cluster

def main():
    net = Network()
    count = 0

    largest_cluster = set()
    for host in net.hosts:
        cluster = find_cluster(net, host)
        if not any(map(lambda h: h.startswith('t'), cluster)):
            continue
        if len(cluster) > len(largest_cluster):
            largest_cluster = cluster

    print(','.join(sorted(list(largest_cluster))))

if __name__ == '__main__':
    main()
