//! Benchmark the `Star::star` implementation for different types.

use graaf::{
    AdjacencyList,
    AdjacencyMatrix,
    Star,
};

fn main() {
    divan::main();
}

pub struct AdjacencyListHashSet {
    pub arcs: Vec<std::collections::HashSet<usize>>,
}

fn star_adjacency_list_hash_set(order: usize) -> AdjacencyListHashSet {
    let mut arcs = vec![std::collections::HashSet::new(); order];

    for v in 1..order {
        arcs[0].insert(v);
        arcs[v].insert(0);
    }

    AdjacencyListHashSet { arcs }
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::star(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_hash_set(n: usize) {
    let _ = star_adjacency_list_hash_set(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::star(n);
}
