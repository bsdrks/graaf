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
        AddArc,
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        Arcs,
        Complement,
        EdgeList,
        Empty,
        ErdosRenyi,
        HasArc,
        Order,
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
    pub arcs: Vec<BTreeSet<usize>>,
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

fn complement_adjacency_list_add_arc_empty_has_arc_order(
    digraph: &AdjacencyList,
) -> AdjacencyList {
    let order = digraph.order();
    let mut digraph = AdjacencyList::empty(order);

    for u in 0..order {
        for v in u + 1..order {
            if !digraph.has_arc(u, v) {
                digraph.add_arc(u, v);
            }

            if !digraph.has_arc(v, u) {
                digraph.add_arc(v, u);
            }
        }
    }

    digraph
}

fn complement_adjacency_list_btree_set_collect(
    digraph: &AdjacencyListBTreeSet,
) -> AdjacencyListBTreeSet {
    let order = digraph.arcs.len();
    let out_neighbors = (0..order).collect::<BTreeSet<_>>();

    AdjacencyListBTreeSet {
        arcs: digraph
            .arcs
            .iter()
            .map(|arcs| {
                out_neighbors.clone().difference(arcs).copied().collect()
            })
            .collect(),
    }
}

fn complement_adjacency_map_add_arc_empty_has_arc_order(
    digraph: &AdjacencyMap,
) -> AdjacencyMap {
    let order = digraph.order();
    let mut digraph = AdjacencyMap::empty(order);

    for u in 0..order {
        for v in u + 1..order {
            if !digraph.has_arc(u, v) {
                digraph.add_arc(u, v);
            }

            if !digraph.has_arc(v, u) {
                digraph.add_arc(v, u);
            }
        }
    }

    digraph
}

fn complement_adjacency_map_btree_set_collect(
    digraph: &AdjacencyMapBTreeSet,
) -> AdjacencyMapBTreeSet {
    let order = digraph.arcs.len();
    let out_neighbors = (0..order).collect::<BTreeSet<_>>();

    AdjacencyMapBTreeSet {
        arcs: digraph
            .arcs
            .iter()
            .map(|(u, arcs)| {
                (*u, out_neighbors.difference(arcs).copied().collect())
            })
            .collect(),
    }
}

fn complement_adjacency_matrix_add_arc_empty_has_arc_order(
    digraph: &AdjacencyMatrix,
) -> AdjacencyMatrix {
    let order = digraph.order();
    let mut digraph = AdjacencyMatrix::empty(order);

    for u in 0..order {
        for v in u + 1..order {
            if !digraph.has_arc(u, v) {
                digraph.add_arc(u, v);
            }

            if !digraph.has_arc(v, u) {
                digraph.add_arc(v, u);
            }
        }
    }

    digraph
}

fn complement_edge_list_add_arc_empty_has_arc_order(
    digraph: &EdgeList,
) -> EdgeList {
    let order = digraph.order();
    let mut digraph = EdgeList::empty(order);

    for u in 0..order {
        for v in u + 1..order {
            if !digraph.has_arc(u, v) {
                digraph.add_arc(u, v);
            }

            if !digraph.has_arc(v, u) {
                digraph.add_arc(v, u);
            }
        }
    }

    digraph
}

fn complement_edge_list_btree_set_collect(
    digraph: &EdgeListBTreeSet,
) -> EdgeListBTreeSet {
    let order = digraph.order;

    EdgeListBTreeSet {
        arcs: (0..order)
            .flat_map(|u| (0..u).chain(u + 1..order).map(move |v| (u, v)))
            .collect::<BTreeSet<(usize, usize)>>()
            .difference(&digraph.arcs)
            .copied()
            .collect(),
        order,
    }
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(bencher: Bencher<'_, '_>, order: usize) {
    let erdos_renyi = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = erdos_renyi.complement();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_add_arc_empty_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let erdos_renyi = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = complement_adjacency_list_add_arc_empty_has_arc_order(
            &erdos_renyi,
        );
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_set_collect(bencher: Bencher<'_, '_>, order: usize) {
    let mut arcs = vec![BTreeSet::new(); order];

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.5, 0).arcs() {
        let _ = arcs[u].insert(v);
    }

    let erdos_renyi = AdjacencyListBTreeSet { arcs };

    bencher.bench(|| {
        let _ = complement_adjacency_list_btree_set_collect(&erdos_renyi);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map(bencher: Bencher<'_, '_>, order: usize) {
    let erdos_renyi = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = erdos_renyi.complement();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_add_arc_empty_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let erdos_renyi = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ =
            complement_adjacency_map_add_arc_empty_has_arc_order(&erdos_renyi);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_btree_set_collect(bencher: Bencher<'_, '_>, order: usize) {
    let mut arcs = BTreeMap::new();

    for (u, v) in AdjacencyMap::erdos_renyi(order, 0.5, 0).arcs() {
        let _ = arcs.entry(u).or_insert_with(BTreeSet::new).insert(v);
    }

    let erdos_renyi = AdjacencyMapBTreeSet { arcs };

    bencher.bench(|| {
        let _ = complement_adjacency_map_btree_set_collect(&erdos_renyi);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(bencher: Bencher<'_, '_>, order: usize) {
    let erdos_renyi = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = erdos_renyi.complement();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_add_arc_empty_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let erdos_renyi = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = complement_adjacency_matrix_add_arc_empty_has_arc_order(
            &erdos_renyi,
        );
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list(bencher: Bencher<'_, '_>, order: usize) {
    let erdos_renyi = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = erdos_renyi.complement();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_add_arc_empty_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let erdos_renyi = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = complement_edge_list_add_arc_empty_has_arc_order(&erdos_renyi);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_btree_set_collect(bencher: Bencher<'_, '_>, order: usize) {
    let erdos_renyi = EdgeListBTreeSet {
        arcs: EdgeList::erdos_renyi(order, 0.5, 0).arcs().collect(),
        order,
    };

    bencher.bench(|| {
        let _ = complement_edge_list_btree_set_collect(&erdos_renyi);
    });
}