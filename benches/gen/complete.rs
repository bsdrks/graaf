//! Benchmark the `Complete::complete` implementation for different types.

use {
    graaf::{
        AddArc,
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        Complete,
        EdgeList,
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

pub struct AdjacencyListBTreeSet {
    pub arcs: Vec<BTreeSet<usize>>,
}

pub struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

pub struct AdjacencyMapTreeSet {
    pub arcs: BTreeMap<usize, BTreeSet<usize>>,
}

pub struct EdgeListBTreeSet {
    pub arcs: BTreeSet<(usize, usize)>,
    pub order: usize,
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_list_btree_set_push(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = Vec::with_capacity(order);

    for u in 0..order {
        arcs.push((0..u).chain((u + 1)..order).collect());
    }

    AdjacencyListBTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_list_btree_set_collect(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    AdjacencyListBTreeSet {
        arcs: (0..order)
            .map(|u| (0..u).chain((u + 1)..order).collect())
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_list_btree_set_insert(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = vec![BTreeSet::new(); order];

    for u in 0..order {
        for v in (u + 1)..order {
            arcs[u].insert(v);
            arcs[v].insert(u);
        }
    }

    AdjacencyListBTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_list_btree_set_remove(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let neighbors = (0..order).collect::<BTreeSet<usize>>();
    let mut arcs = vec![neighbors; order];

    for (u, neighbors) in arcs.iter_mut().enumerate().take(order) {
        neighbors.remove(&u);
    }

    AdjacencyListBTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_list_hash_set_push(
    order: usize,
) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = Vec::with_capacity(order);

    for u in 0..order {
        arcs.push((0..u).chain((u + 1)..order).collect());
    }

    AdjacencyListHashSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_list_hash_set_collect(
    order: usize,
) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    AdjacencyListHashSet {
        arcs: (0..order)
            .map(|u| (0..u).chain((u + 1)..order).collect())
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_list_hash_set_insert(
    order: usize,
) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = vec![HashSet::new(); order];

    for u in 0..order {
        for v in (u + 1)..order {
            arcs[u].insert(v);
            arcs[v].insert(u);
        }
    }

    AdjacencyListHashSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_map_add_arc_empty(order: usize) -> AdjacencyMap {
    let mut digraph = AdjacencyMap::empty(order);

    for u in 0..order {
        for v in (u + 1)..order {
            digraph.add_arc(u, v);
            digraph.add_arc(v, u);
        }
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_map_btree_set_collect(
    order: usize,
) -> AdjacencyMapTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    AdjacencyMapTreeSet {
        arcs: (0..order)
            .map(|u| (u, (0..u).chain((u + 1)..order).collect()))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_edge_list_add_arc_empty(order: usize) -> EdgeList {
    let mut digraph = EdgeList::empty(order);

    for u in 0..order {
        for v in (u + 1)..order {
            digraph.add_arc(u, v);
            digraph.add_arc(v, u);
        }
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_edge_btree_set_list_collect(order: usize) -> EdgeListBTreeSet {
    EdgeListBTreeSet {
        arcs: (0..order)
            .flat_map(|u| (0..u).chain((u + 1)..order).map(move |v| (u, v)))
            .collect(),
        order,
    }
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::complete(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_set_push(n: usize) {
    let _ = complete_adjacency_list_btree_set_push(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_set_collect(n: usize) {
    let _ = complete_adjacency_list_btree_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_set_insert(n: usize) {
    let _ = complete_adjacency_list_btree_set_insert(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_set_remove(n: usize) {
    let _ = complete_adjacency_list_btree_set_remove(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_hash_set_push(n: usize) {
    let _ = complete_adjacency_list_hash_set_push(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_hash_set_collect(n: usize) {
    let _ = complete_adjacency_list_hash_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_hash_set_insert(n: usize) {
    let _ = complete_adjacency_list_hash_set_insert(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::complete(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_add_arc_empty(n: usize) {
    let _ = complete_adjacency_map_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_btree_set_collect(n: usize) {
    let _ = complete_adjacency_map_btree_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::complete(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list(n: usize) {
    let _ = EdgeList::complete(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_add_arc_empty(n: usize) {
    let _ = complete_edge_list_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_btree_set_collect(n: usize) {
    let _ = complete_edge_btree_set_list_collect(n);
}
