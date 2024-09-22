//! Benchmark the `Cycle::cycle` implementation for different types.

use {
    graaf::{
        AdjacencyList,
        AdjacencyMatrix,
        Cycle,
    },
    std::collections::HashSet,
};

fn main() {
    divan::main();
}

pub struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

fn cycle_adjacency_list_hash_set(order: usize) -> AdjacencyListHashSet {
    let mut arcs = vec![HashSet::new(); order];

    if order == 1 {
        return AdjacencyListHashSet { arcs };
    }

    for u in 0..order - 1 {
        let v = u + 1;

        arcs[u].insert(v);
        arcs[v].insert(u);
    }

    let u = order - 1;

    arcs[u].insert(0);
    arcs[0].insert(u);

    AdjacencyListHashSet { arcs }
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::cycle(n);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_hash_set(n: usize) {
    let _ = cycle_adjacency_list_hash_set(n);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::cycle(n);
}
