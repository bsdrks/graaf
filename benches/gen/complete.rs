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

#[derive(Debug)]
pub struct AdjacencyListBTreeSet {
    pub arcs: Vec<BTreeSet<usize>>,
}

#[derive(Debug)]
pub struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

#[derive(Debug)]
pub struct AdjacencyMapTreeSet {
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
            let _ = arcs[u].insert(v);
            let _ = arcs[v].insert(u);
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
        let _ = neighbors.remove(&u);
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
            let _ = arcs[u].insert(v);
            let _ = arcs[v].insert(u);
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
