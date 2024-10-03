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
        gen::prng::Xoshiro256StarStar,
        AddArc,
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        EdgeList,
        Empty,
        GrowingNetwork,
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
fn growing_network_adjacency_list_btree_set_collect(
    order: usize,
    seed: u64,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut rng = Xoshiro256StarStar::new(seed);

    AdjacencyListBTreeSet {
        arcs: once(BTreeSet::new())
            .chain((1..order).map(|u| {
                BTreeSet::from([usize::try_from(rng.next().unwrap())
                    .expect("conversion failed")
                    % u])
            }))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn growing_network_adjacency_list_btree_set_push(
    order: usize,
    seed: u64,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = Vec::with_capacity(order);
    let rng = Xoshiro256StarStar::new(seed);

    for (u, v) in (1..order).zip(rng) {
        arcs.push(BTreeSet::from([usize::try_from(v)
            .expect("conversion failed")
            % u]));
    }

    AdjacencyListBTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn growing_network_adjacency_list_btree_set_add_arc_empty(
    order: usize,
    seed: u64,
) -> AdjacencyList {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut digraph = AdjacencyList::empty(order);
    let rng = Xoshiro256StarStar::new(seed);

    for (u, v) in (1..order).zip(rng) {
        digraph.add_arc(u, usize::try_from(v).expect("conversion failed") % u);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn growing_network_adjacency_list_hash_set_collect(
    order: usize,
    seed: u64,
) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut rng = Xoshiro256StarStar::new(seed);

    AdjacencyListHashSet {
        arcs: once(HashSet::new())
            .chain((1..order).map(|u| {
                HashSet::from([usize::try_from(rng.next().unwrap())
                    .expect("conversion failed")
                    % u])
            }))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn growing_network_adjacency_list_hash_set_push(
    order: usize,
    seed: u64,
) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = Vec::with_capacity(order);
    let rng = Xoshiro256StarStar::new(seed);

    for (u, v) in (1..order).zip(rng) {
        arcs.push(HashSet::from([usize::try_from(v)
            .expect("conversion failed")
            % u]));
    }

    AdjacencyListHashSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn growing_network_adjacency_map_add_arc_empty(
    order: usize,
    seed: u64,
) -> AdjacencyMap {
    let mut digraph = AdjacencyMap::empty(order);
    let rng = Xoshiro256StarStar::new(seed);

    for (u, v) in (1..order).zip(rng) {
        digraph.add_arc(u, usize::try_from(v).expect("conversion failed") % u);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn growing_network_adjacency_map_collect(
    order: usize,
    seed: u64,
) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut rng = Xoshiro256StarStar::new(seed);

    AdjacencyMapBTreeSet {
        arcs: once((0, BTreeSet::new()))
            .chain((1..order).map(|u| {
                (
                    u,
                    BTreeSet::from([usize::try_from(rng.next().unwrap())
                        .expect("conversion failed")
                        % u]),
                )
            }))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn growing_network_edge_list_add_arc_empty(
    order: usize,
    seed: u64,
) -> EdgeList {
    let mut digraph = EdgeList::empty(order);
    let rng = Xoshiro256StarStar::new(seed);

    for (u, v) in (1..order).zip(rng) {
        digraph.add_arc(u, usize::try_from(v).expect("conversion failed") % u);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn growing_network_btree_set_edge_list_collect(
    order: usize,
    seed: u64,
) -> EdgeListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut rng = Xoshiro256StarStar::new(seed);

    EdgeListBTreeSet {
        arcs: (1..order)
            .map(|u| {
                (
                    u,
                    usize::try_from(rng.next().unwrap())
                        .expect("conversion failed")
                        % u,
                )
            })
            .collect(),
        order,
    }
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::growing_network(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_collect(n: usize) {
    let _ = growing_network_adjacency_list_btree_set_collect(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_push(n: usize) {
    let _ = growing_network_adjacency_list_btree_set_push(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_add_arc_empty(n: usize) {
    let _ = growing_network_adjacency_list_btree_set_add_arc_empty(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_hash_set_collect(n: usize) {
    let _ = growing_network_adjacency_list_hash_set_collect(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_hash_set_push(n: usize) {
    let _ = growing_network_adjacency_list_hash_set_push(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::growing_network(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::growing_network(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_add_arc_empty(n: usize) {
    let _ = growing_network_adjacency_map_add_arc_empty(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_btree_set_collect(n: usize) {
    let _ = growing_network_adjacency_map_collect(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list(n: usize) {
    let _ = EdgeList::growing_network(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_add_arc_empty(n: usize) {
    let _ = growing_network_edge_list_add_arc_empty(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_btree_set_collect(n: usize) {
    let _ = growing_network_btree_set_edge_list_collect(n, 0);
}
