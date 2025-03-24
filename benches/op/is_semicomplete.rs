//! Benchmarks of different implementations of
//! `IsSemicomplete::is_semicomplete`.
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
        HasArc,
        IsSemicomplete,
        Order,
        RandomTournament,
        RemoveArc,
        Size,
    },
    std::{
        collections::{
            BTreeMap,
            BTreeSet,
        },
        mem::MaybeUninit,
        sync::Once,
    },
};

fn main() {
    divan::main();
}

#[allow(dead_code)]
#[derive(Debug)]
struct AdjacencyListAlt {
    arcs: Vec<BTreeSet<usize>>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct AdjacencyMapAlt {
    pub arcs: BTreeMap<usize, BTreeSet<usize>>,
}

fn is_semicomplete_adjacency_list_all_all(digraph: &AdjacencyListAlt) -> bool {
    let order = digraph.arcs.len();
    let size = digraph.arcs.iter().map(BTreeSet::len).sum::<usize>();

    if size < order * (order - 1) / 2 {
        return false;
    }

    (0..order).all(|u| {
        (u + 1..order).all(|v| {
            digraph.arcs.get(u).is_some_and(|set| set.contains(&v))
                || digraph.arcs.get(v).is_some_and(|set| set.contains(&u))
        })
    })
}

fn is_semicomplete_adjacency_list_unsafe(digraph: &AdjacencyListAlt) -> bool {
    let order = digraph.arcs.len();

    if order <= 1 {
        return true;
    }

    let arcs_ptr = digraph.arcs.as_ptr();
    let mut total = 0;

    for u in 0..order {
        unsafe {
            total += (*arcs_ptr.add(u)).len();
        }
    }

    if total < order * (order - 1) / 2 {
        return false;
    }

    for u in 0..order {
        for v in (u + 1)..order {
            unsafe {
                let set_u = &*arcs_ptr.add(u);
                let set_v = &*arcs_ptr.add(v);

                if !set_u.contains(&v) && !set_v.contains(&u) {
                    return false;
                }
            }
        }
    }

    true
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

fn is_semicomplete_adjacency_map_unsafe(digraph: &AdjacencyMapAlt) -> bool {
    let order = digraph.arcs.len();
    let size = digraph.arcs.values().map(BTreeSet::len).sum::<usize>();

    if size < order * (order - 1) / 2 {
        return false;
    }

    let mut out_neighbors = Vec::<&BTreeSet<usize>>::with_capacity(order);

    for u in 0..order {
        out_neighbors
            .push(digraph.arcs.get(&u).unwrap_or_else(|| empty_set()));
    }

    for u in 0..order {
        for v in (u + 1)..order {
            if unsafe { !out_neighbors.get_unchecked(u).contains(&v) }
                && unsafe { !out_neighbors.get_unchecked(v).contains(&u) }
            {
                return false;
            }
        }
    }

    true
}

fn is_semicomplete_adjacency_map_all_all(digraph: &AdjacencyMapAlt) -> bool {
    let order = digraph.arcs.len();
    let size = digraph.arcs.values().map(BTreeSet::len).sum::<usize>();

    if size < order * (order - 1) / 2 {
        return false;
    }

    (0..order).all(|u| {
        (u + 1..order).all(|v| {
            digraph.arcs.get(&u).is_some_and(|set| set.contains(&v))
                || digraph.arcs.get(&v).is_some_and(|set| set.contains(&u))
        })
    })
}

fn is_semicomplete_adjacency_matrix_all_all(
    digraph: &AdjacencyMatrix,
) -> bool {
    let order = digraph.order();
    let size = digraph.size();

    if size < order * (order - 1) / 2 {
        return false;
    }

    (0..order).all(|u| {
        (u + 1..order).all(|v| digraph.has_arc(u, v) ^ digraph.has_arc(v, u))
    })
}

fn is_semicomplete_edge_list_all_all(digraph: &EdgeList) -> bool {
    let order = digraph.order();
    let size = digraph.size();

    if size < order * (order - 1) / 2 {
        return false;
    }

    (0..order).all(|u| {
        (u + 1..order).all(|v| digraph.has_arc(u, v) || digraph.has_arc(v, u))
    })
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_almost_random_tournament(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyList::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_almost_random_tournament_all_all(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyList::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    let mut arcs = vec![BTreeSet::new(); order];

    for (u, v) in digraph.arcs() {
        let _ = arcs[u].insert(v);
    }

    let digraph = AdjacencyListAlt { arcs };

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_list_all_all(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_almost_random_tournament_unsafe(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyList::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    let digraph = {
        let mut arcs = vec![BTreeSet::new(); order];

        for (u, v) in digraph.arcs() {
            let _ = arcs[u].insert(v);
        }

        AdjacencyListAlt { arcs }
    };

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_list_unsafe(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_almost_random_tournament(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyMap::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_almost_random_tournament_all_all(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyMap::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);
    let mut arcs = BTreeMap::<usize, BTreeSet<usize>>::new();

    for (u, v) in digraph.arcs() {
        let _ = arcs.entry(u).or_default().insert(v);
    }

    let digraph = AdjacencyMapAlt { arcs };

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_map_all_all(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_almost_random_tournament_unsafe(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyMap::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);
    let mut arcs: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();

    for (u, v) in digraph.arcs() {
        let _ = arcs.entry(u).or_default().insert(v);
    }

    let digraph = AdjacencyMapAlt { arcs };

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_map_unsafe(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_almost_random_tournament(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyMatrix::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_almost_random_tournament_all_all(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyMatrix::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_matrix_all_all(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_almost_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let mut digraph = EdgeList::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_almost_random_tournament_all_all(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = EdgeList::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = is_semicomplete_edge_list_all_all(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_random_tournament_all_all(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::random_tournament(order, 0);
    let mut arcs = vec![BTreeSet::new(); order];

    for (u, v) in digraph.arcs() {
        let _ = arcs[u].insert(v);
    }

    let digraph = AdjacencyListAlt { arcs };

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_list_all_all(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_random_tournament_unsafe(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::random_tournament(order, 0);

    let digraph = {
        let mut arcs = vec![BTreeSet::new(); order];

        for (u, v) in digraph.arcs() {
            let _ = arcs[u].insert(v);
        }

        AdjacencyListAlt { arcs }
    };

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_list_unsafe(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_random_tournament_all_all(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::random_tournament(order, 0);
    let mut arcs = BTreeMap::<usize, BTreeSet<usize>>::new();

    for (u, v) in digraph.arcs() {
        let _ = arcs.entry(u).or_default().insert(v);
    }

    let digraph = AdjacencyMapAlt { arcs };

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_map_all_all(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_random_tournament_unsafe(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::random_tournament(order, 0);
    let mut arcs = BTreeMap::<usize, BTreeSet<usize>>::new();

    for (u, v) in digraph.arcs() {
        let _ = arcs.entry(u).or_default().insert(v);
    }

    let digraph = AdjacencyMapAlt { arcs };

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_map_unsafe(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_random_tournament_all_all(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_matrix_all_all(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_random_tournament_all_all(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_edge_list_all_all(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_all_all(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut arcs = vec![BTreeSet::new(); order];

    for (u, v) in digraph.arcs() {
        let _ = arcs[u].insert(v);
    }

    let digraph = AdjacencyListAlt { arcs };

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_list_all_all(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_unsafe(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    let digraph = {
        let mut arcs = vec![BTreeSet::new(); order];

        for (u, v) in digraph.arcs() {
            let _ = arcs[u].insert(v);
        }

        AdjacencyListAlt { arcs }
    };

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_list_unsafe(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_all_all(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);
    let mut arcs = BTreeMap::<usize, BTreeSet<usize>>::new();

    for (u, v) in digraph.arcs() {
        let _ = arcs.entry(u).or_default().insert(v);
    }

    let digraph = AdjacencyMapAlt { arcs };

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_map_all_all(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_unsafe(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);
    let mut arcs = BTreeMap::<usize, BTreeSet<usize>>::new();

    for (u, v) in digraph.arcs() {
        let _ = arcs.entry(u).or_default().insert(v);
    }

    let digraph = AdjacencyMapAlt { arcs };

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_map_unsafe(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_all_all(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_matrix_all_all(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_all_all(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_edge_list_all_all(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete_all_all(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::complete(order);
    let mut arcs = vec![BTreeSet::new(); order];

    for (u, v) in digraph.arcs() {
        let _ = arcs[u].insert(v);
    }

    let digraph = AdjacencyListAlt { arcs };

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_list_all_all(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete_unsafe(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::complete(order);

    let digraph = {
        let mut arcs = vec![BTreeSet::new(); order];

        for (u, v) in digraph.arcs() {
            let _ = arcs[u].insert(v);
        }

        AdjacencyListAlt { arcs }
    };

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_list_unsafe(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete_all_all(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);
    let mut arcs = BTreeMap::<usize, BTreeSet<usize>>::new();

    for (u, v) in digraph.arcs() {
        let _ = arcs.entry(u).or_default().insert(v);
    }

    let digraph = AdjacencyMapAlt { arcs };

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_map_all_all(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete_unsafe(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);
    let mut arcs = BTreeMap::<usize, BTreeSet<usize>>::new();

    for (u, v) in digraph.arcs() {
        let _ = arcs.entry(u).or_default().insert(v);
    }

    let digraph = AdjacencyMapAlt { arcs };

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_map_unsafe(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete_all_all(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_semicomplete_adjacency_matrix_all_all(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete_all_all(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_semicomplete_edge_list_all_all(&digraph);
    });
}
