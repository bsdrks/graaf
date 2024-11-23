//! Benchmarks of different implementations of `Indegree::is_source`.
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
        HasArc,
        Indegree,
        Vertices,
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
struct AdjacencyListBTreeSet {
    arcs: Vec<BTreeSet<usize>>,
}

#[derive(Debug)]
struct AdjacencyListWeightedBTreeMap {
    arcs: Vec<BTreeMap<usize, usize>>,
}

#[derive(Debug)]
struct AdjacencyMapBTreeSet {
    arcs: Vec<BTreeSet<usize>>,
}

#[derive(Debug)]
struct EdgeListBTreeSet {
    arcs: BTreeSet<(usize, usize)>,
}

fn is_source_indegree<D>(digraph: &D, v: usize) -> bool
where
    D: Indegree,
{
    digraph.indegree(v) == 0
}

fn is_source_adjacency_all_contains(
    digraph: &AdjacencyListBTreeSet,
    v: usize,
) -> bool {
    digraph.arcs.iter().all(|set| !set.contains(&v))
}

fn is_source_adjacency_list_weighted_all_contains(
    digraph: &AdjacencyListWeightedBTreeMap,
    v: usize,
) -> bool {
    digraph.arcs.iter().all(|map| !map.contains_key(&v))
}

fn is_source_adjacency_map_all_contains(
    digraph: &AdjacencyMapBTreeSet,
    v: usize,
) -> bool {
    digraph.arcs.iter().all(|set| !set.contains(&v))
}

fn is_source_adjacency_matrix_all_has_arc(
    digraph: &AdjacencyMatrix,
    v: usize,
) -> bool {
    digraph.vertices().all(|u| !digraph.has_arc(u, v))
}

fn is_source_edge_list_all_ne(digraph: &EdgeListBTreeSet, v: usize) -> bool {
    digraph.arcs.iter().all(|(_, y)| *y != v)
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = digraph.is_source(u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_indegree(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_source_indegree(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_all_contains(bencher: Bencher<'_, '_>, order: usize) {
    let mut digraph = AdjacencyListBTreeSet {
        arcs: vec![BTreeSet::new(); order],
    };

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.05, 0).arcs() {
        let _ = digraph.arcs[u].insert(v);
    }

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_source_adjacency_all_contains(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted(bencher: Bencher<'_, '_>, order: usize) {
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.05, 0).arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        for u in 0..order {
            let _ = digraph.is_source(u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_indegree(bencher: Bencher<'_, '_>, order: usize) {
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.05, 0).arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_source_indegree(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_all_contains(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyListWeightedBTreeMap {
        arcs: vec![BTreeMap::new(); order],
    };

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.05, 0).arcs() {
        let _ = digraph.arcs[u].insert(v, 1);
    }

    bencher.bench(|| {
        for u in 0..order {
            let _ =
                is_source_adjacency_list_weighted_all_contains(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = digraph.is_source(u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_indegree(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_source_indegree(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_all_contains(bencher: Bencher<'_, '_>, order: usize) {
    let mut digraph = AdjacencyMapBTreeSet {
        arcs: vec![BTreeSet::new(); order],
    };

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.05, 0).arcs() {
        let _ = digraph.arcs[u].insert(v);
    }

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_source_adjacency_map_all_contains(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = digraph.is_source(u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_indegree(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_source_indegree(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_all_has_arc(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_source_adjacency_matrix_all_has_arc(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = digraph.is_source(u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_indegree(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_source_indegree(&digraph, u);
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
            let _ = is_source_edge_list_all_ne(&digraph, u);
        }
    });
}
