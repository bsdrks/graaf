//! Benchmarks of different implementations of `ErdosRenyi::erdos_renyi`.
use {
    graaf::{
        gen::prng::Xoshiro256StarStar,
        AddArc,
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        EdgeList,
        Empty,
        ErdosRenyi,
    },
    std::collections::{
        BTreeMap,
        BTreeSet,
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
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
fn erdos_renyi_adjacency_list_add_arc_empty(
    order: usize,
    p: f64,
    seed: u64,
) -> AdjacencyList {
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    let mut digraph = AdjacencyList::empty(order);
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (0..order).filter(|&v| u != v) {
            if rng.next_f64() < p {
                digraph.add_arc(u, v);
            }
        }
    }

    digraph
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
fn erdos_renyi_adjacency_list_btree_set_collect(
    order: usize,
    p: f64,
    seed: u64,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    let mut rng = Xoshiro256StarStar::new(seed);

    AdjacencyListBTreeSet {
        arcs: (0..order)
            .map(|u| {
                (0..u)
                    .chain((u + 1)..order)
                    .filter(|_| rng.next_f64() < p)
                    .collect()
            })
            .collect(),
    }
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
fn erdos_renyi_adjacency_map_add_arc_empty(
    order: usize,
    p: f64,
    seed: u64,
) -> AdjacencyMap {
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    let mut digraph = AdjacencyMap::empty(order);
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (0..order).filter(|&v| u != v) {
            if rng.next_f64() < p {
                digraph.add_arc(u, v);
            }
        }
    }

    digraph
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
fn erdos_renyi_adjacency_map_btree_set_collect(
    order: usize,
    p: f64,
    seed: u64,
) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    let mut rng = Xoshiro256StarStar::new(seed);

    AdjacencyMapBTreeSet {
        arcs: (0..order)
            .map(|u| {
                (
                    u,
                    (0..u)
                        .chain((u + 1)..order)
                        .filter(|_| rng.next_f64() < p)
                        .collect(),
                )
            })
            .collect(),
    }
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
fn erdos_renyi_adjacency_matrix_add_arc_empty(
    order: usize,
    p: f64,
    seed: u64,
) -> AdjacencyMatrix {
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    let mut digraph = AdjacencyMatrix::empty(order);
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (0..order).filter(|&v| u != v) {
            if rng.next_f64() < p {
                digraph.add_arc(u, v);
            }
        }
    }

    digraph
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
fn erdos_renyi_edge_list_add_arc_empty(
    order: usize,
    p: f64,
    seed: u64,
) -> EdgeList {
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    let mut digraph = EdgeList::empty(order);
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (0..order).filter(|&v| u != v) {
            if rng.next_f64() < p {
                digraph.add_arc(u, v);
            }
        }
    }

    digraph
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
fn erdos_renyi_edge_list_btree_set_collect_btree_set_collect(
    order: usize,
    p: f64,
    seed: u64,
) -> EdgeListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    let mut rng = Xoshiro256StarStar::new(seed);

    EdgeListBTreeSet {
        arcs: (0..order)
            .flat_map(|u| {
                (0..u)
                    .chain((u + 1)..order)
                    .filter(|_| rng.next_f64() < p)
                    .map(|v| (u, v))
                    .collect::<BTreeSet<_>>()
            })
            .collect(),
        order,
    }
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
fn erdos_renyi_edge_list_btree_set_collect_vec_collect(
    order: usize,
    p: f64,
    seed: u64,
) -> EdgeListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    let mut rng = Xoshiro256StarStar::new(seed);

    EdgeListBTreeSet {
        arcs: (0..order)
            .flat_map(|u| {
                (0..u)
                    .chain((u + 1)..order)
                    .filter(|_| rng.next_f64() < p)
                    .map(|v| (u, v))
                    .collect::<Vec<_>>()
            })
            .collect(),
        order,
    }
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::erdos_renyi(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_add_arc_empty(n: usize) {
    let _ = erdos_renyi_adjacency_list_add_arc_empty(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_btree_set_collect(n: usize) {
    let _ = erdos_renyi_adjacency_list_btree_set_collect(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::erdos_renyi(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map_add_arc_empty(n: usize) {
    let _ = erdos_renyi_adjacency_map_add_arc_empty(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map_btree_set_collect(n: usize) {
    let _ = erdos_renyi_adjacency_map_btree_set_collect(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::erdos_renyi(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_matrix_add_arc_empty(n: usize) {
    let _ = erdos_renyi_adjacency_matrix_add_arc_empty(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn edge_list(n: usize) {
    let _ = EdgeList::erdos_renyi(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn edge_list_add_arc_empty(n: usize) {
    let _ = erdos_renyi_edge_list_add_arc_empty(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn edge_list_btree_list_collect_btree_set_collect(n: usize) {
    let _ =
        erdos_renyi_edge_list_btree_set_collect_btree_set_collect(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn edge_list_btree_list_collect_vec_collect(n: usize) {
    let _ = erdos_renyi_edge_list_btree_set_collect_vec_collect(n, 0.5, 0);
}
