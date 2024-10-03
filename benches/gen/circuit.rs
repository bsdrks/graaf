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

#[derive(Debug)]
pub struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

#[derive(Debug)]
pub struct EdgeListBTreeSet {
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
