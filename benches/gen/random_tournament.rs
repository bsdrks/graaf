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
