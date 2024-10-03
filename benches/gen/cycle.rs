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
        Cycle,
        EdgeList,
        Empty,
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
fn cycle_adjacency_list_btree_set_collect(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyListBTreeSet {
            arcs: vec![BTreeSet::new()],
        };
    }

    AdjacencyListBTreeSet {
        arcs: (0..order)
            .map(|u| {
                let u = u + order;

                BTreeSet::from([(u - 1) % order, (u + 1) % order])
            })
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_adjacency_list_btree_set_add_arc_empty(
    order: usize,
) -> AdjacencyList {
    let mut digraph = AdjacencyList::empty(order);

    if order == 1 {
        return digraph;
    }

    for u in 0..order - 1 {
        let v = u + 1;

        digraph.add_arc(u, v);
        digraph.add_arc(v, u);
    }

    let u = order - 1;

    digraph.add_arc(u, 0);
    digraph.add_arc(0, u);

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_adjacency_map_btree_set_collect(
    order: usize,
) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyMapBTreeSet {
            arcs: BTreeMap::from([(0, BTreeSet::new())]),
        };
    }

    AdjacencyMapBTreeSet {
        arcs: (0..order)
            .map(|u| {
                (
                    u,
                    BTreeSet::from([(u + order - 1) % order, (u + 1) % order]),
                )
            })
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_adjacency_map_btree_set_add_arc_empty(order: usize) -> AdjacencyMap {
    let mut digraph = AdjacencyMap::empty(order);

    if order == 1 {
        return digraph;
    }

    for u in 0..order - 1 {
        let v = u + 1;

        digraph.add_arc(u, v);
        digraph.add_arc(v, u);
    }

    let u = order - 1;

    digraph.add_arc(u, 0);
    digraph.add_arc(0, u);

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_adjacency_list_hash_set_insert(order: usize) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");
    let mut arcs = vec![HashSet::new(); order];

    if order == 1 {
        return AdjacencyListHashSet { arcs };
    }

    for u in 0..order - 1 {
        let v = u + 1;

        let _ = arcs[u].insert(v);
        let _ = arcs[v].insert(u);
    }

    let u = order - 1;

    let _ = arcs[u].insert(0);
    let _ = arcs[0].insert(u);

    AdjacencyListHashSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_edge_list_add_arc_empty(order: usize) -> EdgeList {
    let mut digraph = EdgeList::empty(order);

    if order == 1 {
        return digraph;
    }

    for u in 0..order - 1 {
        let v = u + 1;

        digraph.add_arc(u, v);
        digraph.add_arc(v, u);
    }

    let u = order - 1;

    digraph.add_arc(u, 0);
    digraph.add_arc(0, u);

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_edge_list_btree_set_flat_map(order: usize) -> EdgeListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return EdgeListBTreeSet {
            arcs: BTreeSet::new(),
            order,
        };
    }

    EdgeListBTreeSet {
        arcs: (0..order)
            .flat_map(|u| {
                once((u, (u + order - 1) % order))
                    .chain(once((u, (u + 1) % order)))
            })
            .collect(),
        order,
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_edge_list_btree_set_flat_map_map(order: usize) -> EdgeListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return EdgeListBTreeSet {
            arcs: BTreeSet::new(),
            order,
        };
    }

    EdgeListBTreeSet {
        arcs: (0..order)
            .flat_map(|u| {
                once((u + order - 1) % order)
                    .chain(once((u + 1) % order))
                    .map(move |v| (u, v))
            })
            .collect(),
        order,
    }
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::cycle(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_collect(n: usize) {
    let _ = cycle_adjacency_list_btree_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_add_arc_empty(n: usize) {
    let _ = cycle_adjacency_list_btree_set_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_hash_set(n: usize) {
    let _ = cycle_adjacency_list_hash_set_insert(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::cycle(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::cycle(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_btree_set_collect(n: usize) {
    let _ = cycle_adjacency_map_btree_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_btree_set_add_arc_empty(n: usize) {
    let _ = cycle_adjacency_map_btree_set_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list(n: usize) {
    let _ = EdgeList::cycle(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_add_arc_empty(n: usize) {
    let _ = cycle_edge_list_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_btree_set_flat_map(n: usize) {
    let _ = cycle_edge_list_btree_set_flat_map(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_btree_set_flat_map_map(n: usize) {
    let _ = cycle_edge_list_btree_set_flat_map_map(n);
}
