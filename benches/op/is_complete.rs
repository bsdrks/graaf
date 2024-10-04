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
        HasEdge,
        IsComplete,
        Order,
    },
};

fn main() {
    divan::main();
}

fn is_complete_has_edge_order<D>(digraph: &D) -> bool
where
    D: Order + HasEdge,
{
    let order = digraph.order();

    for u in 0..order {
        for v in (u + 1)..order {
            if !digraph.has_edge(u, v) {
                return false;
            }
        }
    }

    true
}

fn is_complete_iter_1<D>(digraph: &D) -> bool
where
    D: Order + HasEdge,
{
    let order = digraph.order();

    (0..order).all(|u| (u + 1..order).all(|v| digraph.has_edge(u, v)))
}

fn is_complete_iter_2<D>(digraph: &D) -> bool
where
    D: Order + HasEdge,
{
    let order = digraph.order();

    (0..order).all(|u| ((u + 1)..order).all(|v| digraph.has_edge(u, v)))
}

fn is_complete_iter_3<D>(digraph: &D) -> bool
where
    D: Order + HasEdge,
{
    let order = digraph.order();
    let vertices = 0..order;

    (vertices.clone()).all(|u| {
        vertices
            .clone()
            .filter(|&v| v != u)
            .all(|v| u == v || digraph.has_edge(u, v))
    })
}

fn is_complete_eq_complete<D>(digraph: &D) -> bool
where
    D: Complete + Order + PartialEq,
{
    digraph == &D::complete(digraph.order())
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_complete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_iter_1(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_1(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_iter_2(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_2(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_iter_3(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_3(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_has_edge_order(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_has_edge_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_eq_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_eq_complete(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_complete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_has_edge_order(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = is_complete_has_edge_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_iter_1(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_1(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_iter_2(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_2(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_iter_3(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_3(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_eq_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = is_complete_eq_complete(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_complete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_has_edge_order(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_complete_has_edge_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_iter_1(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_1(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_iter_2(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_2(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_iter_3(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_3(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_eq_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_complete_eq_complete(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_complete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_has_edge_order(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_has_edge_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_iter_1(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_1(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_iter_2(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_2(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_iter_3(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_3(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_eq_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_eq_complete(&digraph);
    });
}
