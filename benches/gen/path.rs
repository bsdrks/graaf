//! Benchmarks of different implementations of `Path::path`.
use {
    graaf::{
        AddArc,
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        EdgeList,
        Empty,
        Path,
    },
    std::{
        collections::{
            BTreeSet,
            HashSet,
        },
        iter::once,
    },
};

fn main() {
    divan::main();
}

#[allow(dead_code)]
#[derive(Debug)]
struct AdjacencyListBTreeSet {
    pub arcs: Vec<BTreeSet<usize>>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct AdjacencyMapBTreeSet {
    pub arcs: BTreeSet<(usize, BTreeSet<usize>)>,
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
fn path_adjacency_list_add_arc_empty(order: usize) -> AdjacencyList {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut digraph = AdjacencyList::empty(order);

    for u in 0..order - 1 {
        digraph.add_arc(u, u + 1);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn path_adjacency_list_btree_set_collect(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    AdjacencyListBTreeSet {
        arcs: (0..order - 1)
            .map(|u| BTreeSet::from([u + 1]))
            .chain(once(BTreeSet::new()))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn path_adjacency_list_hash_set_collect(order: usize) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    AdjacencyListHashSet {
        arcs: (0..order - 1)
            .map(|u| HashSet::from([u + 1]))
            .chain(once(HashSet::new()))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn path_adjacency_map_add_arc_empty(order: usize) -> AdjacencyMap {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut digraph = AdjacencyMap::empty(order);

    for u in 0..order - 1 {
        digraph.add_arc(u, u + 1);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn path_adjacency_map_collect(order: usize) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    AdjacencyMapBTreeSet {
        arcs: (0..order - 1)
            .map(|u| (u, BTreeSet::from([u + 1])))
            .chain(once((order - 1, BTreeSet::new())))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn path_edge_list_add_arc_empty(order: usize) -> EdgeList {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut digraph = EdgeList::empty(order);

    for u in 0..order - 1 {
        digraph.add_arc(u, u + 1);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn path_edge_list_btree_set_collect(order: usize) -> EdgeListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    EdgeListBTreeSet {
        arcs: (0..order - 1).map(|u| (u, u + 1)).collect(),
        order,
    }
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list(n: usize) {
    drop(AdjacencyList::path(n));
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_collect(n: usize) {
    drop(path_adjacency_list_btree_set_collect(n));
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_add_arc_empty(n: usize) {
    drop(path_adjacency_list_add_arc_empty(n));
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_hash_set_collect(n: usize) {
    drop(path_adjacency_list_hash_set_collect(n));
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map(n: usize) {
    drop(AdjacencyMap::path(n));
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_add_arc_empty(n: usize) {
    drop(path_adjacency_map_add_arc_empty(n));
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_collect(n: usize) {
    drop(path_adjacency_map_collect(n));
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_matrix(n: usize) {
    drop(AdjacencyMatrix::path(n));
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list(n: usize) {
    drop(EdgeList::path(n));
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_add_arc_empty(n: usize) {
    drop(path_edge_list_add_arc_empty(n));
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_btree_list_collect(n: usize) {
    drop(path_edge_list_btree_set_collect(n));
}
