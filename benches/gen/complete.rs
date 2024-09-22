//! Benchmark the `Complete::complete` implementation for different types.

use {
    graaf::{
        AdjacencyList,
        AdjacencyMatrix,
        Complete,
    },
    std::collections::HashSet,
};

fn main() {
    divan::main();
}

pub struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

fn complete_adjacency_list_hash_set(order: usize) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = vec![HashSet::new(); order];

    for u in 0..order {
        for v in (u + 1)..order {
            arcs[u].insert(v);
            arcs[v].insert(u);
        }
    }

    AdjacencyListHashSet { arcs }
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::complete(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_hash_set(n: usize) {
    let _ = complete_adjacency_list_hash_set(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::complete(n);
}
