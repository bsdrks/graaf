//! Benchmarks of different implementations of `IsComplete::is_complete`.
use {
    divan::Bencher,
    graaf::{
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        Arcs,
        Complete,
        EdgeList,
        ErdosRenyi,
        HasEdge,
        IsComplete,
        Order,
        Size,
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
struct AdjacencyMapBTreeSet {
    pub arcs: BTreeMap<usize, BTreeSet<usize>>,
}

fn is_complete_has_edge_order<D>(digraph: &D) -> bool
where
    D: HasEdge + Order + Size,
{
    let order = digraph.order();

    if digraph.size() != order * (order - 1) {
        return false;
    }

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
    D: HasEdge + Order + Size,
{
    let order = digraph.order();

    if digraph.size() != order * (order - 1) {
        return false;
    }

    (0..order).all(|u| (u + 1..order).all(|v| digraph.has_edge(u, v)))
}

fn is_complete_iter_2<D>(digraph: &D) -> bool
where
    D: HasEdge + Order + Size,
{
    let order = digraph.order();

    if digraph.size() != order * (order - 1) {
        return false;
    }

    (0..order).all(|u| ((u + 1)..order).all(|v| digraph.has_edge(u, v)))
}

fn is_complete_iter_3<D>(digraph: &D) -> bool
where
    D: HasEdge + Order + Size,
{
    let order = digraph.order();

    if digraph.size() != order * (order - 1) {
        return false;
    }

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
    D: Complete + Order + Size + PartialEq,
{
    let order = digraph.order();

    digraph.size() == order * (order - 1) && digraph == &D::complete(order)
}

fn is_complete_adjacency_map_btree_set_simple(
    digraph: &AdjacencyMapBTreeSet,
) -> bool {
    let expected_neighbors = digraph.arcs.len() - 1;

    for neighbors in digraph.arcs.values() {
        if neighbors.len() != expected_neighbors {
            return false;
        }
    }

    true
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.is_complete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_iter_1(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_iter_1(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_iter_2(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_iter_2(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_iter_3(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_iter_3(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_has_edge_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_has_edge_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_eq_complete(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_eq_complete(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.is_complete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_has_edge_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_has_edge_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_iter_1(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_iter_1(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_iter_2(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_iter_2(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_iter_3(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_iter_3(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_eq_complete(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_eq_complete(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_simple(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);
    let mut arcs = BTreeMap::new();

    // Create a clone in the custom type
    for (u, v) in digraph.arcs() {
        let _ = arcs.entry(u).or_insert_with(BTreeSet::new).insert(v);
    }

    let digraph = AdjacencyMapBTreeSet { arcs };

    bencher.bench(|| {
        let _ = is_complete_adjacency_map_btree_set_simple(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.is_complete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_has_edge_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_has_edge_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_iter_1(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_iter_1(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_iter_2(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_iter_2(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_iter_3(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_iter_3(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_eq_complete(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_eq_complete(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.is_complete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_has_edge_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_has_edge_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_iter_1(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_iter_1(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_iter_2(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_iter_2(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_iter_3(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_iter_3(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_eq_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_complete_eq_complete(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_complete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete_iter_1(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_1(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete_iter_2(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_2(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete_iter_3(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_3(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete_has_edge_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_has_edge_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete_eq_complete(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_eq_complete(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_complete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete_has_edge_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = is_complete_has_edge_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete_iter_1(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_1(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete_iter_2(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_2(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete_iter_3(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_3(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete_eq_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = is_complete_eq_complete(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete_simple(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);
    let mut arcs = BTreeMap::new();

    // Create a clone in the custom type
    for (u, v) in digraph.arcs() {
        let _ = arcs.entry(u).or_insert_with(BTreeSet::new).insert(v);
    }

    let digraph = AdjacencyMapBTreeSet { arcs };

    bencher.bench(|| {
        let _ = is_complete_adjacency_map_btree_set_simple(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_complete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete_has_edge_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_complete_has_edge_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete_iter_1(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_1(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete_iter_2(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_2(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete_iter_3(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_3(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete_eq_complete(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_complete_eq_complete(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_complete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete_has_edge_order(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_has_edge_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete_iter_1(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_1(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete_iter_2(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_2(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete_iter_3(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_iter_3(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete_eq_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_complete_eq_complete(&digraph);
    });
}
