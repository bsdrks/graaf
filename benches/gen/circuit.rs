//! Benchmark the `Circuit::circuit` implementation for different types.

use {
    graaf::{
        AddArc,
        AdjacencyList,
        AdjacencyMatrix,
        Circuit,
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

/// # Panics
///
/// Panics if `order` is zero.
fn circuit_adjacency_list_naive(order: usize) -> AdjacencyList {
    let mut digraph = AdjacencyList::empty(order);

    if order == 1 {
        return digraph;
    }

    for u in 0..order - 1 {
        digraph.add_arc(u, u + 1);
    }

    digraph.add_arc(order - 1, 0);

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn circuit_adjacency_list_hash_set(order: usize) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyListHashSet {
            arcs: vec![HashSet::new()],
        };
    }

    AdjacencyListHashSet {
        arcs: (0..order)
            .map(|u| HashSet::from([(u + 1) % order]))
            .collect::<Vec<_>>(),
    }
}

#[divan::bench(args = [10, 100, 1000, 10000, 100000])]
fn adjacency_list_naive(order: usize) {
    let _ = circuit_adjacency_list_naive(order);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100000])]
fn adjacency_list_hash_set(order: usize) {
    let _ = circuit_adjacency_list_hash_set(order);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::circuit(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::circuit(n);
}
