// Clippy lint groups
#![deny(clippy::all, clippy::cargo, clippy::pedantic, clippy::nursery)]
// Clippy restriction lints
#![deny(
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::impl_trait_in_params,
    clippy::missing_assert_message,
    clippy::multiple_inherent_impl,
    clippy::panic_in_result_fn,
    clippy::redundant_type_annotations,
    clippy::renamed_function_params,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::self_named_module_files,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unwrap_in_result
)]
// Rustc lint groups
#![deny(rust_2018_idioms)]
// Rustc lints
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    variant_size_differences
)]
// Rustdoc lints
#![deny(rustdoc::all)]
// Overwrites
#![allow(clippy::large_stack_frames)]

use {
    graaf::{
        AddArc,
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        EdgeList,
        Empty,
        Star,
    },
    std::{
        collections::{
            BTreeMap,
            BTreeSet,
            HashSet,
        },
        iter::once,
    },
};

fn main() {
    divan::main();
}

#[derive(Debug)]
pub struct AdjacencyListBTreeSet {
    pub arcs: Vec<BTreeSet<usize>>,
}

#[derive(Debug)]
pub struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

#[derive(Debug)]
pub struct AdjacencyMapBTreeSet {
    pub arcs: BTreeMap<usize, BTreeSet<usize>>,
}

#[derive(Debug)]
pub struct EdgeListBTreeSet {
    pub arcs: BTreeSet<(usize, usize)>,
    pub order: usize,
}

/// # Panics
///
/// Panics if `order` is zero.
fn star_adjacency_list_add_arc_empty(order: usize) -> AdjacencyList {
    let mut digraph = AdjacencyList::empty(order);

    for u in 1..order {
        digraph.add_arc(u, 0);
        digraph.add_arc(0, u);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn star_adjacency_list_btree_set_push(order: usize) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = Vec::with_capacity(order);

    arcs.push((1..order).collect());

    for _ in 1..order {
        arcs.push(BTreeSet::from([0]));
    }

    AdjacencyListBTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn star_adjacency_list_btree_set_insert(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = vec![BTreeSet::new(); order];

    for v in 1..order {
        let _ = arcs[0].insert(v);
        let _ = arcs[v].insert(0);
    }

    AdjacencyListBTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn star_adjacency_list_btree_set_collect(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    AdjacencyListBTreeSet {
        arcs: once((1..order).collect())
            .chain((1..order).map(|_| BTreeSet::from([0])))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn star_adjacency_list_hash_set_push(order: usize) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = Vec::with_capacity(order);

    arcs.push((1..order).collect());

    for _ in 1..order {
        arcs.push(HashSet::from([0]));
    }

    AdjacencyListHashSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn star_adjacency_list_hash_set_insert(order: usize) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = vec![HashSet::new(); order];

    for v in 1..order {
        let _ = arcs[0].insert(v);
        let _ = arcs[v].insert(0);
    }

    AdjacencyListHashSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn star_adjacency_list_hash_set_collect(order: usize) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    AdjacencyListHashSet {
        arcs: once((1..order).collect())
            .chain((1..order).map(|_| HashSet::from([0])))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn star_adjacency_map_add_arc_empty(order: usize) -> AdjacencyMap {
    let mut digraph = AdjacencyMap::empty(order);

    for u in 1..order {
        digraph.add_arc(u, 0);
        digraph.add_arc(0, u);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn star_adjacency_map_btree_set_collect(order: usize) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    AdjacencyMapBTreeSet {
        arcs: once((0, (1..order).collect()))
            .chain((1..order).map(|u| (u, BTreeSet::from([0]))))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn star_adjacency_matrix_add_arc_empty(order: usize) -> AdjacencyMatrix {
    let mut digraph = AdjacencyMatrix::empty(order);

    for u in 1..order {
        digraph.add_arc(u, 0);
        digraph.add_arc(0, u);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn star_edge_list_add_arc_empty(order: usize) -> EdgeList {
    let mut digraph = EdgeList::empty(order);

    for u in 1..order {
        digraph.add_arc(u, 0);
        digraph.add_arc(0, u);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn star_edge_list_btree_set_collect(order: usize) -> EdgeListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    EdgeListBTreeSet {
        arcs: (1..order)
            .map(|v| (0, v))
            .chain((1..order).map(|u| (u, 0)))
            .collect(),
        order,
    }
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::star(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_add_arc_empty(n: usize) {
    let _ = star_adjacency_list_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_push(n: usize) {
    let _ = star_adjacency_list_btree_set_push(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_insert(n: usize) {
    let _ = star_adjacency_list_btree_set_insert(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_collect(n: usize) {
    let _ = star_adjacency_list_btree_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_hash_set_push(n: usize) {
    let _ = star_adjacency_list_hash_set_push(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_hash_set_insert(n: usize) {
    let _ = star_adjacency_list_hash_set_insert(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_hash_set_collect(n: usize) {
    let _ = star_adjacency_list_hash_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::star(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_add_arc_empty(n: usize) {
    let _ = star_adjacency_map_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_btree_set_collect(n: usize) {
    let _ = star_adjacency_map_btree_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::star(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_matrix_add_arc_empty(n: usize) {
    let _ = star_adjacency_matrix_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list(n: usize) {
    let _ = EdgeList::star(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_add_arc_empty(n: usize) {
    let _ = star_edge_list_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_btree_set_collect(n: usize) {
    let _ = star_edge_list_btree_set_collect(n);
}
