//! Benchmark the `Biclique::biclique` implementation for different types.

use {
    graaf::{
        AddArc,
        AdjacencyList,
        AdjacencyMatrix,
        Biclique,
        Empty,
    },
    std::collections::HashSet,
};

fn main() {
    divan::main();
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_list_naive(m: usize, n: usize) -> AdjacencyList {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let mut digraph = AdjacencyList::empty(order);

    for u in 0..m {
        for v in m..order {
            digraph.add_arc(u, v);
            digraph.add_arc(v, u);
        }
    }

    digraph
}

struct AdjacencyListHashSet {
    arcs: Vec<HashSet<usize>>,
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_list_hash_set(
    m: usize,
    n: usize,
) -> AdjacencyListHashSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let clique_1 = (0..m).collect::<HashSet<_>>();
    let clique_2 = (m..order).collect::<HashSet<_>>();

    let mut digraph = AdjacencyListHashSet {
        arcs: vec![HashSet::new(); order],
    };

    for u in 0..m {
        digraph.arcs[u].clone_from(&clique_2);
    }

    for u in m..order {
        digraph.arcs[u].clone_from(&clique_1);
    }

    digraph
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
])]
fn adjacency_list_naive_0((m, n): (usize, usize)) {
    let _ = biclique_adjacency_list_naive(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
])]
fn adjacency_list_hash_set((m, n): (usize, usize)) {
    let _ = biclique_adjacency_list_hash_set(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
])]
fn adjacency_list((m, n): (usize, usize)) {
    let _ = AdjacencyList::biclique(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
])]
fn adjacency_matrix((m, n): (usize, usize)) {
    let _ = AdjacencyMatrix::biclique(m, n);
}
