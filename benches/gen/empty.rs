//! Benchmarks of different implementations of `Empty::empty`.
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

#[derive(Debug)]
struct AdjacencyMapBTreeSet {
    pub arcs: BTreeMap<usize, BTreeSet<usize>>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct AdjacencyListHashSet {
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

    for (u, v) in &digraph.arcs {
        for v in v {
            assert_ne!(u, v, "u = {u} equals v = {v}");

            assert!(
                digraph.arcs.contains_key(v),
                "v = {v} isn't in the digraph"
            );
        }
    }

    digraph
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_hash_set(n: usize) {
    let _ = empty_adjacency_list_hash_set(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_from_vec(n: usize) {
    let _ = empty_adjacency_map_from_vec(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::empty(n);
}
