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
        EdgeList,
        Empty,
        ErdosRenyi,
        InNeighbors,
    },
    std::collections::{
        BTreeMap,
        BTreeSet,
    },
};

fn main() {
    divan::main();
}

struct AdjacencyListContains {
    arcs: Vec<BTreeSet<usize>>,
}

struct AdjacencyListWeightedContains {
    arcs: Vec<BTreeMap<usize, usize>>,
}

struct AdjacencyMapContains {
    arcs: BTreeMap<usize, BTreeSet<usize>>,
}

fn in_neighbors_arcs_filter_map_eq<D>(
    digraph: &D,
    v: usize,
) -> impl Iterator<Item = usize> + '_
where
    D: Arcs,
{
    digraph
        .arcs()
        .filter_map(move |(x, y)| (v == y).then_some(x))
}

fn in_neighbors_adjacency_list_contains(
    digraph: &AdjacencyListContains,
    v: usize,
) -> impl Iterator<Item = usize> + '_ {
    digraph
        .arcs
        .iter()
        .enumerate()
        .filter_map(move |(x, set)| set.contains(&v).then_some(x))
}

fn in_neighbors_adjacency_list_weighted_contains(
    digraph: &AdjacencyListWeightedContains,
    v: usize,
) -> impl Iterator<Item = usize> + '_ {
    digraph
        .arcs
        .iter()
        .enumerate()
        .filter_map(move |(x, map)| map.contains_key(&v).then_some(x))
}

fn in_neighbors_adjacency_map_contains(
    digraph: &AdjacencyMapContains,
    v: usize,
) -> impl Iterator<Item = usize> + '_ {
    digraph
        .arcs
        .iter()
        .filter_map(move |(x, set)| set.contains(&v).then_some(*x))
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        for v in 0..order {
            let _ = digraph.in_neighbors(v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_arcs_filter_map_eq(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_arcs_filter_map_eq(&digraph, v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_contains(bencher: Bencher<'_, '_>, order: usize) {
    let mut digraph = AdjacencyListContains {
        arcs: vec![BTreeSet::new(); order],
    };

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.5, 0).arcs() {
        let _ = digraph.arcs[u].insert(v);
    }

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_adjacency_list_contains(&digraph, v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted(bencher: Bencher<'_, '_>, order: usize) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        for v in 0..order {
            let _ = digraph.in_neighbors(v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_arcs_filter_map_eq(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_arcs_filter_map_eq(&digraph, v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_contains(bencher: Bencher<'_, '_>, order: usize) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);

    let mut digraph = AdjacencyListWeightedContains {
        arcs: vec![BTreeMap::new(); order],
    };

    for (u, v) in unweighted.arcs() {
        let _ = digraph.arcs[u].insert(v, 1);
    }

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_adjacency_list_weighted_contains(&digraph, v)
                .count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        for v in 0..order {
            let _ = digraph.in_neighbors(v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_arcs_filter_map_eq(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_arcs_filter_map_eq(&digraph, v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_contains(bencher: Bencher<'_, '_>, order: usize) {
    let mut digraph = AdjacencyMapContains {
        arcs: BTreeMap::new(),
    };

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.5, 0).arcs() {
        let _ = digraph.arcs.entry(v).or_default().insert(u);
    }

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_adjacency_map_contains(&digraph, v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        for v in 0..order {
            let _ = digraph.in_neighbors(v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_arcs_filter_map_eq(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_arcs_filter_map_eq(&digraph, v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        for v in 0..order {
            let _ = digraph.in_neighbors(v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_arcs_filter_map_eq(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_arcs_filter_map_eq(&digraph, v).count();
        }
    });
}
