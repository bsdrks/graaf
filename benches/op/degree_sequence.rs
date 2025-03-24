//! Benchmarks of different implementations of
//! `DegreeSequence::degree_sequence`.
use {
    divan::Bencher,
    graaf::{
        AdjacencyList,
        Arcs,
        Degree,
        DegreeSequence,
        ErdosRenyi,
        Vertices,
    },
    std::{
        collections::BTreeSet,
        num::NonZeroUsize,
        sync::atomic::{
            AtomicUsize,
            Ordering,
        },
        thread::{
            available_parallelism,
            scope,
        },
    },
};

fn main() {
    divan::main();
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct AdjacencyListBTreeSet {
    pub arcs: Vec<BTreeSet<usize>>,
}

fn degree_sequence_adjacency_list_map_degree(
    digraph: &AdjacencyList,
) -> impl Iterator<Item = usize> + '_ {
    digraph.vertices().map(move |u| digraph.degree(u))
}

fn degree_sequence_adjacency_list_btree_set_unsafe(
    digraph: &AdjacencyListBTreeSet,
) -> impl Iterator<Item = usize> {
    let order = digraph.arcs.len();
    let mut indegrees = vec![0; order];

    for out_neighbors in &digraph.arcs {
        for &v in out_neighbors {
            unsafe {
                *indegrees.get_unchecked_mut(v) += 1;
            }
        }
    }

    let arcs_ptr = digraph.arcs.as_ptr();

    (0..order).map(move |u| unsafe {
        *indegrees.get_unchecked(u) + (*arcs_ptr.add(u)).len()
    })
}

fn degree_sequence_adjacency_list_btree_set_unsafe_parallel(
    digraph: &AdjacencyListBTreeSet,
) -> impl Iterator<Item = usize> + '_ {
    let order = digraph.arcs.len();

    let indegrees =
        (0..order).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>();

    let num_threads = available_parallelism().map_or(1, NonZeroUsize::get);
    let chunk_size = order.div_ceil(num_threads);

    scope(|s| {
        for chunk in digraph.arcs.chunks(chunk_size) {
            let indegrees_ref = &indegrees;

            let _ = s.spawn(move || {
                for out_neighbors in chunk {
                    for &v in out_neighbors {
                        unsafe {
                            let _ = indegrees_ref
                                .get_unchecked(v)
                                .fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
            });
        }
    });

    (0..order).map(move |u| unsafe {
        indegrees.get_unchecked(u).load(Ordering::Relaxed)
            + digraph.arcs.get_unchecked(u).len()
    })
}

#[divan::bench(args = [10, 100, 1000, 10000, 20000])]
fn adjacency_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.degree_sequence().count();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_map_degree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = degree_sequence_adjacency_list_map_degree(&digraph).count();
    });
}

#[divan::bench(args = [10, 100, 1000, 10000, 20000])]
fn adjacency_list_erdos_renyi_unsafe(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    let digraph = AdjacencyListBTreeSet {
        arcs: digraph.arcs().fold(
            vec![BTreeSet::new(); order],
            |mut arcs, (u, v)| {
                let _ = arcs[u].insert(v);

                arcs
            },
        ),
    };

    bencher.bench(|| {
        let _ =
            degree_sequence_adjacency_list_btree_set_unsafe(&digraph).count();
    });
}

#[divan::bench(args = [10, 100, 1000, 10000, 20000])]
fn adjacency_list_erdos_renyi_unsafe_parallel(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    let digraph = AdjacencyListBTreeSet {
        arcs: digraph.arcs().fold(
            vec![BTreeSet::new(); order],
            |mut arcs, (u, v)| {
                let _ = arcs[u].insert(v);

                arcs
            },
        ),
    };

    bencher.bench(|| {
        let _ =
            degree_sequence_adjacency_list_btree_set_unsafe_parallel(&digraph)
                .count();
    });
}
