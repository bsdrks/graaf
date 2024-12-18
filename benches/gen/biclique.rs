//! Benchmarks of different implementations of `Biclique::biclique`.
use {
    graaf::{
        AddArc,
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        Biclique,
        EdgeList,
        Empty,
    },
    std::{
        collections::{
            BTreeMap,
            BTreeSet,
            HashSet,
        },
        iter::repeat,
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
    pub arcs: Vec<(usize, usize)>,
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_list_add_arc_empty(m: usize, n: usize) -> AdjacencyList {
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

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_list_btree_set_clone_from(
    m: usize,
    n: usize,
) -> AdjacencyListBTreeSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let clique_1 = (0..m).collect::<BTreeSet<_>>();
    let clique_2 = (m..order).collect::<BTreeSet<_>>();

    let mut digraph = AdjacencyListBTreeSet {
        arcs: vec![BTreeSet::new(); order],
    };

    for u in 0..m {
        digraph.arcs[u].clone_from(&clique_2);
    }

    for u in m..order {
        digraph.arcs[u].clone_from(&clique_1);
    }

    digraph
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_list_btree_set_collect(
    m: usize,
    n: usize,
) -> AdjacencyListBTreeSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let clique_1 = (0..m).collect::<BTreeSet<_>>();
    let clique_2 = (m..order).collect::<BTreeSet<_>>();

    AdjacencyListBTreeSet {
        arcs: (0..m)
            .map(|_| clique_2.clone())
            .chain((m..order).map(|_| clique_1.clone()))
            .collect(),
    }
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_list_btree_set_repeat(
    m: usize,
    n: usize,
) -> AdjacencyListBTreeSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let clique_1 = (0..m).collect::<BTreeSet<_>>();
    let clique_2 = (m..order).collect::<BTreeSet<_>>();

    AdjacencyListBTreeSet {
        arcs: repeat(clique_2)
            .take(m)
            .chain(repeat(clique_1).take(n))
            .collect(),
    }
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_list_hash_set_clone_from(
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

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_map_add_arc_empty(m: usize, n: usize) -> AdjacencyMap {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let mut digraph = AdjacencyMap::empty(order);

    for u in 0..m {
        for v in m..order {
            digraph.add_arc(u, v);
            digraph.add_arc(v, u);
        }
    }

    digraph
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_map_btree_set_clone_from(
    m: usize,
    n: usize,
) -> AdjacencyMapBTreeSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let clique_1 = (0..m).collect::<BTreeSet<_>>();
    let clique_2 = (m..order).collect::<BTreeSet<_>>();

    let mut arcs = BTreeMap::new();

    for u in 0..m {
        let _ = arcs.insert(u, clique_2.clone());
    }

    for u in m..order {
        let _ = arcs.insert(u, clique_1.clone());
    }

    AdjacencyMapBTreeSet { arcs }
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_map_btree_set_repeat(
    m: usize,
    n: usize,
) -> AdjacencyMapBTreeSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let clique_1 = (0..m).collect::<BTreeSet<_>>();
    let clique_2 = (m..order).collect::<BTreeSet<_>>();

    let arcs = repeat(clique_2)
        .take(m)
        .chain(repeat(clique_1).take(n))
        .enumerate()
        .collect();

    AdjacencyMapBTreeSet { arcs }
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_edge_list_add_arc_empty(m: usize, n: usize) -> EdgeList {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let mut digraph = EdgeList::empty(order);

    for u in 0..m {
        for v in m..order {
            digraph.add_arc(u, v);
            digraph.add_arc(v, u);
        }
    }

    digraph
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_edge_list_flat_map(m: usize, n: usize) -> EdgeListBTreeSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;

    EdgeListBTreeSet {
        arcs: (0..m)
            .flat_map(|u| (m..order).map(move |v| (u, v)))
            .chain((m..order).flat_map(|u| (0..m).map(move |v| (u, v))))
            .collect(),
    }
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_edge_list_flat_map_clone(m: usize, n: usize) -> EdgeListBTreeSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let clique_2 = m..order;

    EdgeListBTreeSet {
        arcs: (0..m)
            .flat_map(|u| clique_2.clone().map(move |v| (u, v)))
            .chain(clique_2.clone().flat_map(|u| (0..m).map(move |v| (u, v))))
            .collect(),
    }
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
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
    (100, 10000),
])]
fn adjacency_list_add_arc_empty((m, n): (usize, usize)) {
    let _ = biclique_adjacency_list_add_arc_empty(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_list_btree_set_clone_from((m, n): (usize, usize)) {
    let _ = biclique_adjacency_list_btree_set_clone_from(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_list_btree_set_collect((m, n): (usize, usize)) {
    let _ = biclique_adjacency_list_btree_set_collect(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_list_btree_set_repeat((m, n): (usize, usize)) {
    let _ = biclique_adjacency_list_btree_set_repeat(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_list_hash_set_clone_from((m, n): (usize, usize)) {
    let _ = biclique_adjacency_list_hash_set_clone_from(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_map((m, n): (usize, usize)) {
    let _ = AdjacencyMap::biclique(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_matrix((m, n): (usize, usize)) {
    let _ = AdjacencyMatrix::biclique(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_map_add_arc_empty((m, n): (usize, usize)) {
    let _ = biclique_adjacency_map_add_arc_empty(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_map_btree_set_clone_from((m, n): (usize, usize)) {
    let _ = biclique_adjacency_map_btree_set_clone_from(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_map_btree_set_repeat((m, n): (usize, usize)) {
    let _ = biclique_adjacency_map_btree_set_repeat(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn edge_list((m, n): (usize, usize)) {
    let _ = EdgeList::biclique(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn edge_list_add_arc_empty((m, n): (usize, usize)) {
    let _ = biclique_edge_list_add_arc_empty(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn edge_list_flat_map((m, n): (usize, usize)) {
    let _ = biclique_edge_list_flat_map(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn edge_list_flat_map_clone((m, n): (usize, usize)) {
    let _ = biclique_edge_list_flat_map_clone(m, n);
}
