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
    divan::Bencher,
    graaf::{
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        Complete,
        EdgeList,
        ErdosRenyi,
        Indegree,
        IndegreeSequence,
        IsRegular,
        Outdegree,
        OutdegreeSequence,
        Vertices,
    },
};

fn main() {
    divan::main();
}

fn is_regular_all_indegree_eq_outdegree<D>(digraph: &D) -> bool
where
    D: Indegree + Outdegree + Vertices,
{
    let mut vertices = digraph.vertices();
    let u = vertices.next().expect("a digraph has at least one vertex");
    let indegree = digraph.indegree(u);
    let outdegree = digraph.outdegree(u);

    indegree == outdegree
        && vertices.all(|u| {
            digraph.indegree(u) == indegree
                && digraph.outdegree(u) == outdegree
        })
}

fn is_regular_indegree_sequence_eq_outdegree_sequence<D>(digraph: &D) -> bool
where
    D: IndegreeSequence + OutdegreeSequence,
{
    digraph.indegree_sequence().eq(digraph.outdegree_sequence())
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = digraph.is_regular();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_all_indegree_eq_outdegree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = is_regular_all_indegree_eq_outdegree(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_indegree_sequence_eq_outdegree_sequence(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = is_regular_indegree_sequence_eq_outdegree_sequence(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_regular();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete_all_indegree_eq_outdegree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = is_regular_all_indegree_eq_outdegree(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete_indegree_sequence_eq_outdegree_sequence(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = is_regular_indegree_sequence_eq_outdegree_sequence(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = digraph.is_regular();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_all_indegree_eq_outdegree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = is_regular_all_indegree_eq_outdegree(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_indegree_sequence_eq_outdegree_sequence(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = is_regular_indegree_sequence_eq_outdegree_sequence(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_regular();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete_all_indegree_eq_outdegree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = is_regular_all_indegree_eq_outdegree(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete_indegree_sequence_eq_outdegree_sequence(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = is_regular_indegree_sequence_eq_outdegree_sequence(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = digraph.is_regular();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_all_indegree_eq_outdegree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = is_regular_all_indegree_eq_outdegree(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_indegree_sequence_eq_outdegree_sequence(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = is_regular_indegree_sequence_eq_outdegree_sequence(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_regular();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete_all_indegree_eq_outdegree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_regular_all_indegree_eq_outdegree(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete_indegree_sequence_eq_outdegree_sequence(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_regular_indegree_sequence_eq_outdegree_sequence(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = digraph.is_regular();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_all_indegree_eq_outdegree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = is_regular_all_indegree_eq_outdegree(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_indegree_sequence_eq_outdegree_sequence(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = is_regular_indegree_sequence_eq_outdegree_sequence(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_regular();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete_all_indegree_eq_outdegree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_regular_all_indegree_eq_outdegree(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete_indegree_sequence_eq_outdegree_sequence(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_regular_indegree_sequence_eq_outdegree_sequence(&digraph);
    });
}
