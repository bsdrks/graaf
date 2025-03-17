//! Benchmarks of different implementations of `IsTournament::is_tournament`.
use {
    divan::Bencher,
    graaf::{
        AddArcWeighted, AdjacencyList, AdjacencyListWeighted, AdjacencyMap,
        AdjacencyMatrix, Arcs, Degree, EdgeList, Empty, ErdosRenyi, HasArc,
        IsTournament, Order, RandomTournament, Size,
    },
    std::{
        collections::{BTreeMap, BTreeSet},
        mem::MaybeUninit,
        sync::Once,
    },
};

fn main() {
    divan::main();
}

#[allow(dead_code)]
#[derive(Debug)]
struct AdjacencyMapBTreeSet {
    pub arcs: BTreeMap<usize, BTreeSet<usize>>,
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

#[allow(static_mut_refs)]
fn empty_set() -> &'static BTreeSet<usize> {
    static mut EMPTY: MaybeUninit<BTreeSet<usize>> = MaybeUninit::uninit();
    static INIT: Once = Once::new();

    unsafe {
        INIT.call_once(|| {
            let _ = EMPTY.write(BTreeSet::new());
        });

        EMPTY.assume_init_ref()
    }
}

fn is_tournament_adjacency_map_btree_set_vec(
    digraph: &AdjacencyMapBTreeSet,
) -> bool {
    let order = digraph.arcs.len();
    let size = digraph.arcs.values().map(BTreeSet::len).sum::<usize>();

    if size != order * (order - 1) / 2 {
        return false;
    }

    let mut out_neighbors = Vec::<&BTreeSet<_>>::with_capacity(order);

    for u in 0..order {
        out_neighbors
            .push(digraph.arcs.get(&u).unwrap_or_else(|| empty_set()));
    }

    let ptr = out_neighbors.as_ptr();

    unsafe {
        for u in 0..order {
            let u_set = &*ptr.add(u);

            for v in (u + 1)..order {
                if u_set.contains(&v) == (*ptr.add(v)).contains(&u) {
                    return false;
                }
            }
        }
    }

    true
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = digraph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_random_tournament_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_random_tournament_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_random_tournament_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_random_tournament(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::random_tournament(order, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        let _ = digraph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_random_tournament_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::random_tournament(order, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_random_tournament_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::random_tournament(order, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_random_tournament_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::random_tournament(order, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = digraph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_random_tournament_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_random_tournament_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_random_tournament_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_btree_set_random_tournament_vec(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::random_tournament(order, 0);

    let digraph = {
        let mut arcs = BTreeMap::new();

        for (u, v) in digraph.arcs() {
            let _ = arcs.entry(u).or_insert_with(BTreeSet::new).insert(v);
        }

        AdjacencyMapBTreeSet { arcs }
    };

    bencher.bench(|| {
        let _ = is_tournament_adjacency_map_btree_set_vec(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = digraph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_random_tournament_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_random_tournament_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_random_tournament_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = digraph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_random_tournament_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_random_tournament_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_random_tournament_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_erdos_renyi(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        let _ = digraph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_erdos_renyi_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_erdos_renyi_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_erdos_renyi_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_btree_set_erdos_renyi_vec(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    let digraph = {
        let mut arcs = BTreeMap::new();

        for (u, v) in digraph.arcs() {
            let _ = arcs.entry(u).or_insert_with(BTreeSet::new).insert(v);
        }

        AdjacencyMapBTreeSet { arcs }
    };

    bencher.bench(|| {
        let _ = is_tournament_adjacency_map_btree_set_vec(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.is_tournament();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_has_arc_order(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_has_arc_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_has_arc_order_size(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_degree_order_size(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_tournament_degree_order_size(&digraph);
    });
}
