//! Benchmarks of different implementations of `Circuit::circuit`.
use {
    graaf::{
        AddArc,
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        Circuit,
        EdgeList,
        Empty,
    },
    std::collections::{
        BTreeSet,
        HashSet,
    },
};

fn main() {
    divan::main();
}

#[allow(dead_code)]
struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct EdgeListBTreeSet {
    pub arcs: BTreeSet<(usize, usize)>,
    pub order: usize,
}

/// # Panics
///
/// Panics if `order` is zero.
fn circuit_adjacency_list_add_arc_empty(order: usize) -> AdjacencyList {
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
fn circuit_adjacency_list_hash_set_collect(
    order: usize,
) -> AdjacencyListHashSet {
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

/// # Panics
///
/// Panics if `order` is zero.
fn circuit_edge_list_btree_set_insert(order: usize) -> EdgeListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return EdgeListBTreeSet {
            arcs: BTreeSet::new(),
            order,
        };
    }

    let mut arcs = BTreeSet::new();

    for u in 0..order {
        let _ = arcs.insert((u, (u + 1) % order));
    }

    EdgeListBTreeSet { arcs, order }
}

/// # Panics
///
/// Panics if `order` is zero.
fn circuit_edge_list_btree_set_collect(order: usize) -> EdgeListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return EdgeListBTreeSet {
            arcs: BTreeSet::new(),
            order,
        };
    }

    EdgeListBTreeSet {
        arcs: (0..order).map(|u| (u, (u + 1) % order)).collect(),
        order,
    }
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn adjacency_list_add_arc_empty(order: usize) {
    let _ = circuit_adjacency_list_add_arc_empty(order);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn adjacency_list_hash_set_collect(order: usize) {
    let _ = circuit_adjacency_list_hash_set_collect(order);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::circuit(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::circuit(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::circuit(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn edge_list(n: usize) {
    let _ = EdgeList::circuit(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn edge_list_btree_set_insert(n: usize) {
    let _ = circuit_edge_list_btree_set_insert(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn edge_list_btree_set_collect(n: usize) {
    let _ = circuit_edge_list_btree_set_collect(n);
}
