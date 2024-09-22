//! Benchmark the `Path::path` implementation for different types.

use {
    graaf::{
        AdjacencyList,
        AdjacencyMatrix,
        Path,
    },
    std::collections::HashSet,
};

fn main() {
    divan::main();
}

pub struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

fn path_adjacency_list_hash_set(order: usize) -> AdjacencyListHashSet {
    let mut arcs = vec![HashSet::new(); order];

    for u in 0..order - 1 {
        let v = u + 1;

        arcs[u].insert(v);
        arcs[v].insert(u);
    }

    AdjacencyListHashSet { arcs }
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::path(n);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_hash_set(n: usize) {
    let _ = path_adjacency_list_hash_set(n);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::path(n);
}
