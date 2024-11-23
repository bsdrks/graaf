//! Benchmarks of different implementations of `Outdegree::is_sink`.
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
        Outdegree,
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

fn is_sink_adjacency_matrix_all_has_arc(
    digraph: &AdjacencyMatrix,
    u: usize,
) -> bool {
    digraph.vertices().all(|v| !digraph.has_arc(u, v))
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

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_matrix_all_has_arc(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.05, 0);

    bencher.bench(|| {
        for u in 0..order {
            let _ = is_sink_adjacency_matrix_all_has_arc(&digraph, u);
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
