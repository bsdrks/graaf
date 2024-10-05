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
        Outdegree,
    },
    std::collections::{
        BTreeMap,
        BTreeSet,
    },
};

fn main() {
    divan::main();
}

#[derive(Debug)]
pub struct AdjacencyListBTreeSet {
    arcs: Vec<BTreeSet<usize>>,
}

#[derive(Debug)]
pub struct AdjacencyListWeightedBTreeMap {
    arcs: Vec<BTreeMap<usize, usize>>,
}

#[derive(Debug)]
pub struct AdjacencyMapBTreeSet {
    arcs: Vec<BTreeSet<usize>>,
}

#[derive(Debug)]
pub struct EdgeListBTreeSet {
    arcs: BTreeSet<(usize, usize)>,
}

fn is_sink_outdegree<D>(digraph: &D, u: usize) -> bool
where
    D: Outdegree,
{
    digraph.outdegree(u) == 0
}

fn is_sink_adjacency_is_empty(
    digraph: &AdjacencyListBTreeSet,
    u: usize,
) -> bool {
    digraph.arcs[u].is_empty()
}

fn is_sink_adjacency_list_weighted_is_empty(
    digraph: &AdjacencyListWeightedBTreeMap,
    u: usize,
) -> bool {
    digraph.arcs[u].is_empty()
}

fn is_sink_adjacency_map_is_empty(
    digraph: &AdjacencyMapBTreeSet,
    u: usize,
) -> bool {
    digraph.arcs[u].is_empty()
}

fn is_sink_edge_list_all_ne(digraph: &EdgeListBTreeSet, u: usize) -> bool {
    digraph.arcs.iter().all(|(x, _)| *x != u)
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = digraph.is_sink(u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_outdegree(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_sink_outdegree(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_is_empty(bencher: Bencher<'_, '_>, order: usize) {
    let mut digraph = AdjacencyListBTreeSet {
        arcs: vec![BTreeSet::new(); order],
    };

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.05, 0).arcs() {
        let _ = digraph.arcs[u].insert(v);
    }

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_sink_adjacency_is_empty(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_weighted(bencher: Bencher<'_, '_>, order: usize) {
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.05, 0).arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        for u in 0..order {
            let _ = digraph.is_sink(u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_weighted_outdegree(bencher: Bencher<'_, '_>, order: usize) {
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.05, 0).arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_sink_outdegree(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_weighted_is_empty(bencher: Bencher<'_, '_>, order: usize) {
    let mut digraph = AdjacencyListWeightedBTreeMap {
        arcs: vec![BTreeMap::new(); order],
    };

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.05, 0).arcs() {
        let _ = digraph.arcs[u].insert(v, 1);
    }

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_sink_adjacency_list_weighted_is_empty(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = digraph.is_sink(u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map_outdegree(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_sink_outdegree(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map_is_empty(bencher: Bencher<'_, '_>, order: usize) {
    let mut digraph = AdjacencyMapBTreeSet {
        arcs: vec![BTreeSet::new(); order],
    };

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.05, 0).arcs() {
        let _ = digraph.arcs[u].insert(v);
    }

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_sink_adjacency_map_is_empty(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_matrix(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = digraph.is_sink(u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_matrix_outdegree(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_sink_outdegree(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = digraph.is_sink(u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_outdegree(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_sink_outdegree(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_btree_set_all_ne(bencher: Bencher<'_, '_>, order: usize) {
    let mut digraph = EdgeListBTreeSet {
        arcs: BTreeSet::new(),
    };

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.05, 0).arcs() {
        let _ = digraph.arcs.insert((u, v));
    }

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_sink_edge_list_all_ne(&digraph, u);
        }
    });
}
