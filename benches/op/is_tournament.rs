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
        AddArcWeighted,
        AdjacencyList,
        AdjacencyListWeighted,
        AdjacencyMap,
        AdjacencyMatrix,
        Arcs,
        Degree,
        EdgeList,
        Empty,
        ErdosRenyi,
        HasArc,
        IsTournament,
        Order,
        RandomTournament,
        Size,
    },
};

fn main() {
    divan::main();
}

fn is_tournament_has_arc_order<D>(digraph: &D) -> bool
where
    D: HasArc + Order,
{
    let order = digraph.order();

    for u in 0..order {
        for v in (u + 1)..order {
            if usize::from(digraph.has_arc(u, v))
                + usize::from(digraph.has_arc(v, u))
                != 1
            {
                return false;
            }
        }
    }

    true
}

fn is_tournament_has_arc_order_size<D>(digraph: &D) -> bool
where
    D: HasArc + Order + Size,
{
    let order = digraph.order();

    if digraph.size() != order * (order - 1) / 2 {
        return false;
    }

    (0..order).all(|u| {
        (u + 1..order).all(|v| digraph.has_arc(u, v) ^ digraph.has_arc(v, u))
    })
}

fn is_tournament_degree_order_size<D>(digraph: &D) -> bool
where
    D: Degree + Order + Size,
{
    let order = digraph.order();

    if digraph.size() != order * (order - 1) / 2 {
        return false;
    }

    let degree = order - 1;

    (0..order).all(|u| digraph.degree(u) == degree)
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let graph = AdjacencyList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = graph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_random_tournament_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_random_tournament_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_random_tournament_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_random_tournament(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::random_tournament(order, 0);
    let mut graph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        graph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        let _ = graph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_random_tournament_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::random_tournament(order, 0);
    let mut graph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        graph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_random_tournament_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::random_tournament(order, 0);
    let mut graph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        graph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_random_tournament_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::random_tournament(order, 0);
    let mut graph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        graph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let graph = AdjacencyMap::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = graph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_random_tournament_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyMap::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_random_tournament_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyMap::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_random_tournament_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyMap::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let graph = AdjacencyMatrix::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = graph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_random_tournament_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyMatrix::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_random_tournament_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyMatrix::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_random_tournament_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyMatrix::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let graph = EdgeList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = graph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_random_tournament_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = EdgeList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_random_tournament_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = EdgeList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_random_tournament_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = EdgeList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let graph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = graph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_erdos_renyi(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut graph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        graph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        let _ = graph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_erdos_renyi_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut graph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        graph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_erdos_renyi_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut graph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        graph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_erdos_renyi_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut graph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        graph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let graph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = graph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let graph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = graph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let graph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = graph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&graph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let graph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&graph);
    });
}
