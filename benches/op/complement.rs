//! Benchmarks of different implementations of `Complement::complement`.
use {
    divan::Bencher,
    graaf::{
        AddArc, AdjacencyList, AdjacencyMap, AdjacencyMatrix, Arcs,
        Complement, EdgeList, Empty, ErdosRenyi, HasArc, Order,
    },
    std::{
        cmp::Ordering,
        collections::{BTreeMap, BTreeSet},
    },
};

fn main() {
    divan::main();
}

#[derive(Debug)]
struct AdjacencyListBTreeSet {
    pub arcs: Vec<BTreeSet<usize>>,
}

#[derive(Debug)]
struct AdjacencyMapBTreeSet {
    pub arcs: BTreeMap<usize, BTreeSet<usize>>,
}

#[derive(Debug)]
struct EdgeListBTreeSet {
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

fn complement_adjacency_list_btree_set_unsafe(
    digraph: &AdjacencyListBTreeSet,
) -> AdjacencyListBTreeSet {
    let order = digraph.arcs.len();
    let full = (0..order).collect::<Vec<_>>();
    let full_ptr = full.as_ptr();
    let full_len = full.len();

    AdjacencyListBTreeSet {
        arcs: digraph
            .arcs
            .iter()
            .enumerate()
            .map(|(u, out_neighbors)| {
                let out_vec: Vec<usize> =
                    out_neighbors.iter().copied().collect();

                let out_len = out_vec.len();
                let out_ptr = out_vec.as_ptr();
                let mut diff = Vec::with_capacity(full_len);

                unsafe {
                    let mut i = 0;
                    let mut j = 0;

                    while i < full_len && j < out_len {
                        let a = *full_ptr.add(i);
                        let b = *out_ptr.add(j);

                        match a.cmp(&b) {
                            Ordering::Less => {
                                diff.push(a);
                                i += 1;
                            }
                            Ordering::Greater => {
                                j += 1;
                            }
                            Ordering::Equal => {
                                i += 1;
                                j += 1;
                            }
                        }
                    }

                    while i < full_len {
                        diff.push(*full_ptr.add(i));
                        i += 1;
                    }
                }

                diff.retain(|&x| x != u);
                diff.into_iter().collect::<BTreeSet<usize>>()
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

fn complement_adjacency_map_btree_set_clone_difference_collect(
    digraph: &AdjacencyMapBTreeSet,
) -> AdjacencyMapBTreeSet {
    let order = digraph.arcs.len();
    let out_neighbors = (0..order).collect::<BTreeSet<_>>();

    AdjacencyMapBTreeSet {
        arcs: digraph
            .arcs
            .iter()
            .map(|(u, arcs)| {
                (
                    *u,
                    out_neighbors.clone().difference(arcs).copied().collect(),
                )
            })
            .collect(),
    }
}

fn complement_adjacency_map_btree_set_difference_collect(
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

fn complement_adjacency_map_btree_set_filter_collect(
    digraph: &AdjacencyMapBTreeSet,
) -> AdjacencyMapBTreeSet {
    let order = digraph.arcs.len();

    AdjacencyMapBTreeSet {
        arcs: digraph
            .arcs
            .iter()
            .map(|(u, neighbors)| {
                let complement_neighbors = (0..order)
                    .filter(|v| *v != *u && !neighbors.contains(v))
                    .collect::<BTreeSet<_>>();

                (*u, complement_neighbors)
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
fn adjacency_list_btree_set_unsafe(bencher: Bencher<'_, '_>, order: usize) {
    let mut arcs = vec![BTreeSet::new(); order];

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.5, 0).arcs() {
        let _ = arcs[u].insert(v);
    }

    let erdos_renyi = AdjacencyListBTreeSet { arcs };

    bencher.bench(|| {
        let _ = complement_adjacency_list_btree_set_unsafe(&erdos_renyi);
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
fn adjacency_map_btree_set_clone_difference_collect(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut arcs = BTreeMap::new();

    for (u, v) in AdjacencyMap::erdos_renyi(order, 0.5, 0).arcs() {
        let _ = arcs.entry(u).or_insert_with(BTreeSet::new).insert(v);
    }

    let erdos_renyi = AdjacencyMapBTreeSet { arcs };

    bencher.bench(|| {
        let _ = complement_adjacency_map_btree_set_clone_difference_collect(
            &erdos_renyi,
        );
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_btree_set_difference_collect(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut arcs = BTreeMap::new();

    for (u, v) in AdjacencyMap::erdos_renyi(order, 0.5, 0).arcs() {
        let _ = arcs.entry(u).or_insert_with(BTreeSet::new).insert(v);
    }

    let erdos_renyi = AdjacencyMapBTreeSet { arcs };

    bencher.bench(|| {
        let _ = complement_adjacency_map_btree_set_difference_collect(
            &erdos_renyi,
        );
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_btree_set_filter_collect(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut arcs = BTreeMap::new();

    for (u, v) in AdjacencyMap::erdos_renyi(order, 0.5, 0).arcs() {
        let _ = arcs.entry(u).or_insert_with(BTreeSet::new).insert(v);
    }

    let erdos_renyi = AdjacencyMapBTreeSet { arcs };

    bencher.bench(|| {
        let _ =
            complement_adjacency_map_btree_set_filter_collect(&erdos_renyi);
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
