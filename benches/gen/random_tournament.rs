//! Benchmarks of different implementations of
//! `RandomTournament::random_tournament`.
use {
    graaf::{
        gen::prng::Xoshiro256StarStar, AddArc, AdjacencyList, AdjacencyMap,
        AdjacencyMatrix, EdgeList, Empty, RandomTournament,
    },
    std::{
        collections::{BTreeMap, BTreeSet, HashSet},
        num::NonZero,
        sync::{Arc, Mutex},
        thread::{available_parallelism, scope, spawn},
    },
};

fn main() {
    divan::main();
}

#[derive(Debug)]
#[allow(dead_code)]
struct AdjacencyListBTreeSet {
    pub arcs: Vec<BTreeSet<usize>>,
}

#[derive(Debug)]
struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct AdjacencyMapBTreeSet {
    pub arcs: BTreeMap<usize, BTreeSet<usize>>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct EdgeListBTreeSet {
    pub arcs: BTreeSet<(usize, usize)>,
    pub order: usize,
}

/// # Panics
///
/// Panics if `order` is zero.
fn random_tournament_adjacency_list_add_arc_empty(
    order: usize,
    seed: u64,
) -> AdjacencyList {
    let mut digraph = AdjacencyList::empty(order);
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (u + 1)..order {
            if rng.next_bool() {
                digraph.add_arc(u, v);
            } else {
                digraph.add_arc(v, u);
            }
        }
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn random_tournament_adjacency_list_btree_set_insert(
    order: usize,
    seed: u64,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = vec![BTreeSet::new(); order];
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (u + 1)..order {
            if rng.next_bool() {
                let _ = unsafe { arcs.get_unchecked_mut(u).insert(v) };
            } else {
                let _ = unsafe { arcs.get_unchecked_mut(v).insert(u) };
            }
        }
    }

    AdjacencyListBTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
#[allow(clippy::uninit_vec)]
fn random_tournament_adjacency_list_btree_set_parallel(
    order: usize,
    seed: u64,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyListBTreeSet {
            arcs: vec![BTreeSet::new()],
        };
    }

    let total_pairs = order * (order - 1) / 2;
    let mut pairs: Vec<(usize, usize)> = Vec::with_capacity(total_pairs);

    unsafe {
        pairs.set_len(total_pairs);
    }

    let num_threads = available_parallelism().map(NonZero::get).unwrap_or(1);

    let chunk_size = order.div_ceil(num_threads);
    let mut offsets = vec![0_usize; num_threads + 1];

    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = (start + chunk_size).min(order);
        let mut count = 0;

        for u in start..end {
            count += order - u - 1; // for vertex u, there are (order-u-1) pairs.
        }

        unsafe {
            *offsets.get_unchecked_mut(i + 1) =
                offsets.get_unchecked(i) + count;
        }
    }

    let pairs_ptr_usize = pairs.as_mut_ptr() as usize;

    scope(|s| {
        for i in 0..num_threads {
            let start_vertex = i * chunk_size;
            let end_vertex = (start_vertex + chunk_size).min(order);
            let start_offset = unsafe { *offsets.get_unchecked(i) };
            let thread_seed = seed.wrapping_add(i as u64);

            let _ = s.spawn(move || {
                let mut rng = Xoshiro256StarStar::new(thread_seed);
                let mut offset = start_offset;

                for u in start_vertex..end_vertex {
                    for v in (u + 1)..order {
                        let arc =
                            if rng.next_bool() { (u, v) } else { (v, u) };

                        unsafe {
                            let local_ptr =
                                pairs_ptr_usize as *mut (usize, usize);

                            *local_ptr.add(offset) = arc;
                        }

                        offset += 1;
                    }
                }
            });
        }
    });

    let mut arcs = vec![BTreeSet::new(); order];

    for (u, v) in pairs {
        let _ = unsafe { arcs.get_unchecked_mut(u).insert(v) };
    }

    AdjacencyListBTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn random_tournament_adjacency_list_hash_set_insert(
    order: usize,
    seed: u64,
) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut digraph = AdjacencyListHashSet {
        arcs: vec![HashSet::new(); order],
    };

    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (u + 1)..order {
            if rng.next_bool() {
                let _ = digraph.arcs[u].insert(v);
            } else {
                let _ = digraph.arcs[v].insert(u);
            }
        }
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn random_tournament_adjacency_map_add_arc_empty(
    order: usize,
    seed: u64,
) -> AdjacencyMap {
    let mut digraph = AdjacencyMap::empty(order);
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (u + 1)..order {
            if rng.next_bool() {
                digraph.add_arc(u, v);
            } else {
                digraph.add_arc(v, u);
            }
        }
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn random_tournament_adjacency_map_btree_set_parallel(
    order: usize,
    seed: u64,
) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyMapBTreeSet {
            arcs: BTreeMap::new(),
        };
    }

    let shared_arcs: Arc<Vec<_>> =
        Arc::new((0..order).map(|_| Mutex::new(BTreeSet::new())).collect());

    let num_threads =
        order.min(available_parallelism().map_or(1, NonZero::get));

    let chunk_size = order.div_ceil(num_threads);
    let mut handles = Vec::with_capacity(num_threads);

    for thread_id in 0..num_threads {
        let shared_arcs = Arc::clone(&shared_arcs);
        let start = thread_id * chunk_size;
        let end = (start + chunk_size).min(order);
        let thread_seed = seed.wrapping_add(thread_id as u64);

        let handle = spawn(move || {
            let mut rng = Xoshiro256StarStar::new(thread_seed);

            for u in start..end {
                for v in (u + 1)..order {
                    unsafe {
                        if rng.next_bool() {
                            let _ = shared_arcs
                                .get_unchecked(u)
                                .lock()
                                .unwrap_unchecked()
                                .insert(v);
                        } else {
                            let _ = shared_arcs
                                .get_unchecked(v)
                                .lock()
                                .unwrap_unchecked()
                                .insert(u);
                        }
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        unsafe {
            handle.join().unwrap_unchecked();
        }
    }

    AdjacencyMapBTreeSet {
        arcs: (0..order)
            .map(|u| {
                (u, unsafe {
                    shared_arcs
                        .get_unchecked(u)
                        .lock()
                        .unwrap_unchecked()
                        .clone()
                })
            })
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn random_tournament_adjacency_matrix_add_arc_empty(
    order: usize,
    seed: u64,
) -> AdjacencyMatrix {
    let mut digraph = AdjacencyMatrix::empty(order);
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (u + 1)..order {
            if rng.next_bool() {
                digraph.add_arc(u, v);
            } else {
                digraph.add_arc(v, u);
            }
        }
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn random_tournament_edge_list_add_arc_empty(
    order: usize,
    seed: u64,
) -> EdgeList {
    let mut digraph = EdgeList::empty(order);
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (u + 1)..order {
            if rng.next_bool() {
                digraph.add_arc(u, v);
            } else {
                digraph.add_arc(v, u);
            }
        }
    }

    digraph
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::random_tournament(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_add_arc_empty(n: usize) {
    let _ = random_tournament_adjacency_list_add_arc_empty(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_set_insert(n: usize) {
    let _ = random_tournament_adjacency_list_btree_set_insert(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_set_parallel(n: usize) {
    let _ = random_tournament_adjacency_list_btree_set_parallel(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_hash_set_insert(n: usize) {
    let _ = random_tournament_adjacency_list_hash_set_insert(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::random_tournament(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_add_arc_empty(n: usize) {
    let _ = random_tournament_adjacency_matrix_add_arc_empty(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::random_tournament(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_add_arc_empty(n: usize) {
    let _ = random_tournament_adjacency_map_add_arc_empty(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_btree_set_parallel(n: usize) {
    let _ = random_tournament_adjacency_map_btree_set_parallel(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list(n: usize) {
    let _ = EdgeList::random_tournament(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_add_arc_empty(n: usize) {
    let _ = random_tournament_edge_list_add_arc_empty(n, 0);
}
