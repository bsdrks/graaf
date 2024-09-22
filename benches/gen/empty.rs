//! Benchmark the `Empty::empty` implementation for different types.

use {
    graaf::{
        AdjacencyList,
        AdjacencyMatrix,
        Empty,
    },
    std::collections::HashSet,
};

fn main() {
    divan::main();
}

pub struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

fn empty_adjacency_list_hash_set(order: usize) -> AdjacencyListHashSet {
    let arcs = vec![HashSet::new(); order];

    AdjacencyListHashSet { arcs }
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_hash_set(n: usize) {
    let _ = empty_adjacency_list_hash_set(n);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::empty(n);
}
