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

pub struct AdjacencyListBTreeSet {
    pub arcs: Vec<BTreeSet<usize>>,
}

pub struct AdjacencyListWeightedBTreeMap<W> {
    pub arcs: Vec<BTreeMap<usize, W>>,
}

pub struct AdjacencyMapBTreeSet {
    pub arcs: BTreeMap<usize, BTreeSet<usize>>,
}

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

fn converse_adjacency_list_weighted_add_arc_empty_has_arc_order(
    digraph: &AdjacencyListWeightedBTreeMap<usize>,
) -> AdjacencyListWeightedBTreeMap<usize> {
    let order = digraph.arcs.len();

    let mut converse = AdjacencyListWeightedBTreeMap {
        arcs: vec![BTreeMap::new(); order],
    };

    for (u, v) in digraph.arcs.iter().enumerate() {
        for (v, w) in v {
            converse.arcs[*v].insert(u, *w);
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
                arcs[*v].insert(u, *w);
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
        arcs[v].insert(u);
    }

    let erdos_renyi = AdjacencyListBTreeSet { arcs };

    bencher.bench(|| {
        let _ = converse_adjacency_list_btree_set_for_for(&erdos_renyi);
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
fn adjacency_list_weighted_btree_map_add_arc_empty_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let erdos_renyi = AdjacencyList::erdos_renyi(order, 0.5, 0);

    let erdos_renyi = AdjacencyListWeightedBTreeMap::<usize> {
        arcs: erdos_renyi.arcs().fold(
            vec![BTreeMap::new(); order],
            |mut arcs, (u, v)| {
                arcs[v].insert(u, 0);

                arcs
            },
        ),
    };

    bencher.bench(|| {
        let _ = converse_adjacency_list_weighted_add_arc_empty_has_arc_order(
            &erdos_renyi,
        );
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
                arcs[v].insert(u, 0);

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
        arcs.entry(v).or_default().insert(u);
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
        arcs.insert((v, u));
    }

    let erdos_renyi = EdgeListBTreeSet { arcs, order };

    bencher.bench(|| {
        let _ = converse_edge_list_btree_set_collect(&erdos_renyi);
    });
}
