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
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
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
pub struct AdjacencyMapBTreeSet {
    pub arcs: BTreeMap<usize, BTreeSet<usize>>,
}

#[derive(Debug)]
pub struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

fn empty_adjacency_list_hash_set(order: usize) -> AdjacencyListHashSet {
    let arcs = vec![HashSet::new(); order];

    AdjacencyListHashSet { arcs }
}

fn empty_adjacency_map_from_vec(order: usize) -> AdjacencyMapBTreeSet {
    let digraph = AdjacencyMapBTreeSet {
        arcs: vec![BTreeSet::new(); order]
            .into_iter()
            .enumerate()
            .collect(),
    };

    assert!(
        !digraph.arcs.is_empty(),
        "a digraph has at least one vertex"
    );

    for (u, v) in &digraph.arcs {
        for v in v {
            assert_ne!(u, v, "u = {u} equals v = {v}");

            assert!(
                digraph.arcs.contains_key(v),
                "v = {v} isn't in the digraph"
            );
        }
    }

    digraph
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_hash_set(n: usize) {
    let _ = empty_adjacency_list_hash_set(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_from_vec(n: usize) {
    let _ = empty_adjacency_map_from_vec(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::empty(n);
}
