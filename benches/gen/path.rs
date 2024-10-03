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
    pub arcs: BTreeSet<(usize, BTreeSet<usize>)>,
}

#[derive(Debug)]
pub struct EdgeListBTreeSet {
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
    let _ = AdjacencyList::path(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_collect(n: usize) {
    let _ = path_adjacency_list_btree_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_add_arc_empty(n: usize) {
    let _ = path_adjacency_list_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_hash_set_collect(n: usize) {
    let _ = path_adjacency_list_hash_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::path(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_add_arc_empty(n: usize) {
    let _ = path_adjacency_map_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_collect(n: usize) {
    let _ = path_adjacency_map_collect(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::path(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list(n: usize) {
    let _ = EdgeList::path(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_add_arc_empty(n: usize) {
    let _ = path_edge_list_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_btree_list_collect(n: usize) {
    let _ = path_edge_list_btree_set_collect(n);
}
