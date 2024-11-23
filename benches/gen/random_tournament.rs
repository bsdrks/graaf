//! Benchmarks of different implementations of
//! `RandomTournament::random_tournament`.
use {
    graaf::{
        gen::prng::Xoshiro256StarStar,
        AddArc,
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        EdgeList,
        Empty,
        RandomTournament,
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
struct AdjacencyListBTreeSet {
    pub arcs: Vec<BTreeSet<usize>>,
}

#[derive(Debug)]
struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct AdjacencyMapBTreeSet {
    pub arcs: BTreeMap<usize, BTreeSet<usize>>,
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
fn random_tournament_adjacency_list_add_arc_empty(
    order: usize,
    seed: u64,
) -> AdjacencyList {
    let mut digraph = AdjacencyList::empty(order);
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (u + 1)..order {
            if rng.next_bool() {
                digraph.add_arc(u, v);
            } else {
                digraph.add_arc(v, u);
            }
        }
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn random_tournament_adjacency_list_btree_set_insert(
    order: usize,
    seed: u64,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut digraph = AdjacencyListBTreeSet {
        arcs: vec![BTreeSet::new(); order],
    };

    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (u + 1)..order {
            if rng.next_bool() {
                let _ = digraph.arcs[u].insert(v);
            } else {
                let _ = digraph.arcs[v].insert(u);
            }
        }
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn random_tournament_adjacency_list_hash_set_insert(
    order: usize,
    seed: u64,
) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut digraph = AdjacencyListHashSet {
        arcs: vec![HashSet::new(); order],
    };

    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (u + 1)..order {
            if rng.next_bool() {
                let _ = digraph.arcs[u].insert(v);
            } else {
                let _ = digraph.arcs[v].insert(u);
            }
        }
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn random_tournament_adjacency_map_add_arc_empty(
    order: usize,
    seed: u64,
) -> AdjacencyMap {
    let mut digraph = AdjacencyMap::empty(order);
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (u + 1)..order {
            if rng.next_bool() {
                digraph.add_arc(u, v);
            } else {
                digraph.add_arc(v, u);
            }
        }
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn random_tournament_adjacency_matrix_add_arc_empty(
    order: usize,
    seed: u64,
) -> AdjacencyMatrix {
    let mut digraph = AdjacencyMatrix::empty(order);
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (u + 1)..order {
            if rng.next_bool() {
                digraph.add_arc(u, v);
            } else {
                digraph.add_arc(v, u);
            }
        }
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn random_tournament_edge_list_add_arc_empty(
    order: usize,
    seed: u64,
) -> EdgeList {
    let mut digraph = EdgeList::empty(order);
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (u + 1)..order {
            if rng.next_bool() {
                digraph.add_arc(u, v);
            } else {
                digraph.add_arc(v, u);
            }
        }
    }

    digraph
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::random_tournament(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_add_arc_empty(n: usize) {
    let _ = random_tournament_adjacency_list_add_arc_empty(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_set_insert(n: usize) {
    let _ = random_tournament_adjacency_list_btree_set_insert(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_hash_set_insert(n: usize) {
    let _ = random_tournament_adjacency_list_hash_set_insert(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::random_tournament(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_add_arc_empty(n: usize) {
    let _ = random_tournament_adjacency_matrix_add_arc_empty(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::random_tournament(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_add_arc_empty(n: usize) {
    let _ = random_tournament_adjacency_map_add_arc_empty(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list(n: usize) {
    let _ = EdgeList::random_tournament(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_add_arc_empty(n: usize) {
    let _ = random_tournament_edge_list_add_arc_empty(n, 0);
}
