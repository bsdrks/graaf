//! Benchmark the `Empty::empty` implementation for different types.

use {
    graaf::{
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        Empty,
    },
    std::collections::{
        BTreeMap,
        BTreeSet,
        HashSet,
    },
};

fn main() {
    divan::main();
}

pub struct AdjacencyMapBTreeSet {
    pub arcs: BTreeMap<usize, BTreeSet<usize>>,
}

pub struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

fn empty_adjacency_list_hash_set(order: usize) -> AdjacencyListHashSet {
    let arcs = vec![HashSet::new(); order];

    AdjacencyListHashSet { arcs }
}

fn empty_adjacency_map_from_vec(order: usize) -> AdjacencyMapBTreeSet {
    let digraph = AdjacencyMapBTreeSet {
        arcs: vec![BTreeSet::new(); order]
            .into_iter()
            .enumerate()
            .collect(),
    };

    assert!(
        !digraph.arcs.is_empty(),
        "a digraph has at least one vertex"
    );

    for (u, vv) in digraph.arcs.iter() {
        for v in vv {
            assert_ne!(u, v, "u = {u} equals v = {v}");

            assert!(
                digraph.arcs.contains_key(v),
                "v = {v} isn't in the digraph"
            );
        }
    }

    digraph
}

#[divan::bench(args = [10, 100, 1000, 10000, 100000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100000])]
fn adjacency_list_hash_set(n: usize) {
    let _ = empty_adjacency_list_hash_set(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100000])]
fn adjacency_map_from_vec(n: usize) {
    let _ = empty_adjacency_map_from_vec(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::empty(n);
}
