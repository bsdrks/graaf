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
        AdjacencyListWeighted,
        AdjacencyMap,
        AdjacencyMatrix,
        Arcs,
        Converse,
        EdgeList,
        Empty,
        ErdosRenyi,
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
pub struct AdjacencyListWeightedBTreeMap<W> {
    pub arcs: Vec<BTreeMap<usize, W>>,
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

fn converse_adjacency_list_add_arc_empty_has_arc_order(
    digraph: &AdjacencyList,
) -> AdjacencyList {
    let order = digraph.order();
    let mut converse = AdjacencyList::empty(order);

    for (u, v) in digraph.arcs() {
        converse.add_arc(v, u);
    }

    converse
}

fn converse_adjacency_list_btree_set_for_for(
    digraph: &AdjacencyListBTreeSet,
) -> AdjacencyListBTreeSet {
    let mut arcs = vec![BTreeSet::new(); digraph.arcs.len()];

    for (u, v) in digraph.arcs.iter().enumerate() {
        for v in v {
            let _ = arcs[*v].insert(u);
        }
    }

    AdjacencyListBTreeSet { arcs }
}

fn converse_adjacency_list_btree_set_fold(
    digraph: &AdjacencyListBTreeSet,
) -> AdjacencyListBTreeSet {
    assert!(
        !digraph.arcs.is_empty(),
        "a digraph has at least one vertex"
    );

    AdjacencyListBTreeSet {
        arcs: digraph.arcs.iter().enumerate().fold(
            vec![BTreeSet::new(); digraph.arcs.len()],
            |mut arcs, (u, v)| {
                for v in v {
                    let _ = arcs[*v].insert(u);
                }

                arcs
            },
        ),
    }
}

fn converse_adjacency_list_weighted_btree_map_for_for(
    digraph: &AdjacencyListWeightedBTreeMap<usize>,
) -> AdjacencyListWeightedBTreeMap<usize> {
    let order = digraph.arcs.len();

    let mut converse = AdjacencyListWeightedBTreeMap {
        arcs: vec![BTreeMap::new(); order],
    };

    for (u, v) in digraph.arcs.iter().enumerate() {
        for (v, w) in v {
            let _ = converse.arcs[*v].insert(u, *w);
        }
    }

    converse
}

fn converse_adjacency_list_weighted_btree_map_fold<W>(
    digraph: &AdjacencyListWeightedBTreeMap<W>,
) -> AdjacencyListWeightedBTreeMap<W>
where
    W: Clone + Copy,
{
    let order = digraph.arcs.len();

    let arcs = digraph.arcs.iter().enumerate().fold(
        vec![BTreeMap::new(); order],
        |mut arcs, (u, v)| {
            for (v, w) in v {
                let _ = arcs[*v].insert(u, *w);
            }

            arcs
        },
    );

    AdjacencyListWeightedBTreeMap { arcs }
}

fn converse_adjacency_map_add_arc_empty_has_arc_order(
    digraph: &AdjacencyMap,
) -> AdjacencyMap {
    let order = digraph.order();
    let mut converse = AdjacencyMap::empty(order);

    for (u, v) in digraph.arcs() {
        converse.add_arc(v, u);
    }

    converse
}

fn converse_adjacency_map_btree_set_for_for(
    digraph: &AdjacencyMapBTreeSet,
) -> AdjacencyMapBTreeSet {
    let mut arcs = digraph
        .arcs
        .keys()
        .map(|u| (*u, BTreeSet::new()))
        .collect::<BTreeMap<usize, BTreeSet<usize>>>();

    for (u, v) in &digraph.arcs {
        for v in v {
            let _ = arcs.get_mut(v).unwrap().insert(*u);
        }
    }

    AdjacencyMapBTreeSet { arcs }
}

fn converse_adjacency_matrix_add_arc_empty_has_arc_order(
    digraph: &AdjacencyMatrix,
) -> AdjacencyMatrix {
    let mut converse = AdjacencyMatrix::empty(digraph.order());

    for (u, v) in digraph.arcs() {
        converse.add_arc(v, u);
    }

    converse
}

fn converse_edge_list_add_arc_empty_has_arc_order(
    digraph: &EdgeList,
) -> EdgeList {
    let mut converse = EdgeList::empty(digraph.order());

    for (u, v) in digraph.arcs() {
        converse.add_arc(v, u);
    }

    converse
}

fn converse_edge_list_btree_set_collect(
    digraph: &EdgeListBTreeSet,
) -> EdgeListBTreeSet {
    EdgeListBTreeSet {
        arcs: digraph.arcs.iter().map(|&(u, v)| (v, u)).collect(),
        order: digraph.order,
    }
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(bencher: Bencher<'_, '_>, order: usize) {
    let erdos_renyi = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = erdos_renyi.converse();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_add_arc_empty_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let erdos_renyi = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ =
            converse_adjacency_list_add_arc_empty_has_arc_order(&erdos_renyi);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_set_for_for(bencher: Bencher<'_, '_>, order: usize) {
    let mut arcs = vec![BTreeSet::new(); order];

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.5, 0).arcs() {
        let _ = arcs[v].insert(u);
    }

    let erdos_renyi = AdjacencyListBTreeSet { arcs };

    bencher.bench(|| {
        let _ = converse_adjacency_list_btree_set_for_for(&erdos_renyi);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_set_fold(bencher: Bencher<'_, '_>, order: usize) {
    let erdos_renyi = AdjacencyList::erdos_renyi(order, 0.5, 0);

    let erdos_renyi = AdjacencyListBTreeSet {
        arcs: erdos_renyi.arcs().fold(
            vec![BTreeSet::new(); order],
            |mut arcs, (u, v)| {
                let _ = arcs[v].insert(u);

                arcs
            },
        ),
    };

    bencher.bench(|| {
        let _ = converse_adjacency_list_btree_set_fold(&erdos_renyi);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted(bencher: Bencher<'_, '_>, order: usize) {
    let erdos_renyi = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let erdos_renyi = AdjacencyListWeighted::<usize>::from(erdos_renyi);

    bencher.bench(|| {
        let _ = erdos_renyi.converse();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_btree_map_for_for(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let erdos_renyi = AdjacencyList::erdos_renyi(order, 0.5, 0);

    let erdos_renyi = AdjacencyListWeightedBTreeMap::<usize> {
        arcs: erdos_renyi.arcs().fold(
            vec![BTreeMap::new(); order],
            |mut arcs, (u, v)| {
                let _ = arcs[v].insert(u, 0);

                arcs
            },
        ),
    };

    bencher.bench(|| {
        let _ =
            converse_adjacency_list_weighted_btree_map_for_for(&erdos_renyi);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_btree_map_fold(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let erdos_renyi = AdjacencyList::erdos_renyi(order, 0.5, 0);

    let erdos_renyi = AdjacencyListWeightedBTreeMap::<usize> {
        arcs: erdos_renyi.arcs().fold(
            vec![BTreeMap::new(); order],
            |mut arcs, (u, v)| {
                let _ = arcs[v].insert(u, 0);

                arcs
            },
        ),
    };

    bencher.bench(|| {
        let _ = converse_adjacency_list_weighted_btree_map_fold(&erdos_renyi);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map(bencher: Bencher<'_, '_>, order: usize) {
    let erdos_renyi = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = erdos_renyi.converse();
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
            converse_adjacency_map_add_arc_empty_has_arc_order(&erdos_renyi);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_btree_set_for_for(bencher: Bencher<'_, '_>, order: usize) {
    let mut arcs = BTreeMap::<usize, BTreeSet<usize>>::new();

    for (u, v) in AdjacencyMap::erdos_renyi(order, 0.5, 0).arcs() {
        let _ = arcs.entry(v).or_default().insert(u);
    }

    let erdos_renyi = AdjacencyMapBTreeSet { arcs };

    bencher.bench(|| {
        let _ = converse_adjacency_map_btree_set_for_for(&erdos_renyi);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(bencher: Bencher<'_, '_>, order: usize) {
    let erdos_renyi = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = erdos_renyi.converse();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_add_arc_empty_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let erdos_renyi = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = converse_adjacency_matrix_add_arc_empty_has_arc_order(
            &erdos_renyi,
        );
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list(bencher: Bencher<'_, '_>, order: usize) {
    let erdos_renyi = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = erdos_renyi.converse();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_add_arc_empty_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let erdos_renyi = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = converse_edge_list_add_arc_empty_has_arc_order(&erdos_renyi);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_btree_set_collect(bencher: Bencher<'_, '_>, order: usize) {
    let mut arcs = BTreeSet::new();

    for (u, v) in EdgeList::erdos_renyi(order, 0.5, 0).arcs() {
        let _ = arcs.insert((v, u));
    }

    let erdos_renyi = EdgeListBTreeSet { arcs, order };

    bencher.bench(|| {
        let _ = converse_edge_list_btree_set_collect(&erdos_renyi);
    });
}